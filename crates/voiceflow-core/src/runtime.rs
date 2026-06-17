use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::events::VoiceFlowEvent;

use voiceflow_audio::AudioCapture;
use voiceflow_vad::VadEngine;
use voiceflow_stt::{SpeechRecognizer, WhisperCppRecognizer};

pub trait AudioProvider {
    fn read_audio(&mut self) -> Vec<f32>;
}
impl AudioProvider for AudioCapture {
    fn read_audio(&mut self) -> Vec<f32> {
        self.read_audio()
    }
}

pub trait VadProcessor {
    fn process_audio(&mut self, audio_chunk: &[f32]) -> bool;
}
impl VadProcessor for VadEngine {
    fn process_audio(&mut self, audio_chunk: &[f32]) -> bool {
        self.process_audio(audio_chunk)
    }
}

use std::fs;
use directories::ProjectDirs;
use std::io::{Read, Write};

fn ensure_model_downloaded(tx: &std::sync::mpsc::Sender<VoiceFlowEvent>) -> Result<String, String> {
    let proj_dirs = ProjectDirs::from("com", "VoiceFlow", "VoiceFlow")
        .ok_or("Could not determine local app data directory")?;
    
    let model_dir = proj_dirs.data_local_dir().join("models");
    if !model_dir.exists() {
        fs::create_dir_all(&model_dir).map_err(|e| format!("Failed to create model directory: {}", e))?;
    }

    let model_path = model_dir.join("ggml-base.en.bin");
    if model_path.exists() {
        return Ok(model_path.to_string_lossy().to_string());
    }

    let _ = tx.send(VoiceFlowEvent::ModelDownloading(0));
    
    // Download logic
    let url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin";
    let response = ureq::get(url).call().map_err(|e| format!("Download request failed: {}", e))?;
    
    let total_size: u64 = response.header("Content-Length")
        .and_then(|s| s.parse().ok())
        .unwrap_or(148_000_000); // Base.en is ~141MB

    let mut reader = response.into_reader();
    let mut file = fs::File::create(&model_path).map_err(|e| format!("Failed to create model file: {}", e))?;
    
    let mut buffer = [0; 65536]; // 64KB chunks
    let mut downloaded: u64 = 0;
    let mut last_percent = 0;

    loop {
        let bytes_read = reader.read(&mut buffer).map_err(|e| format!("Error reading download stream: {}", e))?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read]).map_err(|e| format!("Error writing to file: {}", e))?;
        
        downloaded += bytes_read as u64;
        let percent = ((downloaded as f64 / total_size as f64) * 100.0) as u8;
        
        if percent > last_percent {
            last_percent = percent;
            let _ = tx.send(VoiceFlowEvent::ModelDownloading(percent));
        }
    }

    let _ = tx.send(VoiceFlowEvent::ModelDownloadComplete);

    Ok(model_path.to_string_lossy().to_string())
}

#[derive(Debug, Clone, Copy)]
pub enum RuntimeProfile {
    DesktopMac,
    DesktopWindows,
    MobileIos,
    MobileAndroid,
}

pub struct VoiceFlow {
    event_sender: Option<Sender<VoiceFlowEvent>>,
    is_listening: Arc<Mutex<bool>>,
}

impl VoiceFlow {
    pub fn new(_profile: RuntimeProfile) -> Self {
        Self {
            event_sender: None,
            is_listening: Arc::new(Mutex::new(false)),
        }
    }

    pub fn subscribe(&mut self) -> Receiver<VoiceFlowEvent> {
        let (tx, rx) = channel();
        self.event_sender = Some(tx);
        rx
    }

