use crate::pipeline::request::RewriteMode;
use crate::pipeline::intent::UserIntent;

pub fn build_system_prompt(intent: UserIntent, mode: RewriteMode) -> String {
    let mut prompt = String::from("You are VoiceFlow, an AI dictation assistant.\n");
    prompt.push_str("Your only goal is to output the final transformed text.\n");
    prompt.push_str("Do not add explanations or conversational filler.\n");
    
    match intent {
        UserIntent::Dictation => {
            prompt.push_str("Output the text exactly as provided.\n");
        },
        UserIntent::Rewrite => {
            match mode {
                RewriteMode::Light => {
                    prompt.push_str("Fix grammar, spelling, and punctuation only. ");
                    prompt.push_str("Do NOT add content, change the meaning, or change the tone. ");
                    prompt.push_str("Output must be structurally similar to the input.\n");
                },
                RewriteMode::Standard => {
                    prompt.push_str("Improve readability and flow while preserving the core meaning.\n");
                },
                RewriteMode::Aggressive => {
                    prompt.push_str("Perform a full AI rewrite. Expand and improve the content aggressively.\n");
                },
                RewriteMode::Off => {}
            }
        },
        UserIntent::Summarize => {
            prompt.push_str("Summarize the following text concisely.\n");
        },
        UserIntent::Translate => {
            prompt.push_str("Translate the following text into the requested language.\n");
        },
        UserIntent::CorrectGrammar => {
            prompt.push_str("Correct the grammar of the following text without changing its meaning.\n");
        }
    }
    
    prompt
}
