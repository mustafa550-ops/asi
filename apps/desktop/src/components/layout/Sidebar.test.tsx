import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import { Sidebar } from "./Sidebar";

const items = [
  { id: "chat", label: "Sohbet", icon: "💬" },
  { id: "settings", label: "Ayarlar", icon: "⚙️" },
];

describe("Sidebar", () => {
  it("renders sidebar with aria label", () => {
    const { container } = render(
      <Sidebar
        items={items}
        activeTab="chat"
        onTabChange={vi.fn()}
        collapsed={false}
        onToggle={vi.fn()}
      />,
    );
    const sidebar = container.querySelector("aside");
    expect(sidebar).toHaveAttribute("aria-label", "Ana navigasyon");
  });

  it("shows logo when not collapsed", () => {
    render(
      <Sidebar
        items={items}
        activeTab="chat"
        onTabChange={vi.fn()}
        collapsed={false}
        onToggle={vi.fn()}
      />,
    );
    expect(screen.getByText("ADLER")).toBeInTheDocument();
  });

  it("hides logo when collapsed", () => {
    render(
      <Sidebar
        items={items}
        activeTab="chat"
        onTabChange={vi.fn()}
        collapsed={true}
        onToggle={vi.fn()}
      />,
    );
    expect(screen.queryByText("ADLER")).not.toBeInTheDocument();
  });

  it("hides labels when collapsed", () => {
    render(
      <Sidebar
        items={items}
        activeTab="chat"
        onTabChange={vi.fn()}
        collapsed={true}
        onToggle={vi.fn()}
      />,
    );
    expect(screen.queryByText("Sohbet")).not.toBeInTheDocument();
  });

  it("highlights active tab", () => {
    render(
      <Sidebar
        items={items}
        activeTab="settings"
        onTabChange={vi.fn()}
        collapsed={false}
        onToggle={vi.fn()}
      />,
    );
    const buttons = screen.getAllByRole("button");
    const activeBtn = buttons.find((b) => b.textContent?.includes("Ayarlar"));
    expect(activeBtn).toHaveClass("active");
    expect(activeBtn).toHaveAttribute("aria-current", "page");
  });

  it("fires onTabChange on click", () => {
    const onTabChange = vi.fn();
    render(
      <Sidebar
        items={items}
        activeTab="chat"
        onTabChange={onTabChange}
        collapsed={false}
        onToggle={vi.fn()}
      />,
    );
    const buttons = screen.getAllByRole("button");
    const settingsBtn = buttons.find((b) => b.textContent?.includes("Ayarlar"));
    fireEvent.click(settingsBtn!);
    expect(onTabChange).toHaveBeenCalledWith("settings");
  });

  it("fires onToggle on toggle button click", () => {
    const onToggle = vi.fn();
    render(
      <Sidebar
        items={items}
        activeTab="chat"
        onTabChange={vi.fn()}
        collapsed={false}
        onToggle={onToggle}
      />,
    );
    fireEvent.click(screen.getByLabelText("Daralt"));
    expect(onToggle).toHaveBeenCalledOnce();
  });

  it("has correct aria-label on toggle button when collapsed", () => {
    render(
      <Sidebar
        items={items}
        activeTab="chat"
        onTabChange={vi.fn()}
        collapsed={true}
        onToggle={vi.fn()}
      />,
    );
    expect(screen.getByLabelText("Genişlet")).toBeInTheDocument();
  });

  it("renders children in footer", () => {
    render(
      <Sidebar
        items={items}
        activeTab="chat"
        onTabChange={vi.fn()}
        collapsed={false}
        onToggle={vi.fn()}
      >
        footer
      </Sidebar>,
    );
    expect(screen.getByText("footer")).toBeInTheDocument();
  });

  it("applies collapsed class", () => {
    const { container } = render(
      <Sidebar
        items={items}
        activeTab="chat"
        onTabChange={vi.fn()}
        collapsed={true}
        onToggle={vi.fn()}
      />,
    );
    expect(container.querySelector(".layout-sidebar")).toHaveClass("collapsed");
  });

  it("renders all nav items", () => {
    render(
      <Sidebar
        items={items}
        activeTab="chat"
        onTabChange={vi.fn()}
        collapsed={false}
        onToggle={vi.fn()}
      />,
    );
    expect(screen.getByText("Sohbet")).toBeInTheDocument();
    expect(screen.getByText("Ayarlar")).toBeInTheDocument();
  });
});
