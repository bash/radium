macro_rules! impl_err_display {
    ($ty:ty) => {
        impl ::std::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use std::error::Error;
                write!(f, "{}", self.description())
            }
        }
    }
}