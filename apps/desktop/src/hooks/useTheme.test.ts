import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useTheme } from "./useTheme";

const storedKey = "adler-theme";

describe("useTheme", () => {
  beforeEach(() => {
    localStorage.clear();
    document.documentElement.removeAttribute("data-theme");
    window.matchMedia = vi.fn().mockImplementation((query: string) => ({
      matches: false,
      media: query,
      onchange: null,
      addListener: vi.fn(),
      removeListener: vi.fn(),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn(),
    }));
  });

  afterEach(() => {
    localStorage.clear();
  });

  it("returns dark mode by default when no preference", () => {
    const { result } = renderHook(() => useTheme());
    expect(result.current.mode).toBe("dark");
  });

  it("sets data-theme attribute on mount", () => {
    renderHook(() => useTheme());
    expect(document.documentElement.getAttribute("data-theme")).toBe("dark");
  });

  it("saves theme to localStorage", () => {
    renderHook(() => useTheme());
    expect(localStorage.getItem(storedKey)).toBe("dark");
  });

  it("toggles from dark to light", () => {
    const { result } = renderHook(() => useTheme());
    act(() => result.current.toggle());
    expect(result.current.mode).toBe("light");
  });

  it("toggles from light to dark", () => {
    localStorage.setItem(storedKey, "light");
    const { result } = renderHook(() => useTheme());
    expect(result.current.mode).toBe("light");
    act(() => result.current.toggle());
    expect(result.current.mode).toBe("dark");
  });

  it("persists toggle to localStorage and updates data-theme", () => {
    const { result } = renderHook(() => useTheme());
    act(() => result.current.toggle());
    expect(localStorage.getItem(storedKey)).toBe("light");
    expect(document.documentElement.getAttribute("data-theme")).toBe("light");
  });

  it("reads stored theme from localStorage", () => {
    localStorage.setItem(storedKey, "light");
    const { result } = renderHook(() => useTheme());
    expect(result.current.mode).toBe("light");
  });
});
