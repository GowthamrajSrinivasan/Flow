use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;

pub struct AudioCapture {
    _stream: cpal::Stream,
    receiver: mpsc::Receiver<f32>,
    channels: u16,
    sample_rate: u32,
}

impl AudioCapture {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("No input device available")?;

        let config: cpal::StreamConfig = device.default_input_config()?.into();
        let channels = config.channels;
        let sample_rate = config.sample_rate.0;
        
        let (sender, receiver) = mpsc::channel();

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    let _ = sender.send(sample);
                }
            },
            move |err| {
                eprintln!("An error occurred on stream: {}", err);
            },
            None, // Timeout
        )?;

        stream.play()?;

        Ok(Self { _stream: stream, receiver, channels, sample_rate })
    }

    pub fn read_audio(&mut self) -> Vec<f32> {
        let mut raw_data = Vec::new();
        while let Ok(sample) = self.receiver.try_recv() {
            raw_data.push(sample);
        }

        if raw_data.is_empty() {
            return vec![];
        }

        // 1. Convert to Mono
        let channels = self.channels as usize;
        let mut mono_data = Vec::with_capacity(raw_data.len() / channels);
        for chunk in raw_data.chunks(channels) {
            let sum: f32 = chunk.iter().sum();
            mono_data.push(sum / channels as f32);
        }

        // 2. Resample to 16000 Hz
        let target_sr = 16000.0;
        let source_sr = self.sample_rate as f32;
        
        if (source_sr - target_sr).abs() < 1.0 {
            return mono_data;
        }

        let ratio = source_sr / target_sr;
        let target_len = (mono_data.len() as f32 / ratio) as usize;
        let mut resampled = Vec::with_capacity(target_len);

        for i in 0..target_len {
            let src_idx = i as f32 * ratio;
            let idx_floor = src_idx.floor() as usize;
            let idx_ceil = std::cmp::min(idx_floor + 1, mono_data.len() - 1);
            let weight = src_idx - idx_floor as f32;

            let sample = mono_data[idx_floor] * (1.0 - weight) + mono_data[idx_ceil] * weight;
            resampled.push(sample);
        }

        resampled
    }
}
