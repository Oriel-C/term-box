Rust library for pretty-printing information to a terminal in a box.

# Examples

Print an empty box:

```
use term_box::TermBox;

TermBox::default().print()
```

Output:

```
┌─┐
└─┘
```


Create a simple box:

```
use term_box::*;

let my_box = TermBox {
    border_style: BorderShape::Single.into(),
    padding: Padding::ONE_SPACE,
    titles: Titles::none(),
    lines: lines![
        "my",
        "cool",
        "box"
    ]
};

my_box.print()
```

Output:

```
┌──────┐
│ my   │
│ cool │
│ box  │
└──────┘
```

Print a styled box showing the current time since the unix epoch:

```
use term_box::*;
use nu_ansi_term::Color;
use std::time::SystemTime;

let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

let time_box = TermBox {
    border_style: BorderStyle::new_double().with_style(Color::Cyan),
    padding: Padding::spaces(2),
    titles: Titles {
        top: Title("Time since unix epoch", TitlePosition::Centered),
        bottom: Title::empty(),
    },
    lines: lines![
        "",
        format!("In seconds: {}", time.as_secs()),
        format!("In milliseconds: {}", time.as_millis()),
        format!("in nanoseconds: {}", time.as_nanos()),
        Color::Blue.bold().paint("Irrelevant styled text to show that you can do this"),
        AnsiStyle::new().italic().paint("More styled text to show another way"),
        ""
    ]
};

time_box.print()
```

Output (minus coloring):

```
╔═════════════════Time since unix epoch═════════════════╗
║                                                       ║
║  In seconds: 1756144695                               ║
║  In milliseconds: 1756144695323                       ║
║  in nanoseconds: 1756144695323068140                  ║
║  Irrelevant styled text to show that you can do this  ║
║  More styled text to show another way                 ║
║                                                       ║
╚═══════════════════════════════════════════════════════╝
```

Full color output for this example can be found in the sources at test-inputs/time-example.txt.
Use 'cat' or a similar command in a terminal to view it properly.
