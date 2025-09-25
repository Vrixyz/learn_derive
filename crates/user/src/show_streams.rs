//! Shows how to debug a proc-macro by printing the input `TokenStream`s.

use macros::show_streams;

#[show_streams]
pub fn simple() {}

#[show_streams]
pub fn args_and_return(arg: bool) -> i32 {
    1
}

#[show_streams]
pub struct MyStruct {
    field: i32,
}
