import { useEffect, useRef } from "react";

type EventCallback = (payload: string) => void;

const isTauri = () => typeof window !== "undefined" && "__TAURI__" in window;

export function useTauriEvent(event: string, cb: EventCallback) {
  const savedCb = useRef(cb);
  savedCb.current = cb;

  useEffect(() => {
    if (!isTauri()) return;
    let unlisten: (() => void) | undefined;

    (async () => {
      const { listen } = await import("@tauri-apps/api/event");
      unlisten = await listen(event, (e) => {
        savedCb.current(JSON.stringify(e.payload));
      });
    })();

    return () => {
      if (unlisten) unlisten();
    };
  }, [event]);
}
