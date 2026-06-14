use clap::Parser;
use std::time::Instant;
use std::path::Path;
use sysinfo::System;
use hound;

use voiceflow_core::stt::whisper_cpp::WhisperCppRecognizer;
use voiceflow_core::stt::parakeet::ParakeetRecognizer;
use voiceflow_core::stt::SpeechRecognizer;
use voiceflow_core::pipeline::vocabulary::{VocabularyEngine, VocabularyItem};
use voiceflow_core::pipeline::formatting::FormattingEngine;
use voiceflow_shared::metrics::SessionMetrics;
use voiceflow_shared::wer::calculate_wer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    benchmark: bool,
    #[arg(short, long, default_value = "models/ggml-base.en.bin")]
    model: String,
    #[arg(short, long, default_value = "whisper")]
    engine: String,
    #[arg(short, long)]
    inject: Option<String>,
}

fn load_audio(path: &str) -> Vec<f32> {
    let mut reader = hound::WavReader::open(path).expect("Failed to open audio file");
    let spec = reader.spec();
    
    if spec.sample_format == hound::SampleFormat::Float {
        reader.samples::<f32>().map(|s| s.unwrap()).collect()
    } else if spec.sample_format == hound::SampleFormat::Int && spec.bits_per_sample == 16 {
        reader.samples::<i16>().map(|s| s.unwrap() as f32 / 32768.0).collect()
    } else {
        panic!("Unsupported audio format");
    }
}

fn run_test_set(
    name: &str,
    model_path: &str,
    audio_path: &str,
    expected_text: &str,
    engine: &str,
) {
    println!("\n=== Test Set: {} ===", name);
    println!("Engine: {}", engine);
    println!("Model: {}", model_path);
    println!("Audio: {}", audio_path);
    println!("Expected: '{}'", expected_text);

    let mut metrics = SessionMetrics::new("benchmark".to_string());
    let mut sys = System::new_all();
    
    let load_start = Instant::now();
    
    let mut recognizer: Box<dyn SpeechRecognizer> = if engine == "parakeet" {
        Box::new(ParakeetRecognizer::new(model_path).expect("Failed to initialize parakeet recognizer"))
    } else {
        Box::new(WhisperCppRecognizer::new(model_path).expect("Failed to initialize whisper recognizer"))
    };

    metrics.model_load_ms = Some(load_start.elapsed().as_millis() as u64);
    
    if let Ok(metadata) = std::fs::metadata(model_path) {
        let size_mb = metadata.len() / (1024 * 1024);
        metrics.model_size_mb = Some(size_mb);
        metrics.download_size_mb = Some(size_mb);
    }
    
    let audio_data = load_audio(audio_path);
    
    sys.refresh_all();
    let inference_start = Instant::now();
    
    recognizer.start_stream();
    recognizer.process_audio(&audio_data);
    let actual_text = recognizer.final_result();
    
    let inference_time = inference_start.elapsed().as_millis() as u64;
    
    metrics.first_partial_ms = Some(150); // MOCK: Whisper.cpp needs specific callback for this
    metrics.final_transcript_ms = Some(inference_time);
    
    sys.refresh_all();
    if let Ok(pid) = sysinfo::get_current_pid() {
        if let Some(process) = sys.process(pid) {
            metrics.memory_usage_mb = Some(process.memory() / (1024 * 1024));
        } else {
            metrics.memory_usage_mb = Some(0);
        }
    }
    metrics.cpu_usage_percent = Some(sys.global_cpu_usage());
    metrics.complete();

    // Setup Pipeline
    let vocab_items = vec![
        VocabularyItem {
            canonical: "Requill".to_string(),
            aliases: vec!["Requel".to_string(), "Requil".to_string(), "Re Quill".to_string(), "Req will".to_string(), "ReqL".to_string()],
        },
        VocabularyItem {
            canonical: "Genkit".to_string(),
            aliases: vec!["Gen kit".to_string(), "Jinkit".to_string(), "Gankit".to_string()],
        },
        VocabularyItem {
            canonical: "Firebase".to_string(),
            aliases: vec!["Fire base".to_string()],
        },
    ];
    let vocab_engine = VocabularyEngine::new(vocab_items);
    let format_engine = FormattingEngine::new();

    // Run Pipeline
    let vocab_corrected = vocab_engine.apply(&actual_text);
    let final_corrected_text = format_engine.apply(&vocab_corrected);

    let raw_wer = calculate_wer(&expected_text.to_lowercase(), &actual_text.to_lowercase());
    let corrected_wer = calculate_wer(&expected_text.to_lowercase(), &final_corrected_text.to_lowercase());

    println!("--- Benchmark Results ---");
    println!("Load Time: {} ms", metrics.model_load_ms.unwrap_or(0));
    println!("Model Size: {} MB", metrics.model_size_mb.unwrap_or(0));
    println!("First Partial Latency: {} ms (mock)", metrics.first_partial_ms.unwrap_or(0));
    println!("Final Transcript Latency: {} ms", metrics.final_transcript_ms.unwrap_or(0));
    println!("RAM Usage: {} MB", metrics.memory_usage_mb.unwrap_or(0));
    println!("CPU Usage: {:.2}%", metrics.cpu_usage_percent.unwrap_or(0.0));
    println!("Raw Transcript: '{}'", actual_text);
    println!("Raw WER: {:.2}%", raw_wer * 100.0);
    println!("Corrected Transcript: '{}'", final_corrected_text);
    println!("Corrected WER: {:.2}%", corrected_wer * 100.0);
}

fn main() {
    let args = Args::parse();

    if let Some(text) = args.inject {
        println!("Switch to the target application. Injecting in 3 seconds...");
        std::thread::sleep(std::time::Duration::from_secs(3));
        let mut injector = voiceflow_core::injection::get_injector().expect("Failed to get injector");
        injector.inject(&text).expect("Failed to inject text");
        println!("Injection complete!");
        return;
    }

    if args.benchmark {
        println!("--- VoiceFlow STT Benchmark Suite ---");
        
        let long_text = "Hello John, how are you today? Requill integrates with Firebase and Genkit. ".repeat(30).trim().to_string();

        let test_sets = vec![
            ("Short Dictation", "test_audio/short.wav", "Hello John, how are you today?"),
            ("Technical Vocabulary", "test_audio/tech.wav", "Requill integrates with Firebase and Genkit."),
            ("Meeting Dictation", "test_audio/meeting.wav", "Today we reviewed the Firebase migration plan and discussed Genkit integration for Requill."),
            ("Long Dictation", "test_audio/long.wav", &long_text),
        ];
        
        for (name, path, expected) in test_sets {
            if Path::new(path).exists() {
                run_test_set(name, &args.model, path, expected, &args.engine);
            } else {
                println!("\nSkipping {} (audio file missing: {})", name, path);
            }
        }
    } else {
        println!("Please run with --benchmark or --inject flag");
    }
}
