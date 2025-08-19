mod utils;

use super::*;
use utils::*;
use nu_ansi_term::AnsiStrings;

#[test]
fn empty() {
    let box_single = TermBox::default().into_string();
    let box_double = TermBox { border_style: BorderStyle::new_double(), ..TermBox::default() }.into_string();
    let box_line = TermBox::default().append_with("").into_string();
    assert_eq!(box_single, "┌─┐\n└─┘", "single");
    assert_eq!(box_double, "╔═╗\n╚═╝", "double");
    assert_eq!(box_line, "┌─┐\n│ │\n└─┘", "single w/ empty line");
}

#[test]
fn empty_styled() {
    let box_ = TermBox {
        border_style: BorderStyle::new_double().with_style(Color::Purple),
        ..TermBox::default()
    }.into_string();

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
    }.into_string();

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
            BOLD.paint("bolded"),
            Color::Blue.bold().paint("both")
        ]
    }.into_string();

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
    }.into_string();

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
            BOLD.paint("bolded"),
            Color::Blue.bold().paint("both")
        ]
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "styled-with-ansi-text");
}

#[test]
fn padded() {
    let box_ = TermBox {
        border_style: BorderStyle::default(),
        padding: Padding::ONE_SPACE,
        lines: strings![
            "padded",
            "text"
        ]
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "padded")
}

#[test]
fn padded_with_ansi_text() {
    let box_ = TermBox {
        border_style: BorderStyle::new_double().with_style(*BOLD),
        padding: Padding::spaces(3),
        lines: strings![
            "cool",
            AnsiStrings(&[ Color::Red.paint("pa"), Color::Default.paint("dd"), Color::Purple.paint("ed") ]),
            Color::Blue.paint("text")
        ]
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "padded-with-ansi-text")
}

#[test]
fn fat_box() {
    let box_ = TermBox {
        border_style: BorderStyle::new_single(),
        padding: Padding::new('\t', 3),
        lines: strings![
            BOLD.paint("F A T"),
            BOLD.paint("T E X T")
        ]
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "fat-box")
}

#[test]
fn long_box() {
    let box_ = TermBox {
        border_style: BorderStyle::new_double(),
        padding: Padding::none(),
        lines: str::repeat("Long text ", 3).chars().map(String::from).collect()
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "long-box");
}
