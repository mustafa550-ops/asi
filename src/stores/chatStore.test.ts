import { describe, it, expect, beforeEach, vi } from "vitest";
import { useChatStore } from "./chatStore";

vi.mock("../lib/tauri", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "../lib/tauri";

describe("chatStore", () => {
  beforeEach(() => {
    useChatStore.setState({
      messages: [
        { role: "adler", content: "ADLER ASI hazır. Nasıl yardımcı olabilirim?" },
      ],
      input: "",
      loading: false,
    });
    vi.clearAllMocks();
  });

  it("baslangic mesajini icerir", () => {
    const state = useChatStore.getState();
    expect(state.messages).toHaveLength(1);
    expect(state.messages[0].role).toBe("adler");
  });

  it("setInput input degerini gunceller", () => {
    useChatStore.getState().setInput("merhaba");
    expect(useChatStore.getState().input).toBe("merhaba");
  });

  it("send user mesaji ekler ve invoke cagirir", async () => {
    vi.mocked(invoke).mockResolvedValue("Merhaba! Nasıl yardımcı olabilirim?");
    useChatStore.getState().setInput("merhaba");

    await useChatStore.getState().send();

    expect(invoke).toHaveBeenCalledWith("send_command", { command: "merhaba" });
    const messages = useChatStore.getState().messages;
    expect(messages).toHaveLength(3);
    expect(messages[1].role).toBe("user");
    expect(messages[1].content).toBe("merhaba");
    expect(messages[2].role).toBe("adler");
  });

  it("bos input send cagri yapmaz", async () => {
    useChatStore.getState().setInput("");

    await useChatStore.getState().send();

    expect(invoke).not.toHaveBeenCalled();
  });

  it("hata durumunda hata mesaji ekler", async () => {
    vi.mocked(invoke).mockRejectedValue(new Error("API hatası"));
    useChatStore.getState().setInput("test");

    await useChatStore.getState().send();

    const messages = useChatStore.getState().messages;
    expect(messages[messages.length - 1].content).toContain("Hata");
  });
});