    pub fn start_listening(&self) {
        let mut listening = self.is_listening.lock().unwrap();
        if *listening {
            return;
        }
        *listening = true;

        let is_listening_clone = self.is_listening.clone();
        
        let tx = match &self.event_sender {
            Some(sender) => sender.clone(),
            None => return, // No one is listening to events
        };

        // Don't block the UI thread during init/download
        thread::spawn(move || {
            // 0. Auto-download model if missing
            let model_path = match ensure_model_downloaded(&tx) {
                Ok(path) => path,
                Err(e) => {
                    let _ = tx.send(VoiceFlowEvent::Error(format!("Model downloader failed: {}", e)));
                    *is_listening_clone.lock().unwrap() = false;
                    let _ = tx.send(VoiceFlowEvent::ListeningStopped);
                    return;
                }
            };

            // Inform UI we are actually listening now
            let _ = tx.send(VoiceFlowEvent::ListeningStarted);

            // 1. Initialize Mic
            let mut audio_capture = match AudioCapture::new() {
                Ok(cap) => cap,
                Err(e) => {
                    let _ = tx.send(VoiceFlowEvent::Error(format!("Microphone error: {}", e)));
                    *is_listening_clone.lock().unwrap() = false;
                    let _ = tx.send(VoiceFlowEvent::ListeningStopped);
                    return;
                }
            };

            // 2. Initialize VAD
            let mut vad_engine = match VadEngine::new() {
                Ok(vad) => vad,
                Err(e) => {
                    let _ = tx.send(VoiceFlowEvent::Error(format!("VAD error: {}", e)));
                    *is_listening_clone.lock().unwrap() = false;
                    let _ = tx.send(VoiceFlowEvent::ListeningStopped);
                    return;
                }
            };

            // 3. Initialize STT
            let mut whisper = match WhisperCppRecognizer::new(&model_path) {
                Ok(w) => w,
                Err(e) => {
                    let _ = tx.send(VoiceFlowEvent::Error(format!("Whisper load error: {}", e)));
                    *is_listening_clone.lock().unwrap() = false;
                    let _ = tx.send(VoiceFlowEvent::ListeningStopped);
                    return;
                }
            };

            whisper.start_stream();

            run_listening_loop(
                &mut audio_capture,
                &mut vad_engine,
                &mut whisper,
                is_listening_clone.clone(),
                tx.clone(),
            );

            // Ensure we emit the stopped event when the thread exits
            let _ = tx.send(VoiceFlowEvent::ListeningStopped);
        });
    }

    pub fn stop_listening(&self) {
        let mut listening = self.is_listening.lock().unwrap();
        if !*listening {
            return;
        }
        *listening = false;
        // The background thread will detect this and exit, emitting ListeningStopped
    }

    pub fn is_listening(&self) -> bool {
        *self.is_listening.lock().unwrap()
    }
}

