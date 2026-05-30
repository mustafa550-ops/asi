import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, act } from "../../test/test-utils";
import ApprovalPanel from "./ApprovalPanel";

const mockInvoke = vi.fn();
vi.mock("../../lib/tauri", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

let triggerApproval: ((payload: string) => void) | null = null;

vi.mock("../../hooks/useTauriEvent", () => ({
  useTauriEvent: (_event: string, cb: (payload: string) => void) => {
    triggerApproval = cb;
  },
}));

describe("ApprovalPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    triggerApproval = null;
  });

  it("returns null when no approvals", () => {
    const { container } = render(<ApprovalPanel />);
    expect(container.innerHTML).toBe("");
  });

  it("shows approval card after event", () => {
    render(<ApprovalPanel />);
    act(() => { triggerApproval?.(JSON.stringify({ id: "1", task: "test", summary: "detay" })); });
    expect(screen.getByText(/Görev/)).toBeInTheDocument();
    expect(screen.getByText("test")).toBeInTheDocument();
  });

  it("calls onResult when approve clicked", async () => {
    mockInvoke.mockResolvedValue("success");
    const onResult = vi.fn();
    render(<ApprovalPanel onResult={onResult} />);
    act(() => { triggerApproval?.(JSON.stringify({ id: "x1", task: "test", summary: "detay" })); });
    screen.getByText("Onayla").click();
    await vi.waitFor(() => {
      expect(onResult).toHaveBeenCalledWith("approve", "success");
    });
  });
});
