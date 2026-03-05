#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Generate a Homebrew cask from a GitHub release tag.

Usage:
  scripts/generate-homebrew-cask.sh --tag v0.1.0 [--repo owner/name] [--output path]

Options:
  --tag      Release tag (required), e.g. v0.1.0
  --repo     GitHub repo in owner/name format (default: detected from git remote)
  --output   Output cask file path (default: dist/homebrew/<repo-name>.rb)
  -h, --help Show this help

Environment:
  GITHUB_TOKEN  Optional. Used for authenticated GitHub API calls.
USAGE
}

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required command not found: $1" >&2
    exit 1
  fi
}

infer_repo_from_git() {
  local remote
  remote="$(git config --get remote.origin.url 2>/dev/null || true)"

  if [[ -z "$remote" ]]; then
    echo "error: --repo not provided and no git remote.origin.url found" >&2
    exit 1
  fi

  if [[ "$remote" =~ github.com[:/]([^/]+)/([^/.]+)(\.git)?$ ]]; then
    echo "${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
    return
  fi

  echo "error: unable to parse GitHub repo from remote: $remote" >&2
  exit 1
}

curl_api() {
  local url="$1"
  if [[ -n "${GITHUB_TOKEN:-}" ]]; then
    curl -fsSL \
      -H "Authorization: Bearer ${GITHUB_TOKEN}" \
      -H "Accept: application/vnd.github+json" \
      "$url"
  else
    curl -fsSL \
      -H "Accept: application/vnd.github+json" \
      "$url"
  fi
}

sha256_from_url() {
  local url="$1"
  local tmp_file
  tmp_file="$(mktemp)"
  trap 'rm -f "$tmp_file"' RETURN

  if [[ -n "${GITHUB_TOKEN:-}" ]]; then
    curl -fL \
      -H "Authorization: Bearer ${GITHUB_TOKEN}" \
      "$url" \
      -o "$tmp_file"
  else
    curl -fL "$url" -o "$tmp_file"
  fi

  shasum -a 256 "$tmp_file" | awk '{print $1}'
}

TAG=""
REPO=""
OUTPUT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --tag)
      TAG="$2"
      shift 2
      ;;
    --repo)
      REPO="$2"
      shift 2
      ;;
    --output)
      OUTPUT="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
done

if [[ -z "$TAG" ]]; then
  echo "error: --tag is required" >&2
  usage
  exit 1
fi

require_cmd curl
require_cmd jq
require_cmd shasum

if [[ -z "$REPO" ]]; then
  REPO="$(infer_repo_from_git)"
fi

REPO_NAME="${REPO#*/}"
CASK_TOKEN="$(printf '%s' "${REPO_NAME}" | tr '[:upper:]' '[:lower:]')"
VERSION="${TAG#v}"

if [[ -z "$OUTPUT" ]]; then
  OUTPUT="dist/homebrew/${CASK_TOKEN}.rb"
fi

RELEASE_API="https://api.github.com/repos/${REPO}/releases/tags/${TAG}"
echo "Fetching release metadata for ${REPO}@${TAG} ..."
RELEASE_JSON="$(curl_api "$RELEASE_API")"

APP_NAME="$(jq -r '.productName // empty' src-tauri/tauri.conf.json 2>/dev/null || true)"
if [[ -z "$APP_NAME" ]]; then
  APP_NAME="$REPO_NAME"
fi

MAC_INSTALLER_FILTER='(\\.dmg$|\\.app\\.tar\\.gz$|\\.app\\.zip$)'

UNIVERSAL_URL="$(jq -r "
  .assets[]
  | select(.name | test(\"${MAC_INSTALLER_FILTER}\"; \"i\"))
  | select(.name | test(\"universal|univ|all\"; \"i\"))
  | .browser_download_url
" <<<"$RELEASE_JSON" | head -n1)"

ARM_URL="$(jq -r "
  .assets[]
  | select(.name | test(\"${MAC_INSTALLER_FILTER}\"; \"i\"))
  | select(.name | test(\"aarch64|arm64|armv8|apple\"; \"i\"))
  | .browser_download_url
" <<<"$RELEASE_JSON" | head -n1)"

INTEL_URL="$(jq -r "
  .assets[]
  | select(.name | test(\"${MAC_INSTALLER_FILTER}\"; \"i\"))
  | select(.name | test(\"x86_64|x64|amd64|intel\"; \"i\"))
  | .browser_download_url
" <<<"$RELEASE_JSON" | head -n1)"

# Universal macOS artifact can satisfy both arch blocks.
if [[ -z "$ARM_URL" && -n "$UNIVERSAL_URL" ]]; then
  ARM_URL="$UNIVERSAL_URL"
fi
if [[ -z "$INTEL_URL" && -n "$UNIVERSAL_URL" ]]; then
  INTEL_URL="$UNIVERSAL_URL"
fi

if [[ -z "$ARM_URL" && -z "$INTEL_URL" ]]; then
  echo "error: no usable macOS installer assets found in release ${TAG}" >&2
  echo "expected one of: .dmg, .app.tar.gz, .app.zip with arch tags (arm64/x86_64) or universal" >&2
  exit 1
fi

ARM_SHA=""
INTEL_SHA=""
if [[ -n "$ARM_URL" && ( -z "$INTEL_URL" || "$ARM_URL" != "$INTEL_URL" ) ]]; then
  echo "Downloading arm64 macOS installer to compute sha256 ..."
  ARM_SHA="$(sha256_from_url "$ARM_URL")"
fi

if [[ -n "$INTEL_URL" ]]; then
  echo "Downloading x86_64 macOS installer to compute sha256 ..."
  INTEL_SHA="$(sha256_from_url "$INTEL_URL")"
fi

if [[ -n "$ARM_URL" && -n "$INTEL_URL" && "$ARM_URL" == "$INTEL_URL" ]]; then
  ARM_SHA="$INTEL_SHA"
fi

mkdir -p "$(dirname "$OUTPUT")"

cat > "$OUTPUT" <<CASK
cask "${CASK_TOKEN}" do
  version "${VERSION}"
CASK

if [[ -n "$ARM_URL" ]]; then
  cat >> "$OUTPUT" <<CASK

  on_arm do
    sha256 "${ARM_SHA}"
    url "${ARM_URL}"
  end
CASK
fi

if [[ -n "$INTEL_URL" ]]; then
  cat >> "$OUTPUT" <<CASK

  on_intel do
    sha256 "${INTEL_SHA}"
    url "${INTEL_URL}"
  end
CASK
fi

cat >> "$OUTPUT" <<CASK

  name "${APP_NAME}"
  desc "${APP_NAME} desktop app"
  homepage "https://github.com/${REPO}"

  app "${APP_NAME}.app"
end
CASK

echo "Generated cask: ${OUTPUT}"
