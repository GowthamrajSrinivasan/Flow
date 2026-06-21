use voiceflow_core::editing::corrections::resolve_all_tier1;

struct TestCase {
    input: &'static str,
    expected: &'static str,
}

#[test]
fn test_false_start_recovery() {
    let cases = vec![
        TestCase { input: "Let's schedule a meeting. Let's schedule a call with Rahul.", expected: "Let's schedule a call with Rahul." },
        TestCase { input: "The issue is. The issue is in production.", expected: "The issue is in production." },
        TestCase { input: "Please send. Please send the report.", expected: "Please send the report." },
        TestCase { input: "I will be available tomorrow. I will be available next week.", expected: "I will be available next week." },
        TestCase { input: "Let's schedule a meeting let's schedule a call with Rahul.", expected: "Let's schedule a call with Rahul." },
        
        // Should ignore different thoughts
        TestCase { input: "Let's schedule a meeting. Also invite Rahul.", expected: "Let's schedule a meeting. Also invite Rahul." },
    ];

    for case in cases {
        assert_eq!(resolve_all_tier1(case.input), case.expected, "Failed on input: {}", case.input);
    }
}
