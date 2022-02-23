//! The NaN-boxing encoding used is loosely based off https://piotrduperas.com/posts/nan-boxing which is in turned based off SpiderMonkey.
//! TODO: Investigate WebKit Strategy https://brionv.com/log/2018/05/17/javascript-engine-internals-nan-boxing/

// TODO: Add fallback for big-endian.
#[cfg(not(target_endian = "little"))]
compile_error!("zaia currently only supports little-endian platforms");

// TODO: Add fallback for 32-bit.
#[cfg(not(target_pointer_width = "64"))]
compile_error!("zaia currently only supports 64-bit platforms");

const BOOL_MASK: u64 = 0x7FFE000000000002;
const INTEGER_MASK: u64 = 0x7FFC000000000000;
const FLOAT_MASK: u64 = 0xFFFF000000000000;
const TABLE_MASK: u64 = 0xFFFC000000000000;
const STRING_MASK: u64 = 0xFFFE000000000000;

const NIL_VALUE: u64 = 0x7FFE000000000000;
const TRUE_VALUE: u64 = BOOL_MASK | 3;
const FALSE_VALUE: u64 = BOOL_MASK | 2;

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

pub fn is_int(x: u64) -> bool {
    (x & FLOAT_MASK) == INTEGER_MASK
}

pub fn make_int(x: i32) -> u64 {
    x as u64 | INTEGER_MASK
}

pub fn is_float(x: u64) -> bool {
    (x & FLOAT_MASK) != FLOAT_MASK
}

pub fn make_float(x: f64) -> u64 {
    x.to_bits()
}

pub fn is_table(x: u64) -> bool {
    (x & FLOAT_MASK) == TABLE_MASK
}

pub fn make_table(x: *mut u8) -> u64 {
    x as u64 | TABLE_MASK
}

pub fn is_string(x: u64) -> bool {
    (x & FLOAT_MASK) == STRING_MASK
}

pub fn make_string(x: *mut u8) -> u64 {
    x as u64 | STRING_MASK
}

pub fn is_function(x: u64) -> bool {
    todo!()
}

pub fn make_function(x: *mut u8) -> u64 {
    todo!()
}

pub fn is_userdata(x: u64) -> bool {
    todo!()
}

pub fn make_userdata(x: *mut u8) -> u64 {
    todo!()
}