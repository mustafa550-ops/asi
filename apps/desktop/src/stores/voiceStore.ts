import { create } from "zustand";
import { invoke } from "../lib/tauri";

export interface VoiceState {
  isListening: boolean;
  isProcessing: boolean;
  transcript: string;
  startListening: () => Promise<void>;
  stopListening: () => Promise<void>;
  synthesizeSpeech: (text: string) => Promise<void>;
  setTranscript: (text: string) => void;
}

export const useVoiceStore = create<VoiceState>((set) => ({
  isListening: false,
  isProcessing: false,
  transcript: "",

  startListening: async () => {
    set({ isListening: true, transcript: "" });
    try {
      await invoke("start_listening");
    } catch (err) {
      console.error("Voice recording error:", err);
      set({ isListening: false });
    }
  },

  stopListening: async () => {
    set({ isListening: false, isProcessing: true });
    try {
      const response: string = await invoke("stop_listening");
      set({ transcript: response, isProcessing: false });
    } catch (err) {
      console.error("Stop listening error:", err);
      set({ isProcessing: false });
    }
  },

  synthesizeSpeech: async (text: string) => {
    if (!text) return;
    try {
      await invoke("synthesize_speech", { text });
    } catch (err) {
      console.error("Speech synthesis error:", err);
    }
  },

  setTranscript: (text: string) => set({ transcript: text }),
}));
