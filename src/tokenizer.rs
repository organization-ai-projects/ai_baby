pub type Word = String;

// Tokenisation volontairement simple et marrante.
pub fn tokenize(text: &str) -> Vec<Word> {
    let lowered = text.to_lowercase();
    lowered
        .split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '_')
        .filter(|w| !w.trim().is_empty())
        .map(|w| w.to_string())
        .collect()
}
