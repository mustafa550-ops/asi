import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, screen, act } from "../../test/test-utils";
import { ToastContainer, toast } from "./Toast";

describe("ToastContainer", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders empty container", () => {
    const { container } = render(<ToastContainer />);
    expect(container.querySelector(".ui-toast-container")).toBeInTheDocument();
    expect(container.querySelector(".ui-toast")).not.toBeInTheDocument();
  });

  it("displays toast when toast() is called", () => {
    render(<ToastContainer />);
    act(() => { toast("Başarılı", "success"); });
    expect(screen.getByText("Başarılı")).toBeInTheDocument();
    expect(screen.getByText("Başarılı")).toHaveClass("ui-toast-success");
  });

  it("displays multiple toasts", () => {
    render(<ToastContainer />);
    act(() => { toast("Bir"); toast("İki"); toast("Üç"); });
    expect(screen.getByText("Bir")).toBeInTheDocument();
    expect(screen.getByText("İki")).toBeInTheDocument();
    expect(screen.getByText("Üç")).toBeInTheDocument();
  });

  it("applies variant classes", () => {
    render(<ToastContainer />);
    act(() => { toast("Hata", "error"); });
    expect(screen.getByText("Hata")).toHaveClass("ui-toast-error");
    act(() => { toast("Bilgi", "info"); });
    expect(screen.getByText("Bilgi")).toHaveClass("ui-toast-info");
  });

  it("defaults to info variant", () => {
    render(<ToastContainer />);
    act(() => { toast("Varsayılan"); });
    expect(screen.getByText("Varsayılan")).toHaveClass("ui-toast-info");
  });

  it("uses aria-live polite", () => {
    const { container } = render(<ToastContainer />);
    expect(container.querySelector("[aria-live='polite']")).toBeInTheDocument();
  });

  it("removes toast after 4 seconds", async () => {
    render(<ToastContainer />);
    act(() => { toast("Geçici"); });
    expect(screen.getByText("Geçici")).toBeInTheDocument();
    act(() => { vi.advanceTimersByTime(4000); });
    expect(screen.queryByText("Geçici")).not.toBeInTheDocument();
  });
});
