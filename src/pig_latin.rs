// first -> irst-fay
// apple -> apple-hay
pub fn run() {
  if let Some(res) = form_pig_latin(&String::from("apple")) {
    println!("{}", res)
  } else {
    eprintln!("Invalid entry")
  }
}

fn form_pig_latin(word: &str) -> Option<String> {
  let mut chars = word.chars();
  let first = chars.next()?;

  if !first.is_alphabetic() {
    return None;
  }

  Some(if is_vowel(first) {
    format!("{}-hay", word)
  } else {
    let rest: String = chars.collect();
    format!("{}-{}ay", rest, first)
  })
}

fn is_vowel(char: char) -> bool {
  matches!(
    char,
    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'
  )
}
