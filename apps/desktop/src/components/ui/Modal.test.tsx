import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import { Modal } from "./Modal";

describe("Modal", () => {
  it("renders nothing when open is false", () => {
    const { container } = render(
      <Modal open={false} onClose={() => {}} title="Test">
        İçerik
      </Modal>,
    );
    expect(
      container.querySelector(".ui-modal-overlay"),
    ).not.toBeInTheDocument();
  });

  it("renders content when open is true", () => {
    render(
      <Modal open={true} onClose={() => {}} title="Test">
        İçerik
      </Modal>,
    );
    expect(screen.getByText("İçerik")).toBeInTheDocument();
    expect(screen.getByText("Test")).toBeInTheDocument();
  });

  it("renders dialog with correct aria attributes", () => {
    render(
      <Modal open={true} onClose={() => {}} title="Uyarı">
        İçerik
      </Modal>,
    );
    const dialog = screen.getByRole("dialog");
    expect(dialog).toHaveAttribute("aria-modal", "true");
    expect(dialog).toHaveAttribute("aria-label", "Uyarı");
  });

  it("calls onClose when overlay clicked", () => {
    const onClose = vi.fn();
    render(
      <Modal open={true} onClose={onClose} title="Test">
        İçerik
      </Modal>,
    );
    fireEvent.click(screen.getByRole("dialog"));
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("does not call onClose when modal body clicked", () => {
    const onClose = vi.fn();
    render(
      <Modal open={true} onClose={onClose} title="Test">
        İçerik
      </Modal>,
    );
    fireEvent.click(screen.getByText("İçerik"));
    expect(onClose).not.toHaveBeenCalled();
  });

  it("calls onClose on Escape key", () => {
    const onClose = vi.fn();
    render(
      <Modal open={true} onClose={onClose} title="Test">
        İçerik
      </Modal>,
    );
    fireEvent.keyDown(document, { key: "Escape" });
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("renders close button with aria-label", () => {
    render(
      <Modal open={true} onClose={() => {}} title="Test">
        İçerik
      </Modal>,
    );
    const closeBtn = screen.getByLabelText("Kapat");
    expect(closeBtn).toBeInTheDocument();
    fireEvent.click(closeBtn);
  });

  it("removes event listener on unmount", () => {
    const addSpy = vi.spyOn(document, "addEventListener");
    const removeSpy = vi.spyOn(document, "removeEventListener");
    const { unmount } = render(
      <Modal open={true} onClose={() => {}} title="Test">
        İçerik
      </Modal>,
    );
    expect(addSpy).toHaveBeenCalledWith("keydown", expect.any(Function));
    unmount();
    expect(removeSpy).toHaveBeenCalledWith("keydown", expect.any(Function));
    addSpy.mockRestore();
    removeSpy.mockRestore();
  });
});
