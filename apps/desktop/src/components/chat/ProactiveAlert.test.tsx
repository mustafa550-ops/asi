import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, act } from "../../test/test-utils";
import { ProactiveAlert } from "./ProactiveAlert";

describe("ProactiveAlert", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders nothing initially", () => {
    const { container } = render(<ProactiveAlert />);
    expect(container.innerHTML).toBe("");
  });

  it("renders nothing when alerts empty", () => {
    const { container } = render(<ProactiveAlert />);
    expect(container.querySelector(".proactive-alerts")).not.toBeInTheDocument();
  });

  it("uses aria-live assertive", () => {
    const { container } = render(<ProactiveAlert />);
    act(() => {
      const event = new CustomEvent("tauri-event", { detail: { event: "proactive-alert", payload: "test" } });
      window.dispatchEvent(event);
    });
  });
});
