import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import { EventStream } from "./EventStream";

describe("EventStream", () => {
  it("renders event stream header", () => {
    render(<EventStream />);
    expect(screen.getByText("Olay Akışı")).toBeInTheDocument();
  });

  it("renders filter input", () => {
    render(<EventStream />);
    expect(screen.getByLabelText("Olay filtrele")).toBeInTheDocument();
  });

  it("shows empty message when no events", () => {
    render(<EventStream />);
    expect(screen.getByText("Henüz olay yok.")).toBeInTheDocument();
  });

  it("renders region with log role", () => {
    render(<EventStream />);
    expect(screen.getByRole("log")).toBeInTheDocument();
  });

  it("renders aria-label on log", () => {
    render(<EventStream />);
    expect(screen.getByRole("log")).toHaveAttribute(
      "aria-label",
      "Sistem olayları",
    );
  });
});
