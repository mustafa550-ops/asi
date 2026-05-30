import { describe, it, expect } from "vitest";
import { render, screen } from "../../test/test-utils";
import { Card } from "./Card";

describe("Card", () => {
  it("renders children", () => {
    render(<Card>İçerik</Card>);
    expect(screen.getByText("İçerik")).toBeInTheDocument();
  });

  it("renders title when provided", () => {
    render(<Card title="Başlık">İçerik</Card>);
    expect(screen.getByText("Başlık")).toBeInTheDocument();
    expect(screen.getByText("Başlık").tagName).toBe("H3");
  });

  it("does not render title h3 when no title", () => {
    const { container } = render(<Card>İçerik</Card>);
    expect(container.querySelector(".ui-card-title")).not.toBeInTheDocument();
  });

  it("applies custom className", () => {
    const { container } = render(<Card className="custom">İçerik</Card>);
    expect(container.querySelector(".ui-card")).toHaveClass("custom");
  });

  it("renders complex children", () => {
    render(
      <Card>
        <span data-testid="child">Nested</span>
      </Card>,
    );
    expect(screen.getByTestId("child")).toBeInTheDocument();
  });
});
