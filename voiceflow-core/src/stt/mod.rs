pub mod whisper_cpp;
pub mod parakeet;

pub use whisper_cpp::WhisperCppRecognizer;
pub use parakeet::ParakeetRecognizer;

pub trait SpeechRecognizer {
    fn start_stream(&mut self);
    fn process_audio(&mut self, audio: &[f32]);
    fn partial_result(&self) -> Option<String>;
    fn final_result(&mut self) -> String;
}
