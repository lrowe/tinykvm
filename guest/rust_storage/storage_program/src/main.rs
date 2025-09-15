#![feature(thread_id_value)]
use std::process::ExitCode;

#[no_mangle]
extern "C" fn remote_function(_arg: extern "C" fn(i32) -> i32, value: i32) -> i32 {
    println!(
        "Called from thread: {}",
        std::thread::current().id().as_u64()
    );
		let conn = rusqlite::Connection::open_in_memory().unwrap();
		let mut stmt = conn.prepare("SELECT 1 + ?1").unwrap();
		stmt.query_one([value], |row| row.get(0)).unwrap()
}

extern "C" fn double_int(input: i32) -> i32 {
    return input * 2;
}

fn main() -> ExitCode {
    println!("Hello, Storage World!");
    remote_function(double_int, 1);
    return ExitCode::from(123);
}
