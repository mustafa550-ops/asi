import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, act } from "../../test/test-utils";
import { StatusBar } from "./StatusBar";

const mockInvoke = vi.fn();
vi.mock("../../lib/tauri", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

function makeMetrics(overrides: Record<string, unknown> = {}) {
  return JSON.stringify({
    cpu: 45,
    memory: 6.2,
    uptime: "3600s",
    active_agents: 8,
    ...overrides,
  });
}

describe("StatusBar", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders footer with status role", () => {
    mockInvoke.mockResolvedValue(makeMetrics());
    render(<StatusBar />);
    expect(screen.getByRole("status")).toBeInTheDocument();
  });

  it("shows offline indicator initially", () => {
    mockInvoke.mockResolvedValue(makeMetrics());
    render(<StatusBar />);
    expect(screen.getByText(/✗/)).toBeInTheDocument();
  });

  it("updates to online when metrics arrive", async () => {
    mockInvoke.mockResolvedValue(makeMetrics());
    render(<StatusBar />);
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });
    expect(screen.getByText(/✓/)).toBeInTheDocument();
  });

  it("shows agent count from metrics", async () => {
    mockInvoke.mockResolvedValue(makeMetrics({ active_agents: 8 }));
    render(<StatusBar />);
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });
    expect(screen.getByText(/8 ajan/)).toBeInTheDocument();
  });

  it("shows version string", () => {
    mockInvoke.mockResolvedValue(makeMetrics());
    render(<StatusBar />);
    expect(screen.getByText(/v0.2.2/)).toBeInTheDocument();
  });

  it("handles invoke errors gracefully", async () => {
    mockInvoke.mockRejectedValue(new Error("fail"));
    render(<StatusBar />);
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });
    expect(screen.getByRole("status")).toBeInTheDocument();
    expect(screen.getByText(/-- ajan/)).toBeInTheDocument();
  });

  it("uses setInterval for polling", () => {
    const setIntervalSpy = vi.spyOn(window, "setInterval");
    mockInvoke.mockResolvedValue(makeMetrics());
    render(<StatusBar />);
    expect(setIntervalSpy).toHaveBeenCalledWith(expect.any(Function), 15000);
    setIntervalSpy.mockRestore();
  });
});
