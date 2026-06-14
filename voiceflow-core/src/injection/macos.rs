use crate::injection::TextInjector;
use enigo::{Enigo, Keyboard, Settings};

pub struct MacOsTextInjector {
    enigo: Enigo,
}

impl MacOsTextInjector {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let enigo = Enigo::new(&Settings::default())?;
        Ok(Self { enigo })
    }
}

impl TextInjector for MacOsTextInjector {
    fn inject(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        // We use enigo to type the string sequence
        self.enigo.text(text)?;
        Ok(())
    }
}
