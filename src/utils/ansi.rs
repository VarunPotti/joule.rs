use ansi_term;
#[allow(dead_code)]
pub fn red(sentence: &str) -> String {
    let s: String;

    s = ansi_term::Color::Red.bold().paint(sentence).to_string();

    s
}
#[allow(dead_code)]
pub fn blue(sentence: &str) -> String {
    let s: String;

    s = ansi_term::Color::Blue.bold().paint(sentence).to_string();

    s
}

#[allow(dead_code)]
pub fn green(sentence: &str) -> String {
    let s: String;

    s = ansi_term::Color::Green.bold().paint(sentence).to_string();

    s
}

#[allow(dead_code)]
pub fn yellow(sentence: &str) -> String {
    let s: String;

    s = ansi_term::Color::Yellow.bold().paint(sentence).to_string();

    s
}
#[allow(dead_code)]
pub fn purple(sentence: &str) -> String {
    let s: String;

    s = ansi_term::Color::Purple.bold().paint(sentence).to_string();

    s
}
