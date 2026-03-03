import { invoke } from "@tauri-apps/api/core";

export interface AppSettings {
  aiEndpoint: string;
  aiApiKey: string;
  aiModel: string;
  diffType: string;
}

const defaults: AppSettings = {
  aiEndpoint: "https://api.openai.com/v1",
  aiApiKey: "",
  aiModel: "gpt-4o",
  diffType: "unstaged",
};

let _settings = $state<AppSettings>({ ...defaults });
let _loaded = $state(false);

export const settingsStore = {
  get settings() {
    return _settings;
  },

  get loaded() {
    return _loaded;
  },

  async load() {
    try {
      const s = await invoke<AppSettings>("load_settings");
      _settings = s;
    } catch {
      _settings = { ...defaults };
    }
    _loaded = true;
  },

  async save(settings: AppSettings) {
    _settings = settings;
    await invoke("save_settings", { settings });
  },
};
