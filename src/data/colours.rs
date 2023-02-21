use core::fmt;

struct ColoredText {
    color: colored::Color,
    text: String,
}

impl fmt::Display for ColoredText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colored_text = self.text.color(self.color);
        write!(f, "{}", colored_text)
    }
}
