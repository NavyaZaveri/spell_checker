# spelling_corrector 

A simple command line utility to correct typos. Based on Peter Norvig's excellent [article](http://norvig.com/spell-correct.html).
<br>

Example usage:
 ```
  use spelling_corrector::corrector;
  let correct = corrector::SimpleCorrector::default(); 
  let words = correct.correct_sentence("the architexture is inconcievable");
  assert_eq!(words, "the architecture is inconceivable")
```
