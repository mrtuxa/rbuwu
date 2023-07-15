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
macro_rules! eprintwn {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        pwint!("One Line\n");
        let hello = "Hello";
        pwintwn!("{}", hello);
        eprintwn!("Error");
    }
}
