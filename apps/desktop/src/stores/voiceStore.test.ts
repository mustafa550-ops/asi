import { describe, it, expect, beforeEach, vi } from "vitest";
import { useVoiceStore } from "./voiceStore";

const mockInvoke = vi.fn();
vi.mock("../lib/tauri", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

describe("voiceStore", () => {
  beforeEach(() => {
    useVoiceStore.setState({
      isListening: false,
      isProcessing: false,
      transcript: "",
    });
    vi.clearAllMocks();
  });

  it("initial state is idle", () => {
    const s = useVoiceStore.getState();
    expect(s.isListening).toBe(false);
    expect(s.isProcessing).toBe(false);
    expect(s.transcript).toBe("");
  });

  it("startListening sets isListening true and clears transcript", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await useVoiceStore.getState().startListening();
    expect(useVoiceStore.getState().isListening).toBe(true);
    expect(useVoiceStore.getState().transcript).toBe("");
  });

  it("startListening sets isListening false on error", async () => {
    mockInvoke.mockRejectedValue(new Error("microphone error"));
    await useVoiceStore.getState().startListening();
    expect(useVoiceStore.getState().isListening).toBe(false);
  });

  it("stopListening sets isListening false and isProcessing true", async () => {
    useVoiceStore.setState({ isListening: true });
    mockInvoke.mockResolvedValue("merhaba");
    await useVoiceStore.getState().stopListening();
    expect(useVoiceStore.getState().isListening).toBe(false);
    expect(useVoiceStore.getState().isProcessing).toBe(false);
    expect(useVoiceStore.getState().transcript).toBe("merhaba");
  });

  it("stopListening handles error gracefully", async () => {
    useVoiceStore.setState({ isListening: true, isProcessing: false });
    mockInvoke.mockRejectedValue(new Error("stop failed"));
    await useVoiceStore.getState().stopListening();
    expect(useVoiceStore.getState().isProcessing).toBe(false);
  });

  it("synthesizeSpeech calls invoke with text", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await useVoiceStore.getState().synthesizeSpeech("merhaba");
    expect(mockInvoke).toHaveBeenCalledWith("synthesize_speech", {
      text: "merhaba",
    });
  });

  it("synthesizeSpeech does nothing for empty text", async () => {
    await useVoiceStore.getState().synthesizeSpeech("");
    expect(mockInvoke).not.toHaveBeenCalled();
  });

  it("synthesizeSpeech handles error gracefully", async () => {
    mockInvoke.mockRejectedValue(new Error("tts error"));
    await expect(
      useVoiceStore.getState().synthesizeSpeech("test"),
    ).resolves.not.toThrow();
  });

  it("setTranscript updates transcript", () => {
    useVoiceStore.getState().setTranscript("test");
    expect(useVoiceStore.getState().transcript).toBe("test");
  });
});
