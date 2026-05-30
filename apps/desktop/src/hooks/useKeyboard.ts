import { useEffect, useCallback } from "react";

type KeyAction = Record<string, () => void>;

export function useKeyboard(actions: KeyAction, enabled = true) {
  const handler = useCallback(
    (e: KeyboardEvent) => {
      if (!enabled) return;
      const target = e.target as HTMLElement;
      const isInput =
        target.tagName === "INPUT" ||
        target.tagName === "TEXTAREA" ||
        target.isContentEditable;

      if (e.key === "Escape" && actions["Escape"]) {
        actions["Escape"]();
        return;
      }

      if (e.key === "Enter" && !isInput && actions["Enter"]) {
        actions["Enter"]();
        return;
      }

      if (e.ctrlKey || e.metaKey) {
        const combo = `Ctrl+${e.key}`;
        if (actions[combo]) {
          e.preventDefault();
          actions[combo]();
        }
      }
    },
    [actions, enabled],
  );

  useEffect(() => {
    document.addEventListener("keydown", handler);
    return () => document.removeEventListener("keydown", handler);
  }, [handler]);
}
