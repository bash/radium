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

#[macro_export]
macro_rules! test_writer {
    ($writer: expr, $output: expr) => {
        {
            let mut buf = io::Cursor::new($output);
            let mut ctrl = $crate::writer::SyncWriterController::new($writer);
            let result = ctrl.resume(&mut buf);

            (buf.into_inner(), result)
        }
    }
}

