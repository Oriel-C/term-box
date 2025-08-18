mod utils;

use super::*;
use utils::*;

#[test]
fn empty() {
    let box_single = TermBox::default().to_string();
    let box_double = TermBox { border_style: BorderStyle::new_double(), ..TermBox::default() }.to_string();
    assert_eq!(box_single, "┌─┐\n└─┘");
    assert_eq!(box_double, "╔═╗\n╚═╝");
}

#[test]
fn empty_styled() {
    let box_ = TermBox {
        border_style: BorderStyle::new_double().with_style(Color::Purple),
        ..TermBox::default()
    }.to_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "empty-styled");
}


#[test]
fn unstyled() {
    let box_ = TermBox {
        border_style: BorderStyle::default(),
        padding: Padding::default(),
        lines: strings![
            "a",
            "few",
            "lines",
            "for testing"
        ]
    }.to_string();

    assert_okay!(lines_same_len(&box_));
    assert_eq!(box_, "┌───────────┐\n│a          │\n│few        │\n│lines      │\n│for testing│\n└───────────┘");
}

#[test]
fn unstyled_with_ansi_text() {
    let box_ = TermBox {
        border_style: BorderStyle::default(),
        padding: Padding::default(),
        lines: strings![
            "uncolored",
            Color::Red.paint("colored!!"),
            AnsiStyle::new().bold().paint("bolded"),
            Color::Blue.bold().paint("both")
        ]
    }.to_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "unstyled-with-ansi-text");
}

#[test]
fn styled() {
    let box_ = TermBox {
        border_style: BorderStyle::new_single().with_style(Color::LightPurple.bold()),
        padding: Padding::default(),
        lines: strings![
            "some",
            "cool",
            "text"
        ]
    }.to_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "styled");
}

#[test]
fn styled_with_ansi_text() {
    let box_ = TermBox {
        border_style: BorderStyle::new_double().with_style(Color::Black.italic()),
        padding: Padding::default(),
        lines: strings![
            "uncolored",
            Color::Red.paint("colored!!"),
            AnsiStyle::new().bold().paint("bolded"),
            Color::Blue.bold().paint("both")
        ]
    }.to_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "styled-with-ansi-text");
}

#[test]
fn padded() {
    let box_ = TermBox {
        border_style: BorderStyle::default(),
        padding: Padding::spaces(1),
        lines: strings![
            "padded",
            "text"
        ]
    }.to_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "padded")
}

#[test]
fn padded_with_ansi_text() {
    use nu_ansi_term::AnsiStrings;
    let box_ = TermBox {
        border_style: BorderStyle::new_double().with_style(AnsiStyle::new().bold()),
        padding: Padding::spaces(3),
        lines: strings![
            "cool",
            AnsiStrings(&[ Color::Red.paint("pa"), Color::Default.paint("dd"), Color::Purple.paint("ed") ]),
            Color::Blue.paint("text")
        ]
    }.to_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "padded-with-ansi-text")
}
