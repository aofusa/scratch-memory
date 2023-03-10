#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::arch::asm;
use core::ptr;
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[allow(dead_code)]
fn write(fd: usize, buf: *const u8, len: usize) {
    unsafe {
        // syscall!(WRITE, fd, buf as usize, len);
        let mut _write: usize = 1;
        asm!(
            "syscall",
            inout("rax") _write,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") len,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
}

#[allow(dead_code)]
fn exit(status: usize) -> ! {
    unsafe {
        // syscall!(EXIT, status);
        let mut _exit: usize = 60;
        asm!(
            "syscall",
            inout("rax") _exit,
            in("rdi") status,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    loop {}
}

#[allow(dead_code)]
unsafe fn mmap(start: usize, length: usize, prot: usize, flags: usize, fd: usize, offset: usize) -> usize {
    // syscall!(MMAP, start, length, prot, flags, fd, offset);
    let mut _mmap: usize = 9;
    asm!(
        "syscall",
        inout("rax") _mmap,
        in("rdi") start,
        in("rsi") length,
        in("rdx") prot,
        in("r10") flags,
        in("r8") fd,
        in("r9") offset,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    _mmap
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "Hello, world!\n";

    unsafe {
        // メモリ確保
        let start = 0;
        let length = msg.len();  // ページサイズ未満の場合、ページサイズになるように切り上げられる（手元の環境だと4096）
        let prot = 0x7;  // PROT_READ|PROT_WRITE|PROT_EXEC
        let flags = 0x22;  // MAP_PRIVATE|MAP_ANONYMOUS
        let fd = usize::MAX;  // メモリ確保の場合-1を指定する
        let offset = 0;
        let mm = mmap(start, length, prot, flags, fd, offset) as *mut u8;

        // メモリの内容が書き換えられていることを確認
        assert_eq!(mm.read(), 0);
        ptr::write(mm, 42);
        assert_eq!(mm.read(), 42);

        // スタックの内容をメモリにコピーする
        for (idx, v) in msg.chars().enumerate() {
            ptr::write(mm.offset(idx as isize), v as u8);
        }

        // メモリの内容を標準出力に書き出す
        write(1, mm, msg.len());
    }

    exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        unsafe {
            // メモリ確保
            let start = 0;
            let length = 1;  // ページサイズ未満の場合、ページサイズになるように切り上げられる（手元の環境だと4096）
            let prot = 0x7;  // PROT_READ|PROT_WRITE|PROT_EXEC
            let flags = 0x22;  // MAP_PRIVATE|MAP_ANONYMOUS
            let fd = usize::MAX;  // メモリ確保の場合-1を指定する
            let offset = 0;
            let mm = mmap(start, length, prot, flags, fd, offset) as *mut u8;
    
            // メモリの内容が書き換えられていることを確認
            assert_eq!(mm.read(), 0);
            ptr::write(mm, 42);
            assert_eq!(mm.read(), 42);
        }
    }
}


