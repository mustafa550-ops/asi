import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "../../test/test-utils";
import { SystemMetrics } from "./SystemMetrics";

const mockInvoke = vi.fn();
vi.mock("../../lib/tauri", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

describe("SystemMetrics", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders metric bars", () => {
    mockInvoke.mockResolvedValue("sistem durumu");
    render(<SystemMetrics />);
    expect(screen.getByText("CPU Kullanımı")).toBeInTheDocument();
    expect(screen.getByText("Bellek Kullanımı")).toBeInTheDocument();
    expect(screen.getByText("Çalışma Süresi")).toBeInTheDocument();
  });

  it("renders region with accessible label", () => {
    mockInvoke.mockResolvedValue("sistem durumu");
    render(<SystemMetrics />);
    expect(screen.getByRole("region")).toHaveAttribute("aria-label", "Sistem metrikleri");
  });

  it("renders heading", () => {
    mockInvoke.mockResolvedValue("sistem durumu");
    render(<SystemMetrics />);
    expect(screen.getByText("Sistem Metrikleri")).toBeInTheDocument();
  });

  it("handles invoke errors gracefully", () => {
    mockInvoke.mockRejectedValue(new Error("fail"));
    render(<SystemMetrics />);
    expect(screen.getByText("CPU Kullanımı")).toBeInTheDocument();
  });

  it("uses setInterval for polling", () => {
    const setIntervalSpy = vi.spyOn(window, "setInterval");
    mockInvoke.mockResolvedValue("sistem durumu");
    render(<SystemMetrics />);
    expect(setIntervalSpy).toHaveBeenCalledWith(expect.any(Function), 15000);
    setIntervalSpy.mockRestore();
  });
});
