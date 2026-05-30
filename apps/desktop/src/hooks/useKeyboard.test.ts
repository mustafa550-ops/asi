import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook } from "@testing-library/react";
import { fireEvent } from "@testing-library/react";
import { useKeyboard } from "./useKeyboard";

describe("useKeyboard", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("fires Escape action on Escape key", () => {
    const actions = { Escape: vi.fn() };
    renderHook(() => useKeyboard(actions));
    fireEvent.keyDown(document, { key: "Escape" });
    expect(actions.Escape).toHaveBeenCalledOnce();
  });

  it("fires Enter action on Enter key when not in input", () => {
    const actions = { Enter: vi.fn() };
    renderHook(() => useKeyboard(actions));
    fireEvent.keyDown(document, { key: "Enter" });
    expect(actions.Enter).toHaveBeenCalledOnce();
  });

  it("does not fire Enter action on Enter key when in input", () => {
    const actions = { Enter: vi.fn() };
    renderHook(() => useKeyboard(actions));
    const input = document.createElement("input");
    document.body.appendChild(input);
    input.focus();
    fireEvent.keyDown(input, { key: "Enter" });
    expect(actions.Enter).not.toHaveBeenCalled();
    document.body.removeChild(input);
  });

  it("fires Ctrl+key combo action", () => {
    const actions = { "Ctrl+k": vi.fn() };
    renderHook(() => useKeyboard(actions));
    fireEvent.keyDown(document, { key: "k", ctrlKey: true });
    expect(actions["Ctrl+k"]).toHaveBeenCalledOnce();
  });

  it("does not fire when enabled is false", () => {
    const actions = { Escape: vi.fn() };
    renderHook(() => useKeyboard(actions, false));
    fireEvent.keyDown(document, { key: "Escape" });
    expect(actions.Escape).not.toHaveBeenCalled();
  });

  it("cleans up event listener on unmount", () => {
    const actions = { Escape: vi.fn() };
    const { unmount } = renderHook(() => useKeyboard(actions));
    fireEvent.keyDown(document, { key: "Escape" });
    expect(actions.Escape).toHaveBeenCalledTimes(1);
    actions.Escape.mockClear();
    unmount();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(actions.Escape).not.toHaveBeenCalled();
  });
});
