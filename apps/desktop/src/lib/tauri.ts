const isTauri = () => {
  return (
    typeof window !== "undefined" &&
    "__TAURI_INTERNALS__" in window &&
    "__TAURI_IPC__" in window
  );
};

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) {
    try {
      const response = await fetch("http://127.0.0.1:1421/ipc", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ cmd, args }),
      });
      if (!response.ok) {
        throw new Error(`IPC HTTP error: ${response.status}`);
      }
      return await response.json();
    } catch (e) {
      console.warn("Tauri IPC and HTTP fallback failed. Returning mock.", e);
      return `[mock] ${cmd} called with ${JSON.stringify(args)}` as unknown as T;
    }
  }
  const { invoke: tauriInvoke } = await import("@tauri-apps/api/core");
  return tauriInvoke<T>(cmd, args);
}
