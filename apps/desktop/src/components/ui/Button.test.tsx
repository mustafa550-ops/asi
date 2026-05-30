import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import { Button } from "./Button";

describe("Button", () => {
  it("renders children", () => {
    render(<Button>Gönder</Button>);
    expect(screen.getByRole("button", { name: /Gönder/i })).toBeInTheDocument();
  });

  it("applies variant classes", () => {
    const { rerender } = render(<Button variant="danger">Sil</Button>);
    expect(screen.getByRole("button")).toHaveClass("ui-btn-danger");
    rerender(<Button variant="success">Onay</Button>);
    expect(screen.getByRole("button")).toHaveClass("ui-btn-success");
  });

  it("applies size classes", () => {
    render(<Button size="sm">Küçük</Button>);
    expect(screen.getByRole("button")).toHaveClass("ui-btn-sm");
  });

  it("uses default variant primary and size md", () => {
    render(<Button>Varsayılan</Button>);
    const btn = screen.getByRole("button");
    expect(btn).toHaveClass("ui-btn-primary");
    expect(btn).toHaveClass("ui-btn-md");
  });

  it("merges custom className", () => {
    render(<Button className="custom-class">Özel</Button>);
    expect(screen.getByRole("button")).toHaveClass("custom-class");
  });

  it("fires onClick", () => {
    const onClick = vi.fn();
    render(<Button onClick={onClick}>Tıkla</Button>);
    fireEvent.click(screen.getByRole("button"));
    expect(onClick).toHaveBeenCalledOnce();
  });

  it("supports disabled state", () => {
    render(<Button disabled>Pasif</Button>);
    expect(screen.getByRole("button")).toBeDisabled();
  });

  it("passes through native button attributes", () => {
    render(<Button type="submit" data-testid="sub">Gönder</Button>);
    const btn = screen.getByTestId("sub");
    expect(btn).toHaveAttribute("type", "submit");
  });
});
