use voiceflow_core::editing::corrections::resolve_all_tier1;

struct TestCase {
    input: &'static str,
    expected: &'static str,
}

#[test]
fn test_inline_corrections() {
    let cases = vec![
        TestCase { input: "Meet me at 3 PM. No wait 4 PM.", expected: "Meet me at 4 PM." },
        TestCase { input: "Schedule it for Monday. Actually Tuesday.", expected: "Schedule it for Tuesday." },
        TestCase { input: "Budget is 50 thousand. Sorry 60 thousand.", expected: "Budget is 60 thousand." },
        TestCase { input: "Send it to Rahul. Sorry Rajesh.", expected: "Send it to Rajesh." },
        TestCase { input: "Assign this task to Priya. Actually Anitha.", expected: "Assign this task to Anitha." },
        TestCase { input: "Let's meet in Chennai. Actually Bangalore.", expected: "Let's meet in Bangalore." },
        TestCase { input: "Book it for June 10. No June 12.", expected: "Book it for June 12." },
        TestCase { input: "Need 5 licenses. Actually 10.", expected: "Need 10 licenses." },
        TestCase { input: "I need 5 users. Sorry 6 users.", expected: "I need 6 users." },
        
        // Low confidence scenarios (should not trigger replacements)
        TestCase { input: "Sorry for the delay.", expected: "Sorry for the delay." },
        TestCase { input: "Actually this feature is useful.", expected: "Actually this feature is useful." },
        TestCase { input: "Meet at 3 PM. Actually Rahul.", expected: "Meet at 3 PM. Actually Rahul." }, // Low confidence cross-type
    ];

    for case in cases {
        assert_eq!(resolve_all_tier1(case.input), case.expected, "Failed on input: {}", case.input);
    }
}
