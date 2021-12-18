use std::fmt::{Arguments, Debug, Display};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
        static ref FILE: Mutex<Option<BufWriter<File>>> = Mutex::new(None);
    }

pub fn init<P: AsRef<Path>>(file: P) {
    let file = File::create(file).unwrap();

    *FILE.lock().unwrap() = Some(BufWriter::with_capacity(3e6 as usize, file));
}

pub fn write_log(args: Arguments) {
    write!(FILE.lock().unwrap().as_mut().unwrap(), "{}", args).unwrap();
}

pub fn write_keyvalue<Key: Display, Value: Debug>(key: Key, value: Value) {
    write!(FILE.lock().unwrap().as_mut().unwrap(), r#""{}": {:?},"#, key, value).unwrap();
}

pub fn finish() {
    write_log(format_args!("{}", r#""last":""}"#));
    FILE.lock().unwrap().take().unwrap().flush().unwrap();
}