#![feature(thread_id_value)]
use std::process::ExitCode;

#[no_mangle]
extern "C" fn remote_function(arg: extern "C" fn(i32) -> i32, value: i32) -> i32 {
    println!(
        "Called from thread: {}",
        std::thread::current().id().as_u64()
    );
    let conn = sqlite::open(":memory:").unwrap();
    let mut stmt = conn.prepare("SELECT 1 + ?").unwrap();
    stmt.bind((1, 1)).unwrap();
    if let Ok(sqlite::State::Row) = stmt.next() {
        let result = stmt.read::<i64, _>(0).unwrap();
        println!("Storage Result from sqlite: {}", result);
    }
    stmt.reset().unwrap();
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
