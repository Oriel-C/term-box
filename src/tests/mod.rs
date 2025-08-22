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
        lines: lines![
            "a",
            "few",
            "lines",
            "for testing"
        ],
        ..TermBox::default()
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_eq!(box_, "┌───────────┐\n│a          │\n│few        │\n│lines      │\n│for testing│\n└───────────┘");
}

#[test]
fn unstyled_with_ansi_text() {
    let box_ = TermBox {
        lines: lines![
            "uncolored",
            Color::Red.paint("colored!!"),
            BOLD.paint("bolded"),
            Color::Blue.bold().paint("both")
        ],
        ..TermBox::default()
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "unstyled-with-ansi-text");
}

#[test]
fn styled() {
    let box_ = TermBox {
        border_style: BorderStyle::new_single().with_style(Color::LightPurple.bold()),
        lines: lines![
            "some",
            "cool",
            "text"
        ],
        ..TermBox::default()
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "styled");
}

#[test]
fn styled_with_ansi_text() {
    let box_ = TermBox {
        border_style: BorderStyle::new_double().with_style(Color::Black.italic()),
        lines: lines![
            "uncolored",
            Color::Red.paint("colored!!"),
            BOLD.paint("bolded"),
            Color::Blue.bold().paint("both")
        ],
        ..TermBox::default()
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "styled-with-ansi-text");
}

#[test]
fn padded() {
    let box_ = TermBox {
        padding: Padding::ONE_SPACE,
        lines: lines![
            "padded",
            "text"
        ],
        ..TermBox::default()
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "padded")
}

#[test]
fn padded_with_ansi_text() {
    let box_ = TermBox {
        border_style: BorderStyle::new_double().with_style(*BOLD),
        padding: Padding::spaces(3),
        titles: Titles::none(),
        lines: lines![
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
        lines: lines![
            BOLD.paint("F A T"),
            BOLD.paint("T E X T")
        ],
        ..TermBox::default()
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "fat-box")
}

#[test]
fn long_box() {
    let box_ = TermBox {
        border_style: BorderStyle::new_double(),
        padding: Padding::none(),
        lines: str::repeat("Long text ", 3).chars().map(String::from).collect(),
        ..TermBox::default()
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "long-box");
}

// TODO: necessary test: ANSI-control-only titles (all 3 positions; can be done in 1 func)

#[test]
fn titles_left() {
    let box_ = TermBox {
        border_style: BorderStyle::new_single(),
        padding: Padding::ONE_SPACE,
        titles: Titles {
            top: Title("the", TitlePosition::Left),
            bottom: Title(Color::Red.bold().paint("ever"), TitlePosition::Left)
        },
        lines: lines![
            "coolest",
            "box"
        ]
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    assert_matches_template!(box_, "titles-left")
}

#[test]
fn titles_center() {
    let box_ = TermBox {
        border_style: BorderStyle::new_double(),
        padding: Padding::ONE_SPACE,
        titles: Titles {
            top: Title(BOLD.paint("center"), TitlePosition::Centered), // Test: even title, odd len
            bottom: Title(BOLD.paint("of the universe"), TitlePosition::Centered), // Test: odd title, odd len
        },
        lines: lines![
            "the church",
            "viewed the",
            "earth",
            "as the"
        ]
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    // init_template!(&box_, "titles-center");
    assert_matches_template!(box_, "titles-center")
}

#[test]
fn titles_right() {
    let box_ = TermBox {
        border_style: BorderStyle::new_single().with_style(Color::Cyan),
        padding: Padding::none(),
        titles: Titles {
            top: Title(Color::LightMagenta.paint("Nicolaus"), TitlePosition::Right),
            bottom: Title(Color::Blue.bold().paint("Copernicus"), TitlePosition::Right)
        },
        lines: lines![
            "was censured",
            "for saying",
            "otherwise"
        ]
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    // init_template!(&box_, "titles-right");
    assert_matches_template!(box_, "titles-right")
}

#[test]
fn titles_center_2() {
    let box_ = TermBox {
        border_style: BorderStyle::new_single(),
        padding: Padding::none(),
        titles: Titles {
            top: Title(BOLD.paint("odd"), TitlePosition::Centered), // Test: odd title, even len
            bottom: Title(AnsiStyle::new().italic().paint("even"), TitlePosition::Centered) // Test: even title, even len
        },
        lines: lines![
            "even",
            "widths"
        ]
    }.into_string();

    assert_okay!(lines_same_len(&box_));
    // init_template!(&box_, "titles-center-2");
    assert_matches_template!(box_, "titles-center-2");
}
