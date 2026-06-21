use voiceflow_core::editing::context::RewriteContext;
use voiceflow_core::editing::corrections::resolve_all_tier1_with_context;

#[derive(Debug)]
struct StreamingTestCase {
    chunks: Vec<&'static str>,
    expected: &'static str,
}

#[test]
fn test_streaming_corrections() {
    let cases = vec![
        StreamingTestCase {
            chunks: vec!["Meet me at 3 PM no", " wait 4 PM"],
            expected: "Meet me at 4 PM",
        },
        StreamingTestCase {
            chunks: vec!["Schedule it for Monday", " actually Tuesday"],
            expected: "Schedule it for Tuesday",
        },
    ];

    for case in cases {
        let mut context = RewriteContext::new();
        let mut final_output = String::new();
        
        for chunk in &case.chunks {
            final_output.push_str(chunk);
            final_output = resolve_all_tier1_with_context(&final_output, &mut context);
        }
        
        // At the end, any pending corrections that never resolved shouldn't break the final string
        // but for now, we just compare the final output
        assert_eq!(final_output, case.expected, "Failed on chunks: {:?}", case);
    }
}
