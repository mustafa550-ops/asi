const isTauri = () => typeof window !== "undefined" && "__TAURI__" in window;

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) {
    return `[mock] ${cmd} called with ${JSON.stringify(args)}` as unknown as T;
  }
  const { invoke: tauriInvoke } = await import("@tauri-apps/api/core");
  return tauriInvoke<T>(cmd, args);
}
