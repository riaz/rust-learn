fn answer() -> i32{
    42
}

#[test]
fn check_answer_availability() {
    assert_eq!(answer(), 42);
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grep_rust::find_matches("loren ipsum\n sit amen", "loren", &mut result);
    assert_eq!(result, b"loren ipsum\n");
}
