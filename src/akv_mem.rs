use std::slice;
use std::str;

use libactionkv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE update KEY VALUE
    akv_mem FILE insert KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },
        "delete" => store.delete(key).expect("unable to delete value"),
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).expect("unable to insert value");
        }
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).expect("unable to update value");
        }
        _ => eprintln!("{}", &USAGE),
    }

    let story = "Once upon a time...";

    let ptr = story.as_ptr();
    let len = story.len();

    let s = unsafe {
        // First, we build a &[u8]...
        let slice: &[u8] = slice::from_raw_parts(ptr, len);

        // ... and then convert that slice into a string slice
        str::from_utf8(slice)
    };
}