import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import { NotificationCenter } from "./NotificationCenter";

describe("NotificationCenter", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders bell button", () => {
    render(<NotificationCenter />);
    expect(screen.getByLabelText(/Bildirimler/)).toBeInTheDocument();
  });

  it("shows empty message when dropdown opened and no notifications", () => {
    render(<NotificationCenter />);
    fireEvent.click(screen.getByLabelText(/Bildirimler/));
    expect(screen.getByText("Bildirim yok.")).toBeInTheDocument();
  });

  it("toggles dropdown on bell click", () => {
    render(<NotificationCenter />);
    const bell = screen.getByLabelText(/Bildirimler/);
    fireEvent.click(bell);
    expect(screen.getByText("Bildirimler")).toBeInTheDocument();
    fireEvent.click(bell);
    expect(screen.queryByText("Bildirimler")).not.toBeInTheDocument();
  });

  it("renders dropdown with menu role when open", () => {
    render(<NotificationCenter />);
    fireEvent.click(screen.getByLabelText(/Bildirimler/));
    expect(screen.getByRole("menu")).toBeInTheDocument();
  });
});
