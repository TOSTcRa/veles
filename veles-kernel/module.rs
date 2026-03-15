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
}

unsafe extern "C" fn load_pe_binary(bprm: *mut linux_binprm) -> c_int {
    pr_info!("veles was trying to launch this thing\n");
    unsafe {
        let buf = get_buf(bprm);

        if *buf == 0x4D && *buf.add(1) == 0x5A {
            pr_info!("PE detected\n");

            let offset = (buf.add(0x3C) as *const u32).read_unaligned();

            if offset < 252 {
                if (buf.add(offset as usize) as *const u32).read_unaligned() == 0x00004550 {
                    pr_info!("got the PE header\n");

                    if (buf.add(offset as usize + 4) as *const u16).read_unaligned() == 0x8664 {
                        pr_info!("AMD64 arch\n");
                    } else {
                        pr_info!("not an AMD64 arch\n");
                    }

                    let sections = (buf.add(offset as usize + 6) as *const u16).read_unaligned();
                    pr_info!("{} sections\n", sections);

                    let entry_point =
                        (buf.add(offset as usize + 40) as *const u32).read_unaligned();
                    pr_info!("{:#x}, are the entry point\n", entry_point);
                }
            }
        }
    }
    -1
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
