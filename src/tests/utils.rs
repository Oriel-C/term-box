use super::AnsiStyle;
use derive_new::new;
use ansi_width::ansi_width;
use std::cell::LazyCell;

macro_rules! strings {
    ($($strs:expr),*) => {
        vec![ $($strs.to_string()),* ]
    }
}

macro_rules! assert_okay {
    ($expr:expr $(, $name:literal)?) => {
        match $expr {
            Ok(val) => val,
            Err(err) => panic!(concat!("Assertion ", $("'", $name, "' ",)? "failed: {:?}"), err)
        }
    };
}

macro_rules! template_name {
    ($name:literal) => { concat!("test-input/", $name, ".txt") };
}

macro_rules! assert_matches_template {
    ($box:expr, $template:literal) => {{
        const TEMPLATE_PATH: &str = template_name!($template);
        let template = assert_okay!(std::fs::read_to_string(TEMPLATE_PATH), "template exists");
        assert_eq!($box, template);
    }};
}

#[allow(unused)]
macro_rules! init_template {
    ($box:expr, $template:expr) => {{
        const TEMPLATE_PATH: &str = template_name!($template);
        let _ = std::fs::write(TEMPLATE_PATH, $box).unwrap();
        assert!(false, "Test not yet finalized. Check {} contents and change to proper assertion.", TEMPLATE_PATH)
    }};
}

pub(crate) use strings;
pub(crate) use assert_okay;
pub(crate) use template_name;
pub(crate) use assert_matches_template;
#[allow(unused)]
pub(crate) use init_template;

#[derive(Debug, new)]
#[allow(dead_code)]
pub(crate) struct LineLenErr {
    expected: usize,
    found: usize,
    at: usize
}

pub(crate) fn lines_same_len(string: &str) -> Result<usize, LineLenErr> {
    string.split('\n')
        .into_iter()
        .enumerate()
        .try_fold(0, |len, (idx, next)| {
            let next_len = ansi_width(next);
            match len {
                0 => Ok(next_len),
                _ => if len == next_len { Ok(len) } else { Err(LineLenErr::new(len, next_len, idx)) }
            }
        })
}

pub(crate) const BOLD: LazyCell<AnsiStyle> = LazyCell::new(| | AnsiStyle::new().bold());
