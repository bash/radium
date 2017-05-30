#[macro_export]
macro_rules! test_reader {
    ($reader: expr, $input: expr, $( $state:expr ),*) => {
        {
            let mut reader = $reader;
            let slice = $input.as_mut_slice();
            let mut input = &mut slice.as_ref();

            $(
                let next = reader.resume(input).unwrap();
                assert_eq!($state, next);
            )*
        }
    };
}