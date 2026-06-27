use std::fs;
use std::path::Path;
use voiceflow_core::formatting::context::FormattingContext;
use voiceflow_core::formatting::formatter::format;

#[test]
fn test_golden_formatting() {
    let base_path = Path::new("tests/golden/formatting");
    if !base_path.exists() {
        return;
    }

    for entry in fs::read_dir(base_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            let input_path = path.join("input.txt");
            let expected_path = path.join("expected.txt");

            if input_path.exists() && expected_path.exists() {
                let input = fs::read_to_string(&input_path).unwrap().trim().to_string();
                let expected = fs::read_to_string(&expected_path).unwrap().trim().to_string();

                let ctx = FormattingContext::default();
                let result = format(&input, &ctx);

                assert_eq!(
                    result, expected,
                    "Golden test failed for directory: {:?}",
                    path.file_name().unwrap()
                );
            }
        }
    }
}
