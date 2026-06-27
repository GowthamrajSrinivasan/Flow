use voiceflow_core::formatting::streaming::{WindowBuilder, WindowPolicy, MergeEngine};
use voiceflow_core::formatting::context::FormattingContext;
use voiceflow_core::formatting::formatter::format;

#[test]
fn test_window_builder_fixed() {
    let previous = "Let's meet at 3 p";
    let delta = "m tomorrow";
    
    let builder = WindowBuilder::new(WindowPolicy::Fixed(5));
    let (start_idx, window) = builder.build_window(previous, delta);
    
    assert_eq!(window, "Let's meet at 3 p m tomorrow");
    assert_eq!(start_idx, 0); // 5 words is the whole previous buffer
}

#[test]
fn test_window_builder_fixed_large_buffer() {
    let previous = "This is a very long transcript buffer that has many words and we are at the end of it";
    let delta = "now";
    
    // Last 5 words: "at the end of it"
    let builder = WindowBuilder::new(WindowPolicy::Fixed(5));
    let (start_idx, window) = builder.build_window(previous, delta);
    
    assert_eq!(window, "at the end of it now");
    assert_eq!(&previous[..start_idx], "This is a very long transcript buffer that has many words and we are ");
}

#[test]
fn test_merge_engine() {
    let previous = "Let's meet at 3 p";
    let window_start = 0;
    let formatted_window = "Let's meet at 3 PM tomorrow.";
    
    let merger = MergeEngine::new();
    let final_buffer = merger.merge(previous, formatted_window, window_start);
    
    assert_eq!(final_buffer, "Let's meet at 3 PM tomorrow.");
}

#[test]
fn test_merge_engine_large_buffer() {
    let previous = "This is a very long transcript buffer that has many words and we are at the end of it";
    let start_idx = previous.find("at the end of it").unwrap();
    
    let formatted_window = "at the end of it now.";
    
    let merger = MergeEngine::new();
    let final_buffer = merger.merge(previous, formatted_window, start_idx);
    
    assert_eq!(final_buffer, "This is a very long transcript buffer that has many words and we are at the end of it now.");
}

#[test]
fn test_full_streaming_pipeline_email() {
    let previous = "My email is john dot doe at gmail";
    let delta = "dot com period";
    
    let builder = WindowBuilder::new(WindowPolicy::Fixed(20));
    let (start_idx, window) = builder.build_window(previous, delta);
    
    // Pass the window through the formatter
    let ctx = FormattingContext::default();
    let formatted_window = format(&window, &ctx);
    
    let merger = MergeEngine::new();
    let final_buffer = merger.merge(previous, &formatted_window, start_idx);
    
    assert_eq!(final_buffer, "My email is john.doe@gmail.com.");
}

#[test]
fn test_full_streaming_pipeline_url() {
    let previous = "Check out github dot";
    let delta = "com slash repos";
    
    let builder = WindowBuilder::new(WindowPolicy::Fixed(20));
    let (start_idx, window) = builder.build_window(previous, delta);
    
    let ctx = FormattingContext::default();
    let formatted_window = format(&window, &ctx);
    
    let merger = MergeEngine::new();
    let final_buffer = merger.merge(previous, &formatted_window, start_idx);
    
    assert_eq!(final_buffer, "Check out github.com/repos");
}

#[test]
fn test_full_streaming_pipeline_developer() {
    let previous = "Please create a snake case deployment";
    let delta = "manager";
    
    let builder = WindowBuilder::new(WindowPolicy::Fixed(20));
    let (start_idx, window) = builder.build_window(previous, delta);
    
    let mut ctx = FormattingContext::default();
    ctx.mode = voiceflow_core::pipeline::request::FormattingMode::Developer;
    
    let formatted_window = format(&window, &ctx);
    
    let merger = MergeEngine::new();
    let final_buffer = merger.merge(previous, &formatted_window, start_idx);
    
    assert_eq!(final_buffer, "Please create a deployment_manager");
}
