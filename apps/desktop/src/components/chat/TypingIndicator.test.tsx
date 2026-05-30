import { describe, it, expect } from "vitest";
import { render, screen } from "../../test/test-utils";
import { TypingIndicator } from "./TypingIndicator";

describe("TypingIndicator", () => {
  it("renders status role", () => {
    render(<TypingIndicator />);
    expect(screen.getByRole("status")).toBeInTheDocument();
  });

  it("has aria-label indicating thinking state", () => {
    render(<TypingIndicator />);
    expect(screen.getByRole("status")).toHaveAttribute(
      "aria-label",
      "ADLER düşünüyor",
    );
  });

  it("shows thinking text", () => {
    render(<TypingIndicator />);
    expect(screen.getByText(/düşünüyor/)).toBeInTheDocument();
  });

  it("renders three dots", () => {
    const { container } = render(<TypingIndicator />);
    const dots = container.querySelectorAll(".typing-dot");
    expect(dots.length).toBe(3);
  });
});
