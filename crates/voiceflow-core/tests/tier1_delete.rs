use voiceflow_core::editing::corrections::resolve_all_tier1;

struct TestCase {
    input: &'static str,
    expected: &'static str,
}

#[test]
fn test_delete_last_word() {
    let cases = vec![
        TestCase { input: "Hello beautiful delete last word.", expected: "Hello." },
        TestCase { input: "Hello beautiful. Delete last word.", expected: "Hello." },
        TestCase { input: "Project launch tomorrow delete previous word.", expected: "Project launch." },
        TestCase { input: "The meeting is scheduled tomorrow morning. Delete last word.", expected: "The meeting is scheduled tomorrow." },
        
        // Manual user tests
        TestCase { input: "Hello beautiful, delete the last word.", expected: "Hello," },
        TestCase { input: "Hello beautiful delete the last word", expected: "Hello" },
        TestCase { input: "Project launch tomorrow. Delete previous word.", expected: "Project launch." },
        TestCase { input: "The meeting is scheduled tomorrow morning. Delete last word.", expected: "The meeting is scheduled tomorrow." },
        TestCase { input: "Hello beautiful. Could you delete the last word?", expected: "Hello." },
        TestCase { input: "Hello beautiful. Please delete the last word.", expected: "Hello." },
    ];

    for case in cases {
        assert_eq!(resolve_all_tier1(case.input), case.expected, "Failed on input: {}", case.input);
    }
}

#[test]
fn test_delete_last_sentence() {
    let cases = vec![
        TestCase { input: "Hello beautiful. I am here. Delete last sentence.", expected: "Hello beautiful." },
        TestCase { input: "Project launch tomorrow. We are ready! Delete previous sentence.", expected: "Project launch tomorrow." },
        TestCase { input: "The meeting is scheduled tomorrow morning. We will discuss the budget. Delete last sentence", expected: "The meeting is scheduled tomorrow morning." },
        TestCase { input: "First. Second! Third? delete previous sentence", expected: "First. Second!" },
        
        // Manual user tests
        TestCase { input: "Hello beautiful. I am here. Could you delete the last sentence?", expected: "Hello beautiful." },
        TestCase { input: "Hello beautiful. I am here. Please delete the last sentence.", expected: "Hello beautiful." },
        TestCase { input: "One. Two. Delete last sentence. Three.", expected: "One. Three." }, // wait, testing inline command
    ];

    for case in cases {
        assert_eq!(resolve_all_tier1(case.input), case.expected, "Failed on input: {}", case.input);
    }
}
