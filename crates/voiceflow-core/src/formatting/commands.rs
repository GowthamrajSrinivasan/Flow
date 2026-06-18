#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    Formatting,
    Application,
    System,
}

pub struct VoiceCommand {
    pub raw_text: String,
    pub command_type: CommandType,
}

impl VoiceCommand {
    pub fn parse(text: &str) -> Option<Self> {
        let lower = text.to_lowercase();
        
        // Example formatting command
        if lower == "new paragraph" {
            return Some(Self {
                raw_text: text.to_string(),
                command_type: CommandType::Formatting,
            });
        }
        
        // Future placeholders
        if lower.starts_with("open ") {
            return Some(Self {
                raw_text: text.to_string(),
                command_type: CommandType::Application,
            });
        }
        
        None
    }
}
