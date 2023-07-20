#[macro_export]
macro_rules! pwint {
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            write!(std::io::stdout(), $($arg)*).unwrap();
        }
    };
}

#[macro_export]
macro_rules! pwintwn {
    () => {
        println!("");
    };
    ($fmt:expr) => {
        {
            use std::io::Write;
            writeln!(std::io::stdout(), $fmt).unwrap();
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        {
            use std::io::Write;
            writeln!(std::io::stdout(), $fmt, $($arg)*).unwrap();
        }
    };
}

#[macro_export]
macro_rules! epwint {
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            write!(std::io::stderr(), $($arg)*).unwrap();
        }
    };
}

#[macro_export]
macro_rules! epwintwn {
    () => {
        eprintln!("");
    };
    ($fmt:expr) => {
        {
            use std::io::Write;
            writeln!(std::io::stderr(), $fmt).unwrap();
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        {
            use std::io::Write;
            writeln!(std::io::stderr(), $fmt, $($arg)*).unwrap();
        }
    };
}

#[macro_export]
macro_rules! uwufy {
    ($text:expr) => {{
        $text.chars().fold(String::new(), |mut result, c| {
            match c {
                'l' | 'r' => {
                    // Replace 'l' and 'r' with 'w'
                    result.push('w');
                }
                'L' | 'R' => {
                    // Replace 'L' and 'R' with 'W'
                    result.push('W');
                }
                'n' => {
                    // Replace 'n' with 'ny'
                    result.push('n');
                    result.push('y');
                }
                'N' => {
                    // Replace 'N' with 'Ny'
                    result.push('N');
                    result.push('y');
                }
                _ => {
                    // Otherwise, just add the character to the result
                    result.push(c);
                }
            }
            result
        })
    }};
}

#[macro_export]
macro_rules! uwu_pwintwn {
    ($($arg:tt)*) => {{
        let text = uwufy!(format!($($arg)*));
        println!("{}", text);
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        pwint!("One Line");
        let hello = "Hello";
        pwintwn!("{}", hello);
        epwint!("epwint error");
        epwintwn!("Error");
        uwu_pwintwn!("hello");
        let world = "world";
        uwu_pwintwn!("{}", world);
    }
}
