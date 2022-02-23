//! The NaN-boxing encoding used is loosely based off https://piotrduperas.com/posts/nan-boxing which is in turned based off SpiderMonkey.

// TODO: Support x86, arm, wasm32, wasm64, powerpc, powerpc64, mips, mips64
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
compile_error!("zaia currently only supports x86_64 and aarch64");

const BOOL_MASK: u64 = 0x7FFE000000000002;
const INTEGER_MASK: u64 = 0x7FFC000000000000;
const FLOAT_MASK: u64 = 0xFFFF000000000000;
const TABLE_MASK: u64 = 0xFFFC000000000000;
const STRING_MASK: u64 = 0xFFFE000000000000;
const PTR_MASK: u64 = 0xFFFFFFFFFFFF;

const NIL_VALUE: u64 = 0x7FFE000000000000;
const TRUE_VALUE: u64 = BOOL_MASK | 3;
const FALSE_VALUE: u64 = BOOL_MASK | 2;

fn is_ptr(x: u64) -> bool {
    x & 7 == 0
}

pub fn is_nil(x: u64) -> bool {
    x == NIL_VALUE
}

pub fn make_nil() -> u64 {
    NIL_VALUE
}

pub fn is_bool(x: u64) -> bool {
    x & BOOL_MASK == BOOL_MASK
}

pub fn make_bool(x: bool) -> u64 {
    if x {
        TRUE_VALUE
    } else {
        FALSE_VALUE
    }
}

pub fn get_bool(x: u64) -> bool {
    x & 1 == 1
}

pub fn is_int(x: u64) -> bool {
    (x & FLOAT_MASK) == INTEGER_MASK
}

pub fn make_int(x: i32) -> u64 {
    x as u64 | INTEGER_MASK
}

pub fn get_int(x: u64) -> i32 {
    x as i32
}

pub fn is_float(x: u64) -> bool {
    (x & FLOAT_MASK) != FLOAT_MASK
}

pub fn make_float(x: f64) -> u64 {
    x.to_bits()
}

pub fn get_float(x: u64) -> f64 {
    f64::from_bits(x)
}

pub fn is_table(x: u64) -> bool {
    is_ptr(x) && (x & FLOAT_MASK) == TABLE_MASK
}

pub fn make_table(x: *mut u8) -> u64 {
    x as u64 | TABLE_MASK
}

pub fn get_table(x: u64) -> *mut u8 {
    (x & PTR_MASK) as *mut u8
}

pub fn is_string(x: u64) -> bool {
    is_ptr(x) && (x & FLOAT_MASK) == STRING_MASK
}

pub fn make_string(x: *mut u8) -> u64 {
    x as u64 | STRING_MASK
}

pub fn get_string(x: u64) -> *mut u8 {
    (x & PTR_MASK) as *mut u8
}

pub fn is_function(x: u64) -> bool {
    todo!()
}

pub fn make_function(x: *mut u8) -> u64 {
    todo!()
}

pub fn get_function(x: u64) -> *mut u8 {
    (x & PTR_MASK) as *mut u8
}

pub fn is_userdata(x: u64) -> bool {
    todo!()
}

pub fn make_userdata(x: *mut u8) -> u64 {
    todo!()
}

pub fn get_userdata(x: u64) -> *mut u8 {
    (x & PTR_MASK) as *mut u8
}