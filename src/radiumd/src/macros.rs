#[macro_export]
macro_rules! env_var {
    ($name: expr, $default: expr) => {
        match ::std::env::var($name) {
            Ok(val) => match val.parse() {
                Ok(val) => val,
                Err(..) => $default
            },
            Err(..) => $default
        }
    }
}