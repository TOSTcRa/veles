// SPDX-License-Identifier: GPL-2.0

//! Rust minimal module.

use kernel::prelude::*;

module! {
    type: Veles,
    name: "Veles",
    authors: ["tostcr and nnnnmeow"],
    description: "Simple rust module",
    license: "GPL",
}

#[repr(C)]
struct linux_binprm {
    _opaque: u8,
}

#[repr(C)]
struct linux_binfmt {
    lh: [u8; 16],
    module: *mut c_void,
    load_binary: unsafe extern "C" fn(*mut linux_binprm) -> c_int,
}

extern "C" {
    fn __register_binfmt(fmt: *mut linux_binfmt, insert: c_int);

    fn unregister_binfmt(fmt: *mut linux_binfmt);

    fn get_buf(bprm: *mut linux_binprm) -> *const u8;

    fn veles_get_file(bprm: *mut linux_binprm) -> *mut c_void;

    fn kernel_read(file: *mut c_void, buf: *mut u8, count: usize, pos: *mut i64) -> isize;

    fn vm_mmap(
        file: *mut c_void,
        addr: c_ulong,
        size: c_ulong,
        prot: c_ulong,
        flags: c_ulong,
        offset: c_ulong,
    ) -> c_ulong;

    fn begin_new_exec(bprm: *mut linux_binprm) -> c_int;

    fn veles_get_to_user(file: *mut c_void, addr: c_ulong, offset: i64, size: usize) -> isize;

    fn veles_start_thread(ip: c_ulong, sp: c_ulong);

    fn setup_new_exec(bprm: *mut linux_binprm);
}

unsafe extern "C" fn load_pe_binary(bprm: *mut linux_binprm) -> c_int {
    unsafe {
        let buf = get_buf(bprm);

        if *buf == 0x4D && *buf.add(1) == 0x5A {
            pr_info!("PE detected\n");

            let offset = (buf.add(0x3C) as *const u32).read_unaligned();

            if offset < 252 {
                if (buf.add(offset as usize) as *const u32).read_unaligned() == 0x00004550 {
                    pr_info!("got the PE header\n");

                    let file = veles_get_file(bprm);
                    let mut pe_buf = [0u8; 1024];
                    let mut pos: i64 = 0;
                    let pointer = &mut pe_buf[0] as *mut u8;
                    kernel_read(file, pointer, 1024, &mut pos);

                    if (pointer.add(offset as usize + 4) as *const u16).read_unaligned() == 0x8664 {
                        pr_info!("AMD64 arch\n");
                    } else {
                        pr_info!("not an AMD64 arch\n");
                    }

                    let sections =
                        (pointer.add(offset as usize + 6) as *const u16).read_unaligned();
                    pr_info!("{} sections\n", sections);

                    let entry_point =
                        (pointer.add(offset as usize + 40) as *const u32).read_unaligned();
                    pr_info!("{:#x}, is the entry point\n", entry_point);

                    let size_of_header =
                        (pointer.add(offset as usize + 20) as *const u16).read_unaligned();

                    let image_base =
                        (pointer.add(offset as usize + 48) as *const u64).read_unaligned();

                    pr_info!("{:#x} is the image_base\n", image_base);

                    let ret = begin_new_exec(bprm);
                    if ret != 0 {
                        return ret;
                    }
                    setup_new_exec(bprm);
                    let start = offset as usize + 4 + 20 + size_of_header as usize;
                    for i in 0..sections {
                        let sec = start + i as usize * 40;

                        let addr = (pointer.add(sec + 12) as *const u32).read_unaligned();
                        let size = (pointer.add(sec + 16) as *const u32).read_unaligned();
                        let offest_in_file = (pointer.add(sec + 20) as *const u32).read_unaligned();
                        pr_info!(
                            "addres in memory: {}, size: {}, offset in file: {}\n",
                            addr,
                            size,
                            offest_in_file,
                        );

                        let mmap_addr = (image_base + addr as u64) as usize;

                        let result =
                            vm_mmap(core::ptr::null_mut(), mmap_addr, size as usize, 7, 0x32, 0);

                        if (result as isize) < 0 && (result as isize) > -4096 {
                            return result as c_int;
                        }

                        pr_info!("{:#x}, is the result of vm_mmap", result);

                        let file_pos = offest_in_file as i64;
                        veles_get_to_user(file, mmap_addr as c_ulong, file_pos, size as usize);
                    }

                    let stack = vm_mmap(core::ptr::null_mut(), 0, 0x200000, 3, 0x22, 0);

                    let top_stack = stack + 0x200000;

                    veles_start_thread((image_base + entry_point as u64) as usize, top_stack);
                }
            }
        }
    }
    0
}

struct Veles {}

static mut FMT: linux_binfmt = linux_binfmt {
    lh: [0; 16],
    module: core::ptr::null_mut(),
    load_binary: load_pe_binary,
};

impl kernel::Module for Veles {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        unsafe {
            __register_binfmt(&raw mut FMT as *mut linux_binfmt, 0);
        }

        Ok(Veles {})
    }
}

impl Drop for Veles {
    fn drop(&mut self) {
        unsafe {
            unregister_binfmt(&raw mut FMT as *mut linux_binfmt);
        }
    }
}
