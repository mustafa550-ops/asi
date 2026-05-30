import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, act } from "../../test/test-utils";
import { StatusBar } from "./StatusBar";

const mockInvoke = vi.fn();
vi.mock("../../lib/tauri", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

describe("StatusBar", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders footer with status role", () => {
    mockInvoke.mockResolvedValue("sistem durumu");
    render(<StatusBar />);
    expect(screen.getByRole("status")).toBeInTheDocument();
  });

  it("shows offline ollama initially", () => {
    mockInvoke.mockResolvedValue("sistem durumu");
    render(<StatusBar />);
    expect(screen.getByText(/✗/)).toBeInTheDocument();
  });

  it("updates ollama status when invoke indicates ollama is running", async () => {
    mockInvoke.mockResolvedValue("Ollama bagli. 8 ajan aktif.");
    render(<StatusBar />);
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });
    expect(screen.getByText(/✓/)).toBeInTheDocument();
  });

  it("shows agent count", async () => {
    mockInvoke.mockResolvedValue("8 ajan aktif. Ollama bagli.");
    render(<StatusBar />);
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });
    expect(screen.getByText(/8 ajan/)).toBeInTheDocument();
  });

  it("handles invoke errors gracefully", async () => {
    mockInvoke.mockRejectedValue(new Error("fail"));
    render(<StatusBar />);
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });
    expect(screen.getByRole("status")).toBeInTheDocument();
  });

  it("uses setInterval for polling", () => {
    const setIntervalSpy = vi.spyOn(window, "setInterval");
    mockInvoke.mockResolvedValue("sistem durumu");
    render(<StatusBar />);
    expect(setIntervalSpy).toHaveBeenCalledWith(expect.any(Function), 15000);
    setIntervalSpy.mockRestore();
  });
});
