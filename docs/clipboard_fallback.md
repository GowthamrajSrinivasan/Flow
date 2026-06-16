# Clipboard Fallback Feature Documentation

This document records the architectural design, implementation details, and deferred configuration planning for the **Clipboard Fallback** behavior. It serves as a reference for re-implementing this feature in Phase 3 once a user preferences configuration interface is built.

## Background & Rationale

On macOS, keyboard event injection via OS-level events (`CGEventPost` used by the `Enigo` library) reports a success status (`Ok(())`) even if no text input field has focus. The OS simply swallows the simulated keystrokes. 

To prevent user speech from being silently lost when dictating without an active cursor, a **Clipboard Fallback** is the standard industry solution. The transcribed text is automatically placed in the clipboard if injection fails or as a general safety net.

In Phase 2, this was implemented as an automatic behavior. It has been temporarily removed to prevent unsolicited clipboard overwrites, and will be re-introduced in Phase 3 as an **opt-in / opt-out** user preference.

---

## Technical Implementation Reference

### 1. Clipboard Copy Helper
The clipboard capability remains defined in [mod.rs](file:///Users/gowthamrajsrinivasan/Documents/Projects/Flow/voiceflow-core/src/injection/mod.rs) and is ready for reuse:

```rust
pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    #[cfg(target_os = "macos")]
    let cmd = "pbcopy";
    #[cfg(target_os = "windows")]
    let cmd = "clip";
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let cmd = "xclip";

    let mut child = Command::new(cmd)
        .stdin(Stdio::piped())
        .spawn()?;
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    }
    
    child.wait()?;
    Ok(())
}
```

### 2. Auto-Copy Pipeline Integration (Phase 3 Re-implementation Plan)

When the settings interface is added, the Tauri backend should load the configuration file (e.g., `config.json`) mapping to the settings interface. The pipeline inside [lib.rs](file:///Users/gowthamrajsrinivasan/Documents/Projects/Flow/voiceflow-app/src-tauri/src/lib.rs) should check this preference:

```rust
// In Phase 3, query the user preference:
// let auto_copy_enabled = load_preference("onInjectionFailure"); // e.g., "copy_to_clipboard" or "notify"

if !text.is_empty() {
    text = vocab_engine.apply(&text);
    text = format_engine.apply(&text);
    
    let mut clipboard_ok = false;
    if auto_copy_enabled {
        clipboard_ok = match voiceflow_core::injection::copy_to_clipboard(&text) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("[DEBUG] failed to copy to clipboard: {:?}", e);
                false
            }
        };
    }
    
    let _ = app_handle.emit("FinalTranscript", text.clone());
    let _ = app_handle.emit("InjectionStarted", ());
    thread::sleep(Duration::from_millis(300));
    
    match injector.inject(&text) {
        Ok(_) => {
            let _ = app_handle.emit("InjectionCompleted", ());
        }
        Err(e) => {
            if auto_copy_enabled && clipboard_ok {
                let _ = app_handle.emit("ErrorOccurred", "Copied to clipboard (No focus / Perm missing)".to_string());
            } else {
                let _ = app_handle.emit("ErrorOccurred", format!("Injection failed: {}", e));
            }
        }
    }
}
```

## Phase 3 Configuration Integration Plan

In Phase 3, we will expose this setting via the General/Injection Settings UI panel:

```json
{
  "injection": {
    "fallback_behavior": "copy_to_clipboard" 
  }
}
```
*   `notify`: Only display error toast/notification.
*   `copy_to_clipboard`: Copy to system clipboard and notify.
*   `none`: Silently fail.
