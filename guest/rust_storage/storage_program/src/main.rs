#![feature(thread_id_value)]
use std::cell::RefCell;
use std::process::ExitCode;

thread_local! {
  pub static COUNT: std::cell::RefCell<i32> = const { RefCell::new(0) };
  pub static VALUES: std::cell::RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
}

#[no_mangle]
extern "C" fn remote_function(arg: extern "C" fn(i32) -> i32, value: i32) -> i32 {
    println!(
        "Called from thread: {}",
        std::thread::current().id().as_u64()
    );
    COUNT.with_borrow_mut(|count| *count += 1);
    VALUES.with_borrow_mut(|v| v.push(value)); // XXX This line makes it crash
    println!("After push");
    return arg(value);
}

extern "C" fn double_int(input: i32) -> i32 {
    return input * 2;
}

fn main() -> ExitCode {
    println!("Hello, Storage World!");
    remote_function(double_int, 1);
    return ExitCode::from(123);
}
