import { create } from "zustand";
import { invoke } from "../lib/tauri";

export interface SkillStep {
  order: number;
  description: string;
}

export interface Skill {
  id: number;
  name: string;
  description: string;
  triggers: string[];
  approval: string;
  steps: SkillStep[];
  logic_code: string | null;
  evolution: string[];
  run_count: number;
  active: boolean;
  version: number;
  created_at: string;
}

export type Panel = "list" | "detail" | "add";

interface SkillState {
  skills: Skill[];
  loading: boolean;
  error: string;
  panel: Panel;
  selected: Skill | null;
  mdContent: string;
  result: string;
  loadSkills: () => Promise<void>;
  toggleSkill: (name: string) => Promise<void>;
  deleteSkill: (name: string) => Promise<void>;
  runSkill: (name: string) => Promise<void>;
  addSkill: () => Promise<void>;
  setPanel: (p: Panel) => void;
  setSelected: (s: Skill | null) => void;
  setMdContent: (c: string) => void;
  clearResult: () => void;
  clearError: () => void;
}

export const useSkillStore = create<SkillState>((set, get) => ({
  skills: [],
  loading: false,
  error: "",
  panel: "list",
  selected: null,
  mdContent: "",
  result: "",

  loadSkills: async () => {
    set({ loading: true, error: "" });
    try {
      const data = await invoke<Skill[]>("list_skills");
      set({ skills: data, loading: false });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  toggleSkill: async (name: string) => {
    try {
      const msg = await invoke<string>("toggle_skill", { name });
      set({ result: msg });
      get().loadSkills();
    } catch (err) {
      set({ error: String(err) });
    }
  },

  deleteSkill: async (name: string) => {
    try {
      const msg = await invoke<string>("delete_skill", { name });
      set({ result: msg });
      const { selected } = get();
      if (selected?.name === name) {
        set({ selected: null, panel: "list" });
      }
      get().loadSkills();
    } catch (err) {
      set({ error: String(err) });
    }
  },

  runSkill: async (name: string) => {
    try {
      const msg = await invoke<string>("run_skill_by_name", { name });
      set({ result: msg });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  addSkill: async () => {
    const { mdContent } = get();
    if (!mdContent.trim()) return;
    try {
      const msg = await invoke<string>("add_skill_md", { content: mdContent });
      set({ result: msg, mdContent: "", panel: "list" });
      get().loadSkills();
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setPanel: (p: Panel) => set({ panel: p }),
  setSelected: (s: Skill | null) => set({ selected: s }),
  setMdContent: (c: string) => set({ mdContent: c }),
  clearResult: () => set({ result: "" }),
  clearError: () => set({ error: "" }),
}));
