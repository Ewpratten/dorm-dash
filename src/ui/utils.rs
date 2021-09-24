
/// Prepends invisible chars to the start of each line in a block of text
/// to prevent the text from being trimmed by the renderer
pub fn pre_pad_raw_text(text: &str) -> String {
    format!("\u{2800}{}", text)
        .trim()
        .replace("\n", "\n\u{2800}")
}
