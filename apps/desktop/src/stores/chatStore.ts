import { create } from "zustand";
import { invoke } from "../lib/tauri";

export interface Message {
  role: "user" | "adler" | "system";
  content: string;
}

interface ChatState {
  messages: Message[];
  input: string;
  loading: boolean;
  send: () => Promise<void>;
  setInput: (v: string) => void;
  addMessage: (role: Message["role"], content: string) => void;
}

export const useChatStore = create<ChatState>((set, get) => ({
  messages: [
    { role: "adler", content: "ADLER ASI hazır. Nasıl yardımcı olabilirim?" },
  ],
  input: "",
  loading: false,

  send: async () => {
    const text = get().input.trim();
    if (!text || get().loading) return;

    set((s) => ({
      input: "",
      messages: [...s.messages, { role: "user" as const, content: text }],
      loading: true,
    }));

    try {
      const response: string = await invoke("send_command", { command: text });
      set((s) => ({
        messages: [...s.messages, { role: "adler", content: response }],
        loading: false,
      }));
    } catch (err) {
      set((s) => ({
        messages: [...s.messages, { role: "adler", content: `Hata: ${err}` }],
        loading: false,
      }));
    }
  },

  setInput: (v: string) => set({ input: v }),
  addMessage: (role, content) => set((s) => ({ messages: [...s.messages, { role, content }] })),
}));
