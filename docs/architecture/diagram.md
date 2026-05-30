# ADLER ASI Architecture

```mermaid
graph TD
    UI[React Desktop App] -->|IPC| Tauri[Tauri Core]
    Tauri -->|Internal API| Core[Rust Headless Core]
    Core -->|HTTP| LLM[Ollama Local LLM]
    Core -->|SQLCipher| DB[(SQLite Database)]
    Core -->|WebSocket| Voice[Voice Assistant]
```
