import { describe, it, expect } from "vitest";
import { render, screen } from "../../test/test-utils";
import { Badge } from "./Badge";

describe("Badge", () => {
  it("renders children text", () => {
    render(<Badge>Aktif</Badge>);
    expect(screen.getByText("Aktif")).toBeInTheDocument();
  });

  it("applies neutral variant by default", () => {
    render(<Badge>Nötr</Badge>);
    expect(screen.getByText("Nötr")).toHaveClass("ui-badge-neutral");
  });

  it("applies variant classes", () => {
    const { rerender } = render(<Badge variant="success">Başarılı</Badge>);
    expect(screen.getByText("Başarılı")).toHaveClass("ui-badge-success");
    rerender(<Badge variant="error">Hata</Badge>);
    expect(screen.getByText("Hata")).toHaveClass("ui-badge-error");
    rerender(<Badge variant="warning">Uyarı</Badge>);
    expect(screen.getByText("Uyarı")).toHaveClass("ui-badge-warning");
    rerender(<Badge variant="info">Bilgi</Badge>);
    expect(screen.getByText("Bilgi")).toHaveClass("ui-badge-info");
  });

  it("renders as span element", () => {
    const { container } = render(<Badge>Test</Badge>);
    expect(container.querySelector("span")).toBeInTheDocument();
  });
});
