use std::borrow::Cow;

use std::ffi::CStr;

use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a = 42;
    let b: String;
    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;

        b = String::from_raw_parts(b_ptr, 10, 10);
    }

    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}
