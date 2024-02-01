
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
pub(crate) use capitalize;