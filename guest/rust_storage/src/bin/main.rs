#![feature(allocator_api)]
use std::alloc::Allocator;
use std::alloc::Global;

use std::arch::asm;
use std::process::ExitCode;
extern "C" { fn remote_function(arg: fn(i32) -> i32, value: i32) -> i32; }
// Perform a remote allocation with a local allocator
extern "C" { fn remote_allocation(fsbase: u64, alloc: &dyn Allocator) -> Vec<i32, &dyn Allocator>; }

fn double_int(input: i32) -> i32 {
    return input * 2;
}

extern "C" fn do_calculation(input: i32) -> i32 {
    return unsafe { remote_function(double_int, input) };
}
extern "C" fn do_nothing(_input: i32) -> i32 {
	return 42;
}

// Don't exactly know how else to call a named function
// in a dynamic ELF w/interpreter. We don't know the
// base address of the binary, so we can't adjust the
// symbol address.
fn set_callback(name: &str, cb: extern "C" fn(i32) -> i32) {
	let sysnum = 0x10001; // Set callback syscall
	unsafe {
		asm!(
			"out 0, eax",
			in("eax") sysnum,
			in("rdi") cb,
			in("rsi") name.as_ptr(),
			in("rdx") name.len()
		);
	}
}

fn get_current_fsbase() -> u64 {
	let fsbase: u64;
	unsafe {
		asm!(
			"rdfsbase {}",
			out(reg) fsbase,
		);
	}
	return fsbase;
}

fn main() -> ExitCode
{
	println!("Hello, world!");
	let result = unsafe { remote_function(double_int, 21) };
	println!("Result from remote function: {}", result);

	let vec = unsafe { remote_allocation(get_current_fsbase(), &Global) };
	println!("Received vector from remote allocation: {:?}", vec);

	// Register callbacks
	set_callback("do_calculation", do_calculation);
	set_callback("do_nothing", do_nothing);
	return ExitCode::from(result as u8);
}
