#[macro_export]
macro_rules! test_reader {
    ($reader: expr, $input: expr, $( $state:expr ),*) => {
        {
            let mut reader = $reader;
            let input = &mut ::std::io::Cursor::new($input);

            $(
                let next = reader.resume(input).unwrap();
                assert_eq!($state, next);
            )*
        }
    };
}