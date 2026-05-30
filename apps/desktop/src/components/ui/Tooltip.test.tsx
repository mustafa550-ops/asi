import { describe, it, expect } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import { Tooltip } from "./Tooltip";

describe("Tooltip", () => {
  it("renders children", () => {
    render(<Tooltip text="detay">Hover</Tooltip>);
    expect(screen.getByText("Hover")).toBeInTheDocument();
  });

  it("shows popup on mouse enter", () => {
    render(<Tooltip text="detay">Hover</Tooltip>);
    const wrapper = screen.getByRole("tooltip");
    fireEvent.mouseEnter(wrapper);
    expect(screen.getByText("detay")).toBeInTheDocument();
    expect(screen.getByText("detay")).toHaveClass("ui-tooltip-popup");
  });

  it("hides popup on mouse leave", () => {
    render(<Tooltip text="detay">Hover</Tooltip>);
    const wrapper = screen.getByRole("tooltip");
    fireEvent.mouseEnter(wrapper);
    expect(screen.getByText("detay")).toBeInTheDocument();
    fireEvent.mouseLeave(wrapper);
    expect(screen.queryByText("detay")).not.toBeInTheDocument();
  });

  it("shows popup on focus and hides on blur", () => {
    render(<Tooltip text="ipucu">Odak</Tooltip>);
    const wrapper = screen.getByRole("tooltip");
    fireEvent.focus(wrapper);
    expect(screen.getByText("ipucu")).toBeInTheDocument();
    fireEvent.blur(wrapper);
    expect(screen.queryByText("ipucu")).not.toBeInTheDocument();
  });

  it("sets aria-label to tooltip text", () => {
    render(<Tooltip text="aciklama">Element</Tooltip>);
    expect(screen.getByRole("tooltip")).toHaveAttribute(
      "aria-label",
      "aciklama",
    );
  });

  it("is focusable via tabIndex 0", () => {
    render(<Tooltip text="test">Element</Tooltip>);
    expect(screen.getByRole("tooltip")).toHaveAttribute("tabindex", "0");
  });
});
