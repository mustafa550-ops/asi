import { describe, it, expect, vi } from "vitest";
import { render, screen } from "../../test/test-utils";
import { MainLayout } from "./MainLayout";

vi.mock("./Sidebar", () => ({
  Sidebar: ({ items, activeTab }: { items: { id: string; label: string }[]; activeTab: string }) => (
    <div data-testid="mock-sidebar">
      <span data-testid="sidebar-active">{activeTab}</span>
      <span data-testid="sidebar-count">{items.length}</span>
    </div>
  ),
}));

vi.mock("./Header", () => ({
  Header: ({ title, subtitle }: { title: string; subtitle?: string }) => (
    <div data-testid="mock-header">
      <span data-testid="header-title">{title}</span>
      {subtitle && <span data-testid="header-subtitle">{subtitle}</span>}
    </div>
  ),
}));

vi.mock("./StatusBar", () => ({
  StatusBar: () => <div data-testid="mock-statusbar" />,
}));

describe("MainLayout", () => {
  const noop = () => {};

  it("renders sidebar with correct active tab", () => {
    render(<MainLayout activeTab="chat" onTabChange={noop}>child</MainLayout>);
    expect(screen.getByTestId("sidebar-active")).toHaveTextContent("chat");
  });

  it("renders header with current tab label", () => {
    render(<MainLayout activeTab="chat" onTabChange={noop}>child</MainLayout>);
    expect(screen.getByTestId("header-title")).toHaveTextContent("Sohbet");
  });

  it("renders header with ADLER label for unknown tab", () => {
    render(<MainLayout activeTab="unknown" onTabChange={noop}>child</MainLayout>);
    expect(screen.getByTestId("header-title")).toHaveTextContent("ADLER ASI");
  });

  it("renders header subtitle", () => {
    render(<MainLayout activeTab="chat" onTabChange={noop}>child</MainLayout>);
    expect(screen.getByTestId("header-subtitle")).toHaveTextContent("Otonom");
  });

  it("renders children in main content", () => {
    render(<MainLayout activeTab="dashboard" onTabChange={noop}><p>test child</p></MainLayout>);
    expect(screen.getByRole("main")).toHaveTextContent("test child");
  });

  it("renders status bar", () => {
    render(<MainLayout activeTab="chat" onTabChange={noop}>child</MainLayout>);
    expect(screen.getByTestId("mock-statusbar")).toBeInTheDocument();
  });

  it("passes 5 nav items to sidebar", () => {
    render(<MainLayout activeTab="chat" onTabChange={noop}>child</MainLayout>);
    expect(screen.getByTestId("sidebar-count")).toHaveTextContent("5");
  });
});
