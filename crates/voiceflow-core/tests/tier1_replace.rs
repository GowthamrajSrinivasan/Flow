use voiceflow_core::editing::corrections::resolve_all_tier1;

struct TestCase {
    input: &'static str,
    expected: &'static str,
}

#[test]
fn test_basic_replace() {
    let cases = vec![
        TestCase { input: "Meeting in Bangalore. Replace Bangalore with Mumbai.", expected: "Meeting in Mumbai." },
        TestCase { input: "Replace Rahul with Rajesh because he is here.", expected: "Replace Rahul with Rajesh because he is here." },
        TestCase { input: "Meeting in Chennai tomorrow. Replace Chennai with Bangalore.", expected: "Meeting in Bangalore tomorrow." },
        TestCase { input: "Rahul met Rahul. Replace Rahul with Rajesh.", expected: "Rahul met Rajesh." },
        TestCase { input: "Budget is 5 lakh. Replace budget with forecast.", expected: "Forecast is 5 lakh." },
    ];

    for case in cases {
        assert_eq!(resolve_all_tier1(case.input), case.expected, "Failed on input: {}", case.input);
    }
}

#[test]
fn test_replace_command_vs_natural_language() {
    let cases = vec![
        // --- Should NOT Trigger Replace Command ---
        
        // Quoted Speech
        TestCase {
            input: "Rahul met Rahul, but he said \"I will replace Rahul with Rajesh in the job.\"",
            expected: "Rahul met Rahul, but he said \"I will replace Rahul with Rajesh in the job.\"",
        },
        TestCase {
            input: "Rahul met Rahul, but he said 'I will replace Rahul with Rajesh in the job.'",
            expected: "Rahul met Rahul, but he said 'I will replace Rahul with Rajesh in the job.'",
        },
        TestCase {
            input: "Jay said to vimal, 'Please replace the filters with new ones'",
            expected: "Jay said to vimal, 'Please replace the filters with new ones'",
        },
        TestCase {
            input: "She replied, \"Replace Chennai with Bangalore if the customer asks.\"",
            expected: "She replied, \"Replace Chennai with Bangalore if the customer asks.\"",
        },
        TestCase {
            input: "He shouted, \"Replace the old server with the new one!\"",
            expected: "He shouted, \"Replace the old server with the new one!\"",
        },
        TestCase {
            input: "The trainer said, \"Replace X with Y in the formula.\"",
            expected: "The trainer said, \"Replace X with Y in the formula.\"",
        },

        // Reported Speech
        TestCase {
            input: "Rahul said he would replace Rahul with Rajesh in the document.",
            expected: "Rahul said he would replace Rahul with Rajesh in the document.",
        },
        TestCase {
            input: "Priya explained that we should replace Chennai with Bangalore later.",
            expected: "Priya explained that we should replace Chennai with Bangalore later.",
        },
        TestCase {
            input: "The manager instructed us to replace the old logo with the new one.",
            expected: "The manager instructed us to replace the old logo with the new one.",
        },
        TestCase {
            input: "He mentioned that someone might replace the database with a cache.",
            expected: "He mentioned that someone might replace the database with a cache.",
        },

        // Questions
        TestCase {
            input: "Can you replace Rahul with Rajesh in the report?",
            expected: "Can you replace Rahul with Rajesh in the report?",
        },
        TestCase {
            input: "Should we replace Chennai with Bangalore?",
            expected: "Should we replace Chennai with Bangalore?",
        },
        TestCase {
            input: "Why would anyone replace Rahul with Rajesh?",
            expected: "Why would anyone replace Rahul with Rajesh?",
        },
        TestCase {
            input: "Who said to replace the backend with microservices?",
            expected: "Who said to replace the backend with microservices?",
        },

        // Conditional Statements
        TestCase {
            input: "If needed, replace Rahul with Rajesh.",
            expected: "If needed, replace Rahul with Rajesh.",
        },
        TestCase {
            input: "When the migration starts, replace the server with the backup.",
            expected: "When the migration starts, replace the server with the backup.",
        },
        TestCase {
            input: "Before deployment, replace the config with the production version.",
            expected: "Before deployment, replace the config with the production version.",
        },
        TestCase {
            input: "If the customer insists, replace Chennai with Bangalore.",
            expected: "If the customer insists, replace Chennai with Bangalore.",
        },

        // Examples / Documentation
        TestCase {
            input: "Example: Replace Rahul with Rajesh.",
            expected: "Example: Replace Rahul with Rajesh.",
        },
        TestCase {
            input: "The documentation contains the phrase \"Replace X with Y\".",
            expected: "The documentation contains the phrase \"Replace X with Y\".",
        },
        TestCase {
            input: "In this tutorial, replace username with your actual username.",
            expected: "In this tutorial, replace username with your actual username.",
        },
        TestCase {
            input: "The article explains how to replace one value with another.",
            expected: "The article explains how to replace one value with another.",
        },

        // Code / Technical Text
        TestCase {
            input: "Use replace(\"Rahul\", \"Rajesh\") in the code.",
            expected: "Use replace(\"Rahul\", \"Rajesh\") in the code.",
        },
        TestCase {
            input: "The SQL query replaces Rahul with Rajesh automatically.",
            expected: "The SQL query replaces Rahul with Rajesh automatically.",
        },
        TestCase {
            input: "String.Replace(\"Rahul\", \"Rajesh\")",
            expected: "String.Replace(\"Rahul\", \"Rajesh\")",
        },
        TestCase {
            input: "Search and replace Rahul with Rajesh.",
            expected: "Search and replace Rahul with Rajesh.",
        },

        // Educational Context
        TestCase {
            input: "The teacher asked students to replace Rahul with Rajesh in the sentence.",
            expected: "The teacher asked students to replace Rahul with Rajesh in the sentence.",
        },
        TestCase {
            input: "Grammar exercise: replace the noun with a pronoun.",
            expected: "Grammar exercise: replace the noun with a pronoun.",
        },
        TestCase {
            input: "The exam question says replace X with Y.",
            expected: "The exam question says replace X with Y.",
        },

        // Mixed Punctuation
        TestCase {
            input: "\"Replace Rahul with Rajesh,\" he said.",
            expected: "\"Replace Rahul with Rajesh,\" he said.",
        },
        TestCase {
            input: "'Replace Rahul with Rajesh,' he said.",
            expected: "'Replace Rahul with Rajesh,' he said.",
        },
        TestCase {
            input: "(\"Replace Rahul with Rajesh\")",
            expected: "(\"Replace Rahul with Rajesh\")",
        },
        TestCase {
            input: "[Replace Rahul with Rajesh]",
            expected: "[Replace Rahul with Rajesh]",
        },
        TestCase {
            input: "{Replace Rahul with Rajesh}",
            expected: "{Replace Rahul with Rajesh}",
        },

        // Colon Cases
        TestCase {
            input: "Instructions: replace Rahul with Rajesh.",
            expected: "Instructions: replace Rahul with Rajesh.",
        },
        TestCase {
            input: "Example: replace Chennai with Bangalore.",
            expected: "Example: replace Chennai with Bangalore.",
        },
        TestCase {
            input: "Note: replace localhost with your server name.",
            expected: "Note: replace localhost with your server name.",
        },

        // Markdown / Documentation
        TestCase {
            input: "Step 1: Replace Rahul with Rajesh.",
            expected: "Step 1: Replace Rahul with Rajesh.",
        },
        TestCase {
            input: "- Replace Rahul with Rajesh.",
            expected: "- Replace Rahul with Rajesh.",
        },
        TestCase {
            input: "1. Replace Rahul with Rajesh.",
            expected: "1. Replace Rahul with Rajesh.",
        },

        // Natural Language False Positives
        TestCase {
            input: "Never replace kindness with anger.",
            expected: "Never replace kindness with anger.",
        },
        TestCase {
            input: "You cannot replace experience with theory.",
            expected: "You cannot replace experience with theory.",
        },
        TestCase {
            input: "Nothing can replace hard work.",
            expected: "Nothing can replace hard work.",
        },
        TestCase {
            input: "Technology should not replace human judgment.",
            expected: "Technology should not replace human judgment.",
        },

        // --- Should Trigger Replace Command ---
        TestCase {
            input: "Replace Rahul with Rajesh.",
            expected: "Replace Rahul with Rajesh.", // Since Rahul is not found in before_command
        },
        TestCase {
            input: "Rahul was here. Replace Rahul with Rajesh.",
            expected: "Rajesh was here.", // Since Rahul is found!
        },
        TestCase {
            input: "Chennai weather is good. Replace Chennai with Bangalore.",
            expected: "Bangalore weather is good.",
        },
        TestCase {
            input: "Rahul. Please replace Rahul with Rajesh.",
            expected: "Rajesh.", // "Please replace" doesn't block if we have a word boundary
        },
        TestCase {
            input: "Rahul. Can you replace Rahul with Rajesh?",
            expected: "Rajesh.",
        },
    ];

    for case in cases {
        assert_eq!(resolve_all_tier1(case.input), case.expected, "Failed on input: {}", case.input);
    }
}

#[test]
fn test_replace_last_word() {
    let cases = vec![
        TestCase { input: "Hello Rahul. Replace last word with Rajesh.", expected: "Hello Rajesh." },
        TestCase { input: "Meeting is tomorrow. Replace the last word with Friday.", expected: "Meeting is Friday." },
        TestCase { input: "The budget is 5 lakh. Replace last word to crore.", expected: "The budget is 5 crore." },
        
        // Inline test
        TestCase { input: "Hello Rahul. Replace last word with Rajesh. How are you?", expected: "Hello Rajesh. How are you?" },
        
        // Natural language cases that should NOT trigger
        TestCase { input: "Never replace last word with a bad word.", expected: "Never replace last word with a bad word." }, // Natural language, but wait: "Replace last word with a" matches!
        // Wait, "replace last word with a" matches `replace last word with \w+`! 
        // We need to be careful with natural language cases for "replace last word".
        // Let's test what happens when we use it in a normal sentence.
        TestCase { input: "The teacher said to replace last word with an adjective.", expected: "The teacher said to replace last word with an adjective." },
    ];

    for case in cases {
        assert_eq!(resolve_all_tier1(case.input), case.expected, "Failed on input: {}", case.input);
    }
}
