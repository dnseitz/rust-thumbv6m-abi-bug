#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]

extern crate compiler_builtins;
pub use vector_table::RESET;

#[no_mangle]
pub fn start() -> ! {
  pass_to_i64_fn();
  
  // Succeeds
  do_a_mul(10, 10);

  // Succeeds
  do_a_mul(-10, -10);

  // Panics
  do_a_mul(-5, 5);
  loop {}
}

pub fn do_a_mul(a: isize, b: isize) -> isize {
  a * b
}

pub fn pass_to_i64_fn() -> i64 {
  i64_fn(1, -2)
}

fn i64_fn(a: i64, b: i64) -> i64 {
  a + b
}

#[lang="eh_personality"]
extern "C" fn eh_personality() {}
#[lang="panic_fmt"]
#[no_mangle]
extern "C" fn panic_fmt() -> ! { loop {} }

#[cfg(target_arch="arm")]
mod vector_table {
	#[link_section = ".reset"]
	#[no_mangle]
	pub static RESET: fn() -> ! = ::start;
}
