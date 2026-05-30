import { describe, it, expect, vi } from "vitest";
import { renderHook } from "@testing-library/react";
import { useTauriEvent } from "./useTauriEvent";

describe("useTauriEvent", () => {
  it("does not crash outside Tauri environment", () => {
    const cb = vi.fn();
    expect(() => renderHook(() => useTauriEvent("test-event", cb))).not.toThrow();
  });

  it("does not call callback outside Tauri", () => {
    const cb = vi.fn();
    renderHook(() => useTauriEvent("test-event", cb));
    expect(cb).not.toHaveBeenCalled();
  });
});
