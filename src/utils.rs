
macro_rules! capitalize {
    ($string:expr) => {
        if $string.is_empty() {
            $string
        } else {
            let s = $string.to_owned();
            let mut b = s.chars();
            b.next().unwrap().to_uppercase().collect::<String>() + b.as_str()
        }
    };
}

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

macro_rules! print_color {
    () => (print!("\n"));

    ( $tag:ident -> $msg:expr, $($rest:tt)* ) => {
        {
            let color = color_str($msg, stringify!($tag));
            print!("{} ", color);
            print_color!($($rest)*);
        }
    };

    ( $tag:ident -> $msg:expr ) => (print_color!($tag -> $msg,));


    ( $msg:expr, $($rest:tt)* ) => (print_color!(X->$msg, $($rest)*));

    ( $msg:expr ) => (print_color!(X->$msg,));
        
}

macro_rules! colorize {

    () => {String::new()};

    ( [ $($acc:tt)* ]; $tag:ident -> $msg:expr, $($rest:tt)* ) => {
        {
            let color = color_str($msg, stringify!($tag));
            colorize!([ $($acc)* color, ]; $($rest)* )
        }
    };

    ( [ $($acc:tt)* ]; $msg:expr, $($rest:tt)* ) => {colorize!([$($acc)*]; X->$msg, $($rest)*)};

    ( [ $($acc:tt)* ]; $tag:ident -> $msg:expr ) => {colorize!([$($acc)*]; $tag -> $msg,)};

    ( [ $($acc:tt)* ]; $msg:expr ) => {colorize!([$($acc)*]; X->$msg,)};

    ( [ $($acc:tt)* ];) =>  { [$($acc)*].join(" ") };

    ( $($any:tt)* ) => { colorize!([]; $($any)* ) };
}


pub(crate) use {capitalize, print_color, colorize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        print_color!(Fr->"testing", Fbbi->"testing1", b->"testing2", x->"testing3", "testing4", Fgbu->"testing5");
        print_color!("hello");
    }

    #[test]
    fn test_colorize() {
        let col = colorize!(Fgb->"hello again", N->"hello", Fr->"goodbye", "again" );
        println!("{}", col)
    }

}