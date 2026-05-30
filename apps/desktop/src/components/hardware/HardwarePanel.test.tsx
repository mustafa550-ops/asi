import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import HardwarePanel from "./HardwarePanel";

describe("HardwarePanel", () => {
  it("renders the panel title", () => {
    render(<HardwarePanel />);
    expect(screen.getByText("Donanim Paneli")).toBeTruthy();
  });

  it("renders GPIO section", () => {
    render(<HardwarePanel />);
    expect(screen.getByText("GPIO")).toBeTruthy();
    expect(screen.getByText("HIGH")).toBeTruthy();
    expect(screen.getByText("LOW")).toBeTruthy();
    expect(screen.getByText("Oku")).toBeTruthy();
  });

  it("renders relay section", () => {
    render(<HardwarePanel />);
    expect(screen.getByText("Role")).toBeTruthy();
  });

  it("renders device list section", () => {
    render(<HardwarePanel />);
    expect(screen.getByText(/Kesfedilen Cihazlar/)).toBeTruthy();
  });

  it("shows empty state when no devices", () => {
    render(<HardwarePanel />);
    expect(screen.getByText("Cihaz bulunamadi")).toBeTruthy();
  });
});
