#![no_std]
#![no_main]
#![feature(asm_const)]

use core::arch::asm;

const SYS_WRITE0: u8 = 0x4;
const SYS_WRITEC: u8 = 0x3;
const ANGEL_SWI: u8 = 0xab;
static mut FOO: [u8;13] = *b"Hello World!\0";

/// Ouputs a c string using semihosting
fn puts(s: &core::ffi::CStr) {
    unsafe {
        asm!(
            "movs r0, {syswrite0}",
            "movs r1, {s}",
            "bkpt {angel_swi}",
            syswrite0 = const SYS_WRITE0,
            s = in(reg) s.as_ptr(),
            angel_swi = const ANGEL_SWI,
            out("r0") _,
            out("r1") _,
        );
    }
}

/// Ouputs a character byte using semihostting
fn putc(c: u8) {
    unsafe {
        asm!(
            "movs r0, {syswrite0}",
            "movs r1, {c}",
            "bkpt {angel_swi}",
            syswrite0 = const SYS_WRITEC,
            c = in(reg) core::ptr::addr_of!(c),
            angel_swi = const ANGEL_SWI,
            out("r0") _,
            out("r1") _,
        );
    }
}

struct MyPuts;
use core::fmt::Write;

impl core::fmt::Write for MyPuts {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        s.as_bytes().iter().for_each(|c| putc(*c));
        Ok(())
    }
}

const BPKT: u8 = 0x3;

fn bpkt() {
    unsafe {
        asm!("bkpt {number}",
             number = const BPKT
             );
    }
}


trait A {
    fn toto(&self) -> u8;
}

fn print_an_A(a: &impl A) {
    putc(a.toto());
}


impl A for u8 {
    fn toto(&self) -> u8 {
        *self
    }
}

#[no_mangle]
extern "C" fn _start() -> ! {
    bpkt();

    let x: u8 = b'Z';
    print_an_A(&x);
    bpkt();
    putc(b'\n');
    putc(b'\n');
    putc(b'\n');
    putc(b'\n');
    putc(b'\n');
    putc(b'\n');
    putc(b'O');
    putc(b'K');



    // let mut s = MyPuts;
    // writeln!(s, "Bonjour les gens {:x}", 0xFACADE).ok();
/*
    let c_string = core::ffi::CStr::from_bytes_until_nul(b"Hello world\ndone\n\0").unwrap();
    unsafe {
        FOO[0] = 'A' as u8;
        puts(core::ffi::CStr::from_bytes_with_nul_unchecked(&FOO));
    }
    puts(c_string);*/
    loop {}
}

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> !{
    let mut s = MyPuts;
    s.write_str("\n\nPANIC\n\n").ok();
    loop {}
}
