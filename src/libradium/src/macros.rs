#[macro_export]
macro_rules! maybe {
    ($value:expr) => {
        match $value {
            None => { return None },
            Some(value) => { value }
        }
    }
}