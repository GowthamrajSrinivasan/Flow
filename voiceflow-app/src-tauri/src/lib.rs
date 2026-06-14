use tauri::Emitter;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use global_hotkey::GlobalHotKeyEvent;
use voiceflow_core::hotkey::VoiceFlowHotKeyManager;
use voiceflow_core::audio_capture::AudioCapture;
use voiceflow_core::stt::{SpeechRecognizer, WhisperCppRecognizer};
use voiceflow_core::pipeline::vocabulary::{VocabularyEngine, VocabularyItem};
use voiceflow_core::pipeline::formatting::FormattingEngine;
use voiceflow_core::injection::get_injector;
use tauri::Manager;

#[tauri::command]
fn stop_listening(stop_tx: tauri::State<std::sync::mpsc::Sender<()>>) {
    let _ = stop_tx.send(());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![stop_listening])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let window = app.get_webview_window("main").expect("no main window");
            
            // Initialize Hotkey Manager
            let _hotkey_manager = VoiceFlowHotKeyManager::new().expect("Failed to init hotkey manager");
            app.manage(_hotkey_manager);
            
            // UI Stop Channel
            let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();
            app.manage(stop_tx);

            // Initialize Engines
            let model_path = "/Users/gowthamrajsrinivasan/Documents/Projects/Flow/models/ggml-base.en.bin";
            let whisper_recognizer = Arc::new(Mutex::new(
                WhisperCppRecognizer::new(model_path).expect("Failed to load Whisper model")
            ));

            let receiver = GlobalHotKeyEvent::receiver();

            thread::spawn(move || {
                let vocab_items = vec![
                    VocabularyItem {
                        canonical: "Requill".to_string(),
                        aliases: vec!["Requel".to_string(), "Requil".to_string(), "Re Quill".to_string()],
                    }
                ];
                let vocab_engine = VocabularyEngine::new(vocab_items);
                let format_engine = FormattingEngine::new();
                let mut injector = get_injector().expect("Failed to initialize text injector");

                let mut is_recording = false;
                let mut audio_capture: Option<AudioCapture> = None;
                let mut partial_display_text = String::new();
                let mut last_partial_time = std::time::Instant::now();

                loop {
                    let mut should_toggle = false;

                    if let Ok(event) = receiver.try_recv() {
                        if event.state == global_hotkey::HotKeyState::Pressed {
                            should_toggle = true;
                        }
                    }

                    if let Ok(_) = stop_rx.try_recv() {
                        if is_recording {
                            should_toggle = true;
                        }
                    }

                    if should_toggle {
                        is_recording = !is_recording;

                        if is_recording {
                            // Show overlay BEFORE emitting, but don't steal focus
                            let _ = window.show();
                            let _ = app_handle.emit("ListeningStarted", ());
                            partial_display_text.clear();
                            last_partial_time = std::time::Instant::now();
                            let mut recognizer = whisper_recognizer.lock().unwrap();
                            recognizer.start_stream();
                            
                            match AudioCapture::new() {
                                Ok(capture) => {
                                    audio_capture = Some(capture);
                                }
                                Err(e) => {
                                    let _ = app_handle.emit("ErrorOccurred", format!("Mic error: {}", e));
                                    is_recording = false;
                                }
                            }
                        } else {
                            let _ = app_handle.emit("ListeningStopped", ());
                            
                            if let Some(mut capture) = audio_capture.take() {
                                let audio_data = capture.read_audio();
                                
                                let mut recognizer = whisper_recognizer.lock().unwrap();
                                recognizer.process_audio(&audio_data);
                                let mut text = recognizer.final_result();
                                
                                if !text.is_empty() {
                                    text = vocab_engine.apply(&text);
                                    text = format_engine.apply(&text);
                                    
                                    let _ = app_handle.emit("FinalTranscript", text.clone());
                                    
                                    let _ = app_handle.emit("InjectionStarted", ());
                                    
                                    // Small delay so the UI can hide first, restoring focus
                                    thread::sleep(Duration::from_millis(300));
                                    
                                    match injector.inject(&text) {
                                        Ok(_) => {
                                            let _ = app_handle.emit("InjectionCompleted", ());
                                        }
                                        Err(e) => {
                                            let _ = app_handle.emit("ErrorOccurred", format!("Injection error: {}", e));
                                        }
                                    }
                                } else {
                                    let _ = app_handle.emit("ErrorOccurred", "No speech detected");
                                }
                                
                                // Hide overlay after injection
                                thread::sleep(Duration::from_millis(1500));
                                let _ = window.hide();
                            }
                        }
                    }

                    if is_recording {
                        if let Some(capture) = audio_capture.as_mut() {
                            let new_audio = capture.read_audio();
                            if !new_audio.is_empty() {
                                let mut recognizer = whisper_recognizer.lock().unwrap();
                                recognizer.process_audio(&new_audio);
                            }
                        }

                        // 3 seconds gives Whisper enough context to produce real words
                        if last_partial_time.elapsed() >= Duration::from_millis(3000) {
                            last_partial_time = std::time::Instant::now();
                            let mut recognizer = whisper_recognizer.lock().unwrap();
                            if let Some(chunk_text) = recognizer.partial_result() {
                                if !chunk_text.is_empty() {
                                    if !partial_display_text.is_empty() {
                                        partial_display_text.push(' ');
                                    }
                                    partial_display_text.push_str(&chunk_text);
                                    let _ = app_handle.emit("PartialTranscript", partial_display_text.clone());
                                }
                            }
                        }
                    }

                    thread::sleep(Duration::from_millis(20));
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

