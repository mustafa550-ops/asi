import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import { ChatHistory } from "./ChatHistory";

const sessions = [
  { id: "1", title: "Kripto analizi", date: "2024-01-01" },
  { id: "2", title: "Sistem bakımı", date: "2024-01-02" },
];

describe("ChatHistory", () => {
  it("renders nothing when sessions is empty", () => {
    const { container } = render(
      <ChatHistory sessions={[]} activeId={null} onSelect={vi.fn()} onDelete={vi.fn()} />
    );
    expect(container.innerHTML).toBe("");
  });

  it("renders session list", () => {
    render(<ChatHistory sessions={sessions} activeId={null} onSelect={vi.fn()} onDelete={vi.fn()} />);
    expect(screen.getByText("Kripto analizi")).toBeInTheDocument();
    expect(screen.getByText("Sistem bakımı")).toBeInTheDocument();
  });

  it("marks active session", () => {
    render(<ChatHistory sessions={sessions} activeId="1" onSelect={vi.fn()} onDelete={vi.fn()} />);
    const items = screen.getAllByRole("option");
    expect(items[0]).toHaveClass("active");
  });

  it("sets aria-selected on active session", () => {
    render(<ChatHistory sessions={sessions} activeId="2" onSelect={vi.fn()} onDelete={vi.fn()} />);
    const items = screen.getAllByRole("option");
    expect(items[1]).toHaveAttribute("aria-selected", "true");
  });

  it("fires onSelect when session clicked", () => {
    const onSelect = vi.fn();
    render(<ChatHistory sessions={sessions} activeId={null} onSelect={onSelect} onDelete={vi.fn()} />);
    fireEvent.click(screen.getByText("Sistem bakımı"));
    expect(onSelect).toHaveBeenCalledWith("2");
  });

  it("uses listbox role", () => {
    render(<ChatHistory sessions={sessions} activeId={null} onSelect={vi.fn()} onDelete={vi.fn()} />);
    expect(screen.getByRole("listbox")).toHaveAttribute("aria-label", "Sohbet geçmişi");
  });
});
