import { invoke } from "@tauri-apps/api/core";

export type Theme = "light" | "dark";

export interface AppSettings {
  aiEndpoint: string;
  aiApiKey: string;
  aiModel: string;
  diffType: string;
  theme: Theme;
  lastOpenedFolder: string;
}

const defaults: AppSettings = {
  aiEndpoint: "https://api.openai.com/v1",
  aiApiKey: "",
  aiModel: "gpt-4o",
  diffType: "unstaged",
  theme: "light",
  lastOpenedFolder: "",
};

let _settings = $state<AppSettings>({ ...defaults });
let _loaded = $state(false);

function applyTheme(theme: Theme) {
  if (theme === "dark") {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
}

export const settingsStore = {
  get settings() {
    return _settings;
  },

  get loaded() {
    return _loaded;
  },

  get theme(): Theme {
    return _settings.theme;
  },

  async load() {
    try {
      const s = await invoke<AppSettings>("load_settings");
      _settings = s;
    } catch {
      _settings = { ...defaults };
    }
    applyTheme(_settings.theme);
    _loaded = true;
  },

  async save(settings: AppSettings) {
    _settings = settings;
    applyTheme(settings.theme);
    await invoke("save_settings", { settings });
  },

  async toggleTheme() {
    const newTheme: Theme = _settings.theme === "dark" ? "light" : "dark";
    _settings = { ..._settings, theme: newTheme };
    applyTheme(newTheme);
    await invoke("save_settings", { settings: _settings });
  },
};
