pub fn color_str(input: &str, tag: &str) -> String {
    let mut it = tag.chars().peekable();
    let mut attr: Vec<&str> = vec![];
    let mut newline = "";

    while let Some(m) = it.next() {
        match m {
            'F' => {
                if let Some(n) = it.peek() {
                    let col = match n {
                        'r' => "31",
                        'g' => "32",
                        'y' => "33",
                        'b' => "34",
                        'm' => "35",
                        'c' => "36",
                        'w' => "37",
                        'k' => "30",
                        _ => ""
                    };
                    if !col.is_empty() {
                        it.next();
                        attr.push(col)
                    }
                }
            },
            'b' => attr.push("1"),
            'i' => attr.push("3"),
            'u' => attr.push("4"),
            'N' => newline = "\n",
            _ => {}
        }
    };

    format!("{}\x1b[{}m{}\x1b[0m", newline, attr.join(";"), input)
}


/// Inspired by the [row](https://github.com/phsym/prettytable-rs/blob/master/src/row.rs) macro in [prettytable](https://github.com/phsym/prettytable-rs)
/// 
/// ## Usage 
/// `colorize!(token->"str", ...)`
/// 
/// ## Tokens
/// Tokens can change color or font styling depending on their order and usage
/// 
/// ### Styling
///     1. b -> bold
///     2. u -> underline
///     3. i -> italic
/// 
/// ### Color
/// Color tokens start with an `F` (for foreground)
///     1. Fb -> blue
///     2. Fr -> red
///     3. Fg -> green
///     4. Fy -> yellow
///     5. Fm -> magenta
///     6. Fc -> cyan
///     7. Fw -> white
///     8. Fk -> black
/// 
/// ### Examples
/// ```
/// // Returns "Hello" in bold green
/// let color_string = colorize!(Fgb->"Hello");
/// assert_eq!(String::from("\x1b[1;32mHello\x1b[0m"), color_string);
/// 
/// // Returns "Hello" in italic blue and "World" underlined in magenta
/// let color_string = colorize!(iFb->"Hello", Fmu->"World");
/// assert_eq!(String::from("\x1b[3;34mHello\x1b[0m \x1b[4;35mWorld\x1b[0m"), color_string);
/// ```
#[macro_export]
macro_rules! colorize {

    () => {String::new()};

    ( [ $($acc:tt)* ]; $tag:ident -> $msg:expr, $($rest:tt)* ) => {
        {
            let color = $crate::color_str($msg, stringify!($tag));
            colorize!([ $($acc)* color, ]; $($rest)* )
        }
    };

    ( [ $($acc:tt)* ]; $msg:expr, $($rest:tt)* ) => {colorize!([$($acc)* $msg.to_string() ,]; $($rest)*)};

    ( [ $($acc:tt)* ]; $tag:ident -> $msg:expr ) => {colorize!([$($acc)*]; $tag -> &format!("{:?}", $msg),)};

    ( [ $($acc:tt)* ]; $msg:expr ) => {colorize!([$($acc)* $msg.to_string() ,]; )};

    ( [ $($acc:tt)* ]; ) =>  { [$($acc)*].join(" ") };

    ( $($any:tt)* ) => { colorize!([]; $($any)* ) };
}

/// `println` using the `colorize` macro
/// 
/// See [`colorize!`] for more details
#[macro_export]
macro_rules! print_color {
    () => (println!(""));
    ( $($any:tt)* ) => ( println!("{}", colorize!([]; $($any)*)) );
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_print_color() {
        print_color!(Fr->"testing", Fbbi->"testing1", b->"testing2{}", x->"testing3", "testing4", Fgbu->"testing5");
        print_color!("hello");
    }

    #[test]
    fn test_colorize() {
        let col = colorize!(Fgb->"hello again", N->"hello", "and", FrFb->"goodbye", "again" );
        println!("{}", col)
    }

    #[test]
    fn test_path_buf() {
        use std::path::PathBuf;
        let path = PathBuf::from_str("some").unwrap();
        let col = colorize!(Fgb->path);
        println!("{}", col);
    }

}