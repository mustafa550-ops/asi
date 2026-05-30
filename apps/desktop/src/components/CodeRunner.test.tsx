import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import CodeRunner from "./CodeRunner";

describe("CodeRunner", () => {
  it("renders code content", () => {
    render(<CodeRunner code='print("hello")' language="python" />);
    expect(screen.getByText(/print/)).toBeTruthy();
  });

  it("shows run button", () => {
    render(<CodeRunner code="console.log('test')" />);
    expect(screen.getByRole("button", { name: /çaliştir/i })).toBeTruthy();
  });

  it("shows language label when provided", () => {
    render(<CodeRunner code="fn main() {}" language="rust" />);
    expect(screen.getByText("rust")).toBeTruthy();
  });

  it("shows line numbers when enabled", () => {
    const { container } = render(<CodeRunner code={"line1\nline2\nline3"} showLineNumbers />);
    const lineNums = container.querySelectorAll(".coderunner-line-num");
    expect(lineNums.length).toBe(3);
  });
});
