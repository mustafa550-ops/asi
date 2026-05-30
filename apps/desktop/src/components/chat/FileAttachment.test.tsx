import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import { FileAttachment } from "./FileAttachment";

const mockToast = vi.fn();
vi.mock("../ui/Toast", () => ({
  toast: (...args: unknown[]) => mockToast(...args),
}));

describe("FileAttachment", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders attach button", () => {
    render(<FileAttachment onAttach={vi.fn()} />);
    expect(screen.getByLabelText("Dosya ekle")).toBeInTheDocument();
  });

  it("has hidden file input with correct accept types", () => {
    render(<FileAttachment onAttach={vi.fn()} />);
    const input = document.querySelector('input[type="file"]');
    expect(input).toBeInTheDocument();
    expect(input).toHaveAttribute(
      "accept",
      ".md,.txt,.json,.csv,.yaml,.yml,.toml,.rs,.ts,.tsx,.py,.js,.html,.css,.xml,.sql",
    );
    expect(input).toHaveAttribute("hidden");
  });

  it("applies dragging class on dragover", () => {
    const { container } = render(<FileAttachment onAttach={vi.fn()} />);
    const dropZone = container.querySelector(".file-attach")!;
    fireEvent.dragOver(dropZone);
    expect(dropZone).toHaveClass("file-attach-dragging");
  });

  it("removes dragging class on dragleave", () => {
    const { container } = render(<FileAttachment onAttach={vi.fn()} />);
    const dropZone = container.querySelector(".file-attach")!;
    fireEvent.dragOver(dropZone);
    expect(dropZone).toHaveClass("file-attach-dragging");
    fireEvent.dragLeave(dropZone);
    expect(dropZone).not.toHaveClass("file-attach-dragging");
  });

  it("handles drop event", () => {
    const onAttach = vi.fn();
    const { container } = render(<FileAttachment onAttach={onAttach} />);
    const dropZone = container.querySelector(".file-attach")!;
    const file = new File(["test"], "test.md", { type: "text/markdown" });
    Object.defineProperty(file, "text", {
      value: () => Promise.resolve("test"),
    });
    fireEvent.drop(dropZone, { dataTransfer: { files: [file] } });
    expect(dropZone).not.toHaveClass("file-attach-dragging");
  });
});
