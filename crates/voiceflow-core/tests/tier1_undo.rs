use voiceflow_core::editing::corrections::resolve_all_tier1;

struct TestCase {
    input: &'static str,
    expected: &'static str,
}

#[test]
fn test_undo_commands() {
    let cases = vec![
        TestCase { input: "Create a meeting tomorrow. Never mind.", expected: "" },
        TestCase { input: "Call Rahul. Cancel that.", expected: "" },
        TestCase { input: "Book tickets. Ignore that.", expected: "" },
        TestCase { input: "Send report to team. Scratch that.", expected: "" },
        TestCase { input: "Hello John. Send report. Never mind.", expected: "Hello John." },
        TestCase { input: "Hello John. Send report. Never mind. How are you?", expected: "Hello John. How are you?" },
        TestCase { input: "Call Rahul cancel that", expected: "" },
        TestCase { input: "Hello John call Rahul cancel that how are you", expected: "Hello John call Rahul cancel that how are you" }, // Not isolated
        TestCase { input: "Cancel that.", expected: "" },
        
        // Strict boundary tests
        TestCase { input: "Never mind if someone says you are not good enough in your life.", expected: "Never mind if someone says you are not good enough in your life." },
        TestCase { input: "Never mind the noise outside.", expected: "Never mind the noise outside." },
        TestCase { input: "Never mind what others think.", expected: "Never mind what others think." },
        TestCase { input: "Never mind if the deployment fails initially.", expected: "Never mind if the deployment fails initially." },
        TestCase { input: "Create a meeting tomorrow. Please cancel that.", expected: "" },
        TestCase { input: "Call Rahul. Can you cancel that?", expected: "" },
    ];

    for case in cases {
        assert_eq!(resolve_all_tier1(case.input), case.expected, "Failed on input: {}", case.input);
    }
}
