import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import { Input } from "./Input";

describe("Input", () => {
  it("renders an input element", () => {
    render(<Input />);
    expect(screen.getByRole("textbox")).toBeInTheDocument();
  });

  it("renders label when provided", () => {
    render(<Input label="Kullanıcı Adı" id="username" />);
    expect(screen.getByLabelText("Kullanıcı Adı")).toBeInTheDocument();
  });

  it("does not render label when not provided", () => {
    const { container } = render(<Input />);
    expect(container.querySelector(".ui-input-label")).not.toBeInTheDocument();
  });

  it("applies custom className", () => {
    render(<Input className="custom" />);
    expect(screen.getByRole("textbox")).toHaveClass("custom");
  });

  it("fires onChange", () => {
    const onChange = vi.fn();
    render(<Input onChange={onChange} />);
    fireEvent.change(screen.getByRole("textbox"), { target: { value: "test" } });
    expect(onChange).toHaveBeenCalled();
  });

  it("passes placeholder", () => {
    render(<Input placeholder="yaz..." />);
    expect(screen.getByPlaceholderText("yaz...")).toBeInTheDocument();
  });

  it("associates label with input via htmlFor", () => {
    render(<Input label="Email" id="email" />);
    expect(screen.getByLabelText("Email")).toHaveAttribute("id", "email");
  });

  it("supports disabled state", () => {
    render(<Input disabled />);
    expect(screen.getByRole("textbox")).toBeDisabled();
  });
});
