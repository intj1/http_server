pub fn get_next_word(sentence: &str) -> Option<(&str, &str)>{
   for (i, c) in sentence.chars().enumerate() {
        if c == ' ' {
            return Some((&sentence[..i], &sentence[i + 1..]))
        }
   } 
   None
}