
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

macro_rules! print_color {
    () => (print!("\n"));

    ( $tag:ident -> $msg:expr, $($rest:tt)* ) => {
        {
            match stringify!($tag) {
                "f" => print!("\x1b[35m{}\x1b[0m ", $msg),
                _ => print!("{} ", $msg)
            };
            print_color!($($rest)*);
        }
    };

    ( $tag:ident -> $msg:expr ) => (print_color!($tag -> $msg,));


    ( $msg:expr, $($rest:tt)* ) => (print_color!(N->$msg, $($rest)*));
        
}


pub(crate) use {capitalize, print_color};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        print_color!(f->"testing", "testing", f->"testing2", "testing4", "testing5", f->"testing6");
    }

}