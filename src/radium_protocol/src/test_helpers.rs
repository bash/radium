#[macro_export]
macro_rules! test_reader {
    ($reader: expr, $input: expr) => {
        {
            let mut buf = io::Cursor::new($input);
            let mut ctrl = $crate::reader::SyncReaderController::new($reader);

            ctrl.resume(&mut buf)
        }
    }
}

