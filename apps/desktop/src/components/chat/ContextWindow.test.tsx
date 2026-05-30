import { describe, it, expect } from "vitest";
import { render, screen } from "../../test/test-utils";
import { ContextWindow } from "./ContextWindow";

describe("ContextWindow", () => {
  it("renders nothing when sources empty", () => {
    const { container } = render(<ContextWindow sources={[]} />);
    expect(container.innerHTML).toBe("");
  });

  it("renders source titles", () => {
    const sources = [
      { title: "Dosya1.md", source: "docs/", relevance: 0.9 },
      { title: "Dosya2.md", source: "docs/", relevance: 0.7 },
    ];
    const { container } = render(<ContextWindow sources={sources} />);
    expect(container.querySelector(".chat-context")).toHaveTextContent(
      "Dosya1.md",
    );
    expect(container.querySelector(".chat-context")).toHaveTextContent(
      "Dosya2.md",
    );
  });

  it("renders region with accessible label", () => {
    const sources = [{ title: "x.md", source: "docs/", relevance: 0.5 }];
    render(<ContextWindow sources={sources} />);
    expect(screen.getByRole("region")).toHaveAttribute(
      "aria-label",
      "Kullanılan kaynaklar",
    );
  });

  it("displays Kaynaklar heading", () => {
    const sources = [{ title: "x.md", source: "docs/", relevance: 0.5 }];
    render(<ContextWindow sources={sources} />);
    expect(screen.getByText("Kaynaklar:")).toBeInTheDocument();
  });
});
