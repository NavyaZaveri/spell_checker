pub mod corrector;

#[test]
fn test_corrector() {
    let correct = corrector::SimpleCorrector::default();
    let word = correct.correct("construktion");
    assert_eq!(word.unwrap(), "construction")
}