import { describe, it, expect } from "vitest";
import { render, screen } from "../test/test-utils";
import { MarkdownRenderer } from "./MarkdownRenderer";

describe("MarkdownRenderer", () => {
  it("renders plain text", () => {
    render(<MarkdownRenderer content="Merhaba Dünya" />);
    expect(screen.getByText("Merhaba Dünya")).toBeInTheDocument();
  });

  it("renders bold text", () => {
    render(<MarkdownRenderer content="**kalın** metin" />);
    expect(screen.getByText("kalın", { selector: "strong" })).toBeInTheDocument();
  });

  it("renders italic text", () => {
    render(<MarkdownRenderer content="*italik* metin" />);
    expect(screen.getByText("italik", { selector: "em" })).toBeInTheDocument();
  });

  it("renders inline code", () => {
    render(<MarkdownRenderer content="`kod` parçası" />);
    expect(document.querySelector("code")).toHaveTextContent("kod");
  });

  it("renders links", () => {
    render(<MarkdownRenderer content="[tıklayın](https://example.com)" />);
    const link = document.querySelector("a");
    expect(link).toHaveAttribute("href", "https://example.com");
    expect(link).toHaveAttribute("target", "_blank");
  });

  it("renders h1 heading", () => {
    render(<MarkdownRenderer content="# Başlık 1" />);
    const h1 = document.querySelector(".md-h1");
    expect(h1).toHaveTextContent("Başlık 1");
  });

  it("renders h2 heading", () => {
    render(<MarkdownRenderer content="## Başlık 2" />);
    const h2 = document.querySelector(".md-h2");
    expect(h2).toHaveTextContent("Başlık 2");
  });

  it("renders h3 heading", () => {
    render(<MarkdownRenderer content="### Başlık 3" />);
    const h3 = document.querySelector(".md-h3");
    expect(h3).toHaveTextContent("Başlık 3");
  });

  it("renders unordered list items", () => {
    const { container } = render(<MarkdownRenderer content={"- madde 1\n- madde 2"} />);
    const items = container.querySelectorAll(".md-li");
    expect(items.length).toBe(2);
  });

  it("renders ordered list items", () => {
    const { container } = render(<MarkdownRenderer content={"1. birinci\n2. ikinci"} />);
    const items = container.querySelectorAll(".md-li");
    expect(items.length).toBe(2);
  });

  it("renders fenced code blocks", () => {
    render(<MarkdownRenderer content={"```rust\nfn main() {}\n```"} />);
    const codeBlock = document.querySelector(".md-code-block");
    expect(codeBlock).toBeInTheDocument();
    expect(document.querySelector(".md-code")).toHaveTextContent("fn main() {}");
  });

  it("renders code blocks with language class", () => {
    render(<MarkdownRenderer content={"```typescript\nconst x = 1;\n```"} />);
    const code = document.querySelector(".md-code");
    expect(code).toHaveClass("lang-typescript");
  });

  it("escapes HTML in content", () => {
    render(<MarkdownRenderer content="<script>alert('xss')</script>" />);
    expect(screen.queryByText("alert('xss')")).not.toBeInTheDocument();
  });

  it("uses memo to avoid re-rendering", () => {
    const { rerender } = render(<MarkdownRenderer content="ilk" />);
    expect(screen.getByText("ilk")).toBeInTheDocument();
    rerender(<MarkdownRenderer content="ikinci" />);
    expect(screen.getByText("ikinci")).toBeInTheDocument();
  });
});
