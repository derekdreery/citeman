use crossref::{CrossRefMessage, Work};

const EXAMPLE_WORK: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/example_work.json"));

#[test]
fn parse_example_work_response() {
    let _: CrossRefMessage<Work> = serde_json::from_str(EXAMPLE_WORK).unwrap();
}