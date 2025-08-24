use super::*;

pub(super) fn line_len(line: &CountedString, padding: usize) -> usize {
    line.width + TermBox::SIDES + (TermBox::SIDES * padding)
}

pub(super) fn make_line(
    buf: &mut String,
    edge_string: &str,
    pad_string: &CountedString,
    text: &CountedString,
    min_len: usize
) {
    buf.push_str(edge_string);
    buf.push_str(pad_string.str());
    buf.push_str(text.str());
    
    let diff = min_len - line_len(text, pad_string.width);
    if diff > 0 {
        buf.push_str(&str::repeat(" ", diff))
    }

    buf.push_str(pad_string.str());
    buf.push_str(edge_string);
    buf.push('\n')
}

struct HorizLineArgs<'a> {
    len: usize,
    style: BorderStyle,
    title: &'a Title,
    left: BorderChar,
    right: BorderChar
}

pub(super) fn make_top_line(buf: &mut String, tbox: &TermBox, len: usize) {
    make_top_or_bottom_line(buf, HorizLineArgs {
        len,
        style: tbox.border_style, title: &tbox.titles.top,
        left: BorderChar::TopLeft, right: BorderChar::TopRight
    });
    buf.push('\n')
}

pub(super) fn make_bottom_line(buf: &mut String, tbox: &TermBox, len: usize) {
    make_top_or_bottom_line(buf, HorizLineArgs {
        len,
        style: tbox.border_style, title: &tbox.titles.bottom,
        left: BorderChar::BotLeft, right: BorderChar::BotRight
    })
}

pub(crate) const DEFAULT_DIST_FROM_CORNER: usize = 1;

fn make_top_or_bottom_line(buf: &mut String, args: HorizLineArgs) {
    let style = args.style;
    let shape = style.shape;
    let edge_char = shape.get_char(BorderChar::Edge);
    // String.len() is in bytes
    let mut tmp_buf = alloc_title_buf(&args);
    tmp_buf += shape.get_char(args.left);

    let right_char = shape.get_char(args.right);
    if !args.title.is_empty() {
        tmp_buf = ins_title(tmp_buf, edge_char, right_char, &args);
    } else {
        tmp_buf += &(edge_char.repeat(args.len - TermBox::SIDES) + right_char);
    }

    // Works in all cases except a styled right title, which would be fairly complicated
    // for something not very worth covering for
    // let actual = tmp_buf.len();
    // assert!(actual == init_cap, "{actual} != {init_cap}");

    if style.ansi.is_plain() {
        buf.push_str(&tmp_buf);
    } else {
        buf.push_str(&style.ansi.paint(tmp_buf).to_string());
    }
}

fn alloc_title_buf(args: &HorizLineArgs) -> String {
    let mut cap = BorderChar::NUM_BYTES * (args.len - args.title.width());
    cap += args.title.len_bytes();
    String::with_capacity(cap)
}

fn ins_title(mut buf: String, edge_char: &str, right_char: &str, args: &HorizLineArgs) -> String {
    let title = args.title;
    let left_pad_len = title.left_pad_len(args.len);

    buf += &edge_char.repeat(left_pad_len);
    buf += title.text();

    let right_pad_len = title.right_pad_len(args.len);
    let right_pad = edge_char.repeat(right_pad_len) + right_char;

    // titles may reset the style, so apply it again if we have one
    if args.style.ansi.is_plain() {
        buf += &right_pad;
    } else {
        buf += &args.style.ansi.paint(right_pad).to_string();
    }

    buf
}
