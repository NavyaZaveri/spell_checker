//! # spelling-corrector
//!
//! A simple command line utility to correct typos. Based on this Peter Norvig's excellent [article](http://norvig.com/spell-correct.html).
//!
//! Example usage:
//! ```
//!  use spelling_corrector::corrector;
//!  let correct = corrector::SimpleCorrector::default();
//!  let words = correct.correct_sentence("the architexture is inconcievable");
//!  assert_eq!(words, "the architecture is inconceivable")
//! ```


pub mod corrector;


#[test]
fn test_corrector() {
    let correct = corrector::SimpleCorrector::default();
    let word = correct.correct("construktion");
    assert_eq!(word.unwrap(), "construction")
}