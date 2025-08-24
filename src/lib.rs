//! Print text in pretty boxes to the terminal.
//!
//! This library integrates heavily with [nu_ansi_term] for styling output.
//!
//! # Examples
//!
//! Print an empty box:
//!
//! ```
//! use term_box::TermBox;
//!
//! TermBox::default().print()
//! ```
//!
//! Create a simple box:
//!
//! ```
//! use term_box::*;
//!
//! let my_box = TermBox {
//!     border_style: BorderStyle::new_single(),
//!     padding: Padding::ONE_SPACE,
//!     titles: Titles::none(),
//!     lines: lines![
//!         "my",
//!         "cool",
//!         "box"
//!     ]
//! };
//!
//! // Depending on terminal font, gaps between the lines in the border of the box shown in
//! // documentation may or may not appear.
//! let output = "
//! ┌──────┐
//! │ my   │
//! │ cool │
//! │ box  │
//! └──────┘
//! ";
//!
//! assert_eq!(my_box.into_string(), output.trim());
//! ```
//!
//! Print a styled box showing the current time since the unix epoch:
//!
//! ```
//! use term_box::*;
//! use nu_ansi_term::Color;
//! use std::time::SystemTime;
//!
//! let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
//!
//! let time_box = TermBox {
//!     border_style: BorderStyle::new_double().with_style(Color::Cyan),
//!     padding: Padding::spaces(2),
//!     titles: Titles {
//!         top: Title("Time since unix epoch", TitlePosition::Centered),
//!         bottom: Title::empty(),
//!     },
//!     lines: lines![
//!         "",
//!         format!("In seconds: {}", time.as_secs()),
//!         format!("In milliseconds: {}", time.as_millis()),
//!         format!("in nanoseconds: {}", time.as_nanos()),
//!         Color::Blue.bold().paint("Irrelevant styled text to show that you can do this"),
//!         AnsiStyle::new().italic().paint("More styled text to show another way"),
//!         ""
//!     ]
//! };
//!
//! time_box.print()
//! ```

#[cfg(test)]
mod tests;

mod core;
mod padding;

pub mod border;
pub mod line;
pub mod title;

pub use {
    nu_ansi_term::{Color, Style as AnsiStyle},
    border::{BorderShape, BorderStyle},
    title::{Title, Titles, TitlePosition},
    padding::Padding,
    core::*
};

pub(crate) use {border::BorderChar, line::CountedString};

