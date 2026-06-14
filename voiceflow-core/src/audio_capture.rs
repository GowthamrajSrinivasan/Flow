use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;

pub struct AudioCapture {
    _stream: cpal::Stream,
    receiver: mpsc::Receiver<f32>,
}

impl AudioCapture {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("No input device available")?;

        let config: cpal::StreamConfig = device.default_input_config()?.into();
        
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

        Ok(Self { _stream: stream, receiver })
    }

    pub fn read_audio(&mut self) -> Vec<f32> {
        let mut data = Vec::new();
        while let Ok(sample) = self.receiver.try_recv() {
            data.push(sample);
        }
        data
    }
}
