use failure::Fallible;
use std::io;

pub fn get_words(amount: usize) -> Fallible<Vec<String>> {
  // println!("Please enter the words, separated by a space");

  let mut input = String::new();
  let words = loop {
    io::stdin().read_line(&mut input)?;

    let words =
      input.trim_end_matches(|c| char::is_control(c) || char::is_whitespace(c) || c == '\n');
    let words = words
      .split_ascii_whitespace()
      .map(|w| w.into())
      .collect::<Vec<_>>();
    //allow any length if amount is 0
    if amount == 0 {
      break words;
    }
    if words.len() > amount {
      println!(
        "Too many words: {}, pls try again with only {} words",
        words.len(),
        amount
      );
      input.clear()
    } else if words.len() < amount {
      println!(
        "Too few words: {}, pls try again with only {} words",
        words.len(),
        amount
      );
      input.clear()
    } else {
      break words;
    }
  };
  Ok(words)
}