pub fn run_listening_loop<A: AudioProvider, V: VadProcessor, S: SpeechRecognizer>(
    audio_capture: &mut A,
    vad_engine: &mut V,
    whisper: &mut S,
    is_listening_clone: Arc<Mutex<bool>>,
    tx: Sender<VoiceFlowEvent>,
) {
    let mut last_partial_time = Instant::now();
    let mut silence_start: Option<Instant> = None;
    let silence_threshold = Duration::from_millis(1500);
    let mut has_spoken = false;
    
    // Wait a tiny bit for the mic to warm up
    thread::sleep(Duration::from_millis(100));

    loop {
        // Check if we should stop listening (e.g. user pressed hotkey again)
        if !*is_listening_clone.lock().unwrap() {
            break;
        }

        // Read incoming audio chunks
        let chunk = audio_capture.read_audio();
        if !chunk.is_empty() {
            whisper.process_audio(&chunk);
            let is_speech = vad_engine.process_audio(&chunk);
            
            if is_speech {
                has_spoken = true;
                silence_start = None;
            } else {
                // Only start tracking silence to stop IF the user has already spoken
                if has_spoken && silence_start.is_none() {
                    silence_start = Some(Instant::now());
                }
            }
        }

        // Periodically yield partial transcripts
        if last_partial_time.elapsed() > Duration::from_millis(300) {
            last_partial_time = Instant::now();
            if let Some(partial) = whisper.partial_result() {
                if !partial.is_empty() {
                    let _ = tx.send(VoiceFlowEvent::PartialTranscript(partial));
                }
            }
        }

        // If VAD detected silence for > 700ms, finalize!
        if let Some(start) = silence_start {
            if start.elapsed() > silence_threshold {
                let final_text = whisper.final_result();
                if !final_text.is_empty() {
                    let _ = tx.send(VoiceFlowEvent::FinalTranscript(final_text));
                }
                // Stop listening automatically because utterance is complete
                *is_listening_clone.lock().unwrap() = false;
                break;
            }
        }

        thread::sleep(Duration::from_millis(20));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voiceflow_state_machine() {
        let mut engine = VoiceFlow::new(RuntimeProfile::DesktopMac);
        assert!(!engine.is_listening(), "Engine should not be listening on creation");

        let receiver = engine.subscribe();

        // Simulate hotkey press
        engine.start_listening();
        assert!(engine.is_listening(), "Engine should be listening after start");
        
        let event = receiver.try_recv().expect("Should have received an event");
        match event {
            VoiceFlowEvent::ListeningStarted => {},
            _ => panic!("Expected ListeningStarted event"),
        }

        // Simulate hotkey toggle
        engine.stop_listening();
        assert!(!engine.is_listening(), "Engine should not be listening after stop");

        let event = receiver.recv_timeout(std::time::Duration::from_secs(2)).expect("Should have received ListeningStopped event");
        match event {
            VoiceFlowEvent::ListeningStopped | VoiceFlowEvent::Error(_) => {},
            _ => panic!("Expected ListeningStopped or Error event"),
        }
    }

    struct MockAudioProvider {
        chunks: std::vec::IntoIter<Vec<f32>>,
    }
    impl AudioProvider for MockAudioProvider {
        fn read_audio(&mut self) -> Vec<f32> {
            self.chunks.next().unwrap_or_else(Vec::new)
        }
    }

    struct MockVadProcessor {
        results: std::vec::IntoIter<bool>,
    }
    impl VadProcessor for MockVadProcessor {
        fn process_audio(&mut self, _audio_chunk: &[f32]) -> bool {
            self.results.next().unwrap_or(false)
        }
    }

    struct MockSpeechRecognizer;
    impl SpeechRecognizer for MockSpeechRecognizer {
        fn start_stream(&mut self) {}
        fn process_audio(&mut self, _audio: &[f32]) {}
        fn partial_result(&mut self) -> Option<String> {
            None
        }
        fn final_result(&mut self) -> String {
            "Mock final transcript".to_string()
        }
    }

    #[test]
    fn test_pause_stops_transcription() {
        let (tx, rx) = channel();
        let is_listening = Arc::new(Mutex::new(true));

        // We simulate some speech frames, and then silence.
        // run_listening_loop will sleep for 20ms each iteration.
        // 1500ms / 20ms = 75 iterations of silence needed to trigger the pause stop.
        // We will provide a few speech chunks, then 100 silence chunks.
        let mut audio_chunks = Vec::new();
        let mut vad_results = Vec::new();

        // 5 speech chunks
        for _ in 0..5 {
            audio_chunks.push(vec![0.5; 160]);
            vad_results.push(true);
        }

        // 100 silence chunks (which takes ~2000ms loop time)
        for _ in 0..100 {
            audio_chunks.push(vec![0.0; 160]);
            vad_results.push(false);
        }

        let mut audio = MockAudioProvider {
            chunks: audio_chunks.into_iter(),
        };
        let mut vad = MockVadProcessor {
            results: vad_results.into_iter(),
        };
        let mut stt = MockSpeechRecognizer;

        // Run the loop. It should exit on its own after silence threshold is met.
        run_listening_loop(&mut audio, &mut vad, &mut stt, is_listening.clone(), tx);

        // Verify the loop correctly turned off listening
        assert!(!*is_listening.lock().unwrap(), "Listening should be false after pause triggers stop");

        // Collect emitted events
        let mut final_transcript_received = false;
        while let Ok(event) = rx.try_recv() {
            if let VoiceFlowEvent::FinalTranscript(text) = event {
                assert_eq!(text, "Mock final transcript");
                final_transcript_received = true;
            }
        }

        assert!(final_transcript_received, "Expected FinalTranscript event to be sent after pause");
    }
}
