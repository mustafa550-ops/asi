import { describe, it, expect } from "vitest";
import { render, screen } from "../../test/test-utils";
import { Header } from "./Header";

describe("Header", () => {
  it("renders title", () => {
    render(<Header title="Sohbet" />);
    expect(screen.getByRole("banner")).toBeInTheDocument();
    expect(screen.getByText("Sohbet")).toBeInTheDocument();
  });

  it("renders subtitle when provided", () => {
    render(<Header title="Sohbet" subtitle="Otonom" />);
    expect(screen.getByText("Otonom")).toBeInTheDocument();
  });

  it("does not render subtitle when not provided", () => {
    render(<Header title="Sohbet" />);
    expect(screen.queryByText("Otonom")).not.toBeInTheDocument();
  });

  it("renders children in actions area", () => {
    render(
      <Header title="Sohbet">
        <button>Eylem</button>
      </Header>,
    );
    expect(screen.getByText("Eylem")).toBeInTheDocument();
  });

  it("renders title as h1", () => {
    render(<Header title="Dashboard" />);
    expect(screen.getByText("Dashboard").tagName).toBe("H1");
  });
});
