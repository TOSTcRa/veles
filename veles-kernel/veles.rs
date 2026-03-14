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

struct Veles {}

impl kernel::Module for Veles {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("module was loaded");

        Ok(Veles {})
    }
}

impl Drop for Veles {
    fn drop(&mut self) {
        pr_info!("modules was unloaded\n");
    }
}
