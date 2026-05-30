import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import { SlashCommands } from "./SlashCommands";

describe("SlashCommands", () => {
  it("renders filtered commands", () => {
    render(<SlashCommands input="/a" onSelect={vi.fn()} onClose={vi.fn()} />);
    expect(screen.getByText("/analiz")).toBeInTheDocument();
  });

  it("renders nothing when no match", () => {
    const { container } = render(
      <SlashCommands input="/xyz" onSelect={vi.fn()} onClose={vi.fn()} />,
    );
    expect(container.innerHTML).toBe("");
  });

  it("renders all commands when input is just /", () => {
    render(<SlashCommands input="/" onSelect={vi.fn()} onClose={vi.fn()} />);
    expect(screen.getByText("/analiz")).toBeInTheDocument();
    expect(screen.getByText("/donanim")).toBeInTheDocument();
    expect(screen.getByText("/piyasa")).toBeInTheDocument();
    expect(screen.getByText("/sistem")).toBeInTheDocument();
    expect(screen.getByText("/bellek")).toBeInTheDocument();
  });

  it("highlights selected command", () => {
    render(<SlashCommands input="/" onSelect={vi.fn()} onClose={vi.fn()} />);
    const items = screen.getAllByRole("option");
    expect(items[0]).toHaveClass("active");
  });

  it("fires onSelect on click", () => {
    const onSelect = vi.fn();
    render(<SlashCommands input="/" onSelect={onSelect} onClose={vi.fn()} />);
    fireEvent.click(screen.getByText("/analiz"));
    expect(onSelect).toHaveBeenCalledWith("/analiz");
  });

  it("fires onSelect on Enter key", () => {
    const onSelect = vi.fn();
    render(<SlashCommands input="/" onSelect={onSelect} onClose={vi.fn()} />);
    fireEvent.keyDown(document, { key: "Enter" });
    expect(onSelect).toHaveBeenCalledWith("/analiz");
  });

  it("fires onClose on Escape key", () => {
    const onClose = vi.fn();
    render(<SlashCommands input="/" onSelect={vi.fn()} onClose={onClose} />);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("uses listbox role", () => {
    render(<SlashCommands input="/" onSelect={vi.fn()} onClose={vi.fn()} />);
    expect(screen.getByRole("listbox")).toBeInTheDocument();
  });
});
