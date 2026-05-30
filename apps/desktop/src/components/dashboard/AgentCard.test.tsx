import { describe, it, expect } from "vitest";
import { render, screen } from "../../test/test-utils";
import { AgentCard } from "./AgentCard";

describe("AgentCard", () => {
  it("renders agent name and description", () => {
    render(
      <AgentCard name="Diagnostic" status="ready" description="Hata teşhisi" />,
    );
    expect(screen.getByText("Diagnostic")).toBeInTheDocument();
    expect(screen.getByText("Hata teşhisi")).toBeInTheDocument();
  });

  it("renders badge with correct variant for ready status", () => {
    render(<AgentCard name="Test" status="ready" description="test" />);
    expect(screen.getByText("Hazır")).toBeInTheDocument();
    expect(screen.getByText("Hazır")).toHaveClass("ui-badge-success");
  });

  it("renders badge for idle status", () => {
    render(<AgentCard name="Test" status="idle" description="test" />);
    expect(screen.getByText("Boşta")).toHaveClass("ui-badge-info");
  });

  it("renders badge for error status", () => {
    render(<AgentCard name="Test" status="error" description="test" />);
    expect(screen.getByText("Hata")).toHaveClass("ui-badge-error");
  });

  it("renders badge for busy status", () => {
    render(<AgentCard name="Test" status="busy" description="test" />);
    expect(screen.getByText("Meşgul")).toHaveClass("ui-badge-warning");
  });

  it("uses article role with aria-label", () => {
    render(<AgentCard name="Intent Judge" status="ready" description="test" />);
    expect(screen.getByRole("article")).toHaveAttribute(
      "aria-label",
      "Ajan: Intent Judge",
    );
  });

  it("shows last action when provided", () => {
    render(
      <AgentCard
        name="Test"
        status="ready"
        description="test"
        lastAction="Analiz tamamlandı"
      />,
    );
    expect(screen.getByText("Son: Analiz tamamlandı")).toBeInTheDocument();
  });

  it("shows run count when provided", () => {
    render(
      <AgentCard name="Test" status="ready" description="test" runCount={42} />,
    );
    expect(screen.getByText("42 çalışma")).toBeInTheDocument();
  });

  it("does not show last action when not provided", () => {
    render(<AgentCard name="Test" status="ready" description="test" />);
    expect(screen.queryByText(/Son:/)).not.toBeInTheDocument();
  });
});
