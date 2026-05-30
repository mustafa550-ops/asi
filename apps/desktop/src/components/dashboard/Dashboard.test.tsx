import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "../../test/test-utils";
import Dashboard from "./Dashboard";

const mockInvoke = vi.fn();
vi.mock("../../lib/tauri", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

vi.mock("./AgentCard", () => ({
  AgentCard: ({ name, status }: { name: string; status: string }) => (
    <div data-testid="agent-card">
      {name} ({status})
    </div>
  ),
}));

vi.mock("./EventStream", () => ({
  EventStream: () => <div data-testid="event-stream" />,
}));

vi.mock("./SystemMetrics", () => ({
  SystemMetrics: () => <div data-testid="system-metrics" />,
}));

vi.mock("../ui/Card", () => ({
  Card: ({ title, children }: { title: string; children: React.ReactNode }) => (
    <div data-testid="card">
      <h3>{title}</h3>
      {children}
    </div>
  ),
}));

const AGENTS_JSON = JSON.stringify([
  { name: "Intent Judge", status: "ready", description: "Niyet sınıflandırma" },
  { name: "Diagnostic", status: "idle", description: "Hata teşhisi" },
  { name: "Hardware", status: "idle", description: "GPIO kontrolü" },
  { name: "Market Analyst", status: "idle", description: "Kripto analizi" },
  { name: "System Manager", status: "ready", description: "Sistem izleme" },
  { name: "Document Analyst", status: "idle", description: "Belge analizi" },
  { name: "Voice Handler", status: "idle", description: "Ses işleme" },
  { name: "Supervisor", status: "ready", description: "Optimizasyon" },
]);

describe("Dashboard", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockInvoke.mockImplementation((cmd: string) => {
      if (cmd === "get_agent_statuses") return Promise.resolve(AGENTS_JSON);
      return Promise.resolve("sistem durumu");
    });
  });

  it("renders agent cards", async () => {
    render(<Dashboard />);
    await waitFor(() => {
      expect(screen.getByText("Intent Judge (ready)")).toBeInTheDocument();
      expect(screen.getByText("Diagnostic (idle)")).toBeInTheDocument();
      expect(screen.getByText("Supervisor (ready)")).toBeInTheDocument();
    });
  });

  it("renders 8 agent cards", async () => {
    render(<Dashboard />);
    await waitFor(() => {
      const cards = screen.getAllByTestId("agent-card");
      expect(cards.length).toBe(8);
    });
  });

  it("renders system metrics", () => {
    render(<Dashboard />);
    expect(screen.getByTestId("system-metrics")).toBeInTheDocument();
  });

  it("renders event stream", () => {
    render(<Dashboard />);
    expect(screen.getByTestId("event-stream")).toBeInTheDocument();
  });

  it("renders agent cards heading", () => {
    render(<Dashboard />);
    expect(screen.getByText("Ajan Durumu")).toBeInTheDocument();
  });

  it("renders system info section", async () => {
    render(<Dashboard />);
    await waitFor(() => {
      expect(screen.getByText("Sistem Bilgisi")).toBeInTheDocument();
    });
  });

  it("shows error fallback when system info fails", async () => {
    mockInvoke.mockImplementation((cmd: string) => {
      if (cmd === "get_agent_statuses") return Promise.resolve(AGENTS_JSON);
      return Promise.reject(new Error("db error"));
    });
    render(<Dashboard />);
    await waitFor(() => {
      expect(screen.getByText("Sistem bilgisi alınamadı")).toBeInTheDocument();
    });
  });
});
