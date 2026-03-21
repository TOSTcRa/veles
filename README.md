# Veles

**Hybrid NT/Linux kernel compatibility layer** -
run Windows binaries natively on Linux, without emulation or virtualization.

Named after the Slavic god who walks between worlds.

## The Idea

The CPU doesn't care about file formats.
A Windows `.exe` and a Linux ELF binary both compile down to the same x86 instructions.
The difference is in four layers above the hardware: binary format (PE vs ELF),
syscalls (NT vs Linux), API (Win32 vs POSIX), and graphics/drivers (DirectX vs Vulkan).

Veles bridges these layers at the kernel level.
Not through userspace translation (Wine), not through a VM -
through a loadable kernel module that teaches Linux to natively understand
NT syscalls and PE binaries.

The end goal: a Linux system where `./game.exe` runs at native speed next to `./server`,
and neither knows nor cares about the other.

## How It Works

```
┌─────────────────────────────────────────────────┐
│              Userspace Applications             │
│         ELF binaries    PE binaries (.exe)      │
├────────────────────┬────────────────────────────┤
│   Linux syscalls   │   NT syscalls              │
│   (native)         │   (veles kernel module)    │
├────────────────────┴────────────────────────────┤
│                  Linux Kernel                   │
│           + veles.ko (Rust module)              │
├─────────────────────────────────────────────────┤
│                   Hardware                      │
└─────────────────────────────────────────────────┘
```

### Kernel Module (Phase 1 — in progress)

The core is a Rust kernel module (`veles.ko`)
that adds NT compatibility to the Linux kernel:

- **PE Loader** — `binfmt` handler that recognizes PE/COFF binaries,
  parses headers, maps sections into memory, and resolves imports
- **NT Syscall Table** — dispatch table for
  NT syscalls (`NtCreateFile`, `NtReadFile`, `NtWriteFile`, etc.)
- **NT → Linux Translator** — maps NT syscall
  semantics to Linux equivalents (handles → fds, Unicode → UTF-8,
  backslash paths → forward slash)
- **NT Object Manager** — minimal implementation of the
  NT object namespace (`\Device`, `\Registry`, `\BaseNamedObjects`)
- **Process Environment** — PEB/TEB structures so Windows binaries
  find what they expect in memory

### Userspace Support (Phase 2)

- Registry emulation backed by files (`~/.veles/registry/`)
- Wine DLL integration — reuse Wine's mature Win32 API
  implementations (`kernel32`, `ntdll`, `user32`) on top of
  Veles's native NT syscall layer
- Transparent path translation (drive letters ↔ Linux mount points)

### Driver & Graphics Compatibility (Phase 3)

- WDM (Windows Driver Model) subset for basic driver loading
- DirectX → Vulkan via DXVK/vkd3d-proton

### Deep Compatibility (Phase 4)

- Full `NtQuerySystemInformation` implementation
- Kernel structure emulation (SSDT, PEB/TEB, loaded module lists)
  for introspective software
- Certificate/signing infrastructure for software that validates system file signatures

## Current Status

**Stage:** Early kernel module development —
PE parser can read headers, sections, architecture, and entry points.

**Progress:**

- [x] Kernel module loads and registers PE binfmt handler
- [x] MZ magic byte detection
- [x] PE/COFF header parsing (architecture, sections, entry point)
- [x] Section mapping (address, size, offset)
- [ ] Import table resolution
- [ ] NT syscall dispatch table
- [ ] NT → Linux syscall translation
- [ ] NT Object Manager
- [ ] PEB/TEB process environment

## Building

### Requirements

- Linux kernel 6.1+ with `CONFIG_RUST=y`
- Rust toolchain (rustc 1.94+)
- LLVM/Clang 21+
- Kernel headers for your running kernel

### Kernel Module

```bash
cd veles-kernel
sudo PATH="$HOME/.cargo/bin:$PATH" HOME="$HOME" RUSTUP_HOME="$HOME/.rustup" \
    make -C /usr/src/linux M=$(pwd) LLVM=1 modules
```

### Loading

```bash
sudo insmod veles-kernel/veles.ko
# Check dmesg for output
dmesg | tail
```

## Why Not Just Use Wine?

Wine is excellent — 30+ years of Win32 API reimplementation.
But it has architectural limits:

| | Wine | Veles |
|-----------------------|-----------------------------------|------------------------|
| Syscall path | Userspace translation | Kernel-level dispatch |
| Kernel drivers (.sys) | Not supported | WDM subset (planned) |
| Per-syscall overhead | Translation + context switches | Native speed |
| Integration depth | Isolated prefix | First-class citizen |

Veles doesn't replace Wine — it uses Wine's DLLs as the Win32 API layer, while providing native
NT syscall dispatch underneath. Best of both worlds.

## Prior Art

This isn't the first attempt at kernel-level NT compatibility.
Veles builds on lessons from:

- **[Longene](http://www.longene.org/)** (Linux Unified Kernel) — proved the concept ~2006,
  abandoned due to team size, not technical impossibility
- **[ReactOS](https://reactos.org/)** — 28 years of open-source
  NT kernel reimplementation and documentation
- **[Wine](https://www.winehq.org/)** — 30+ years of Win32 API
  reimplementation (MIT licensed)
- **WSL1** — Microsoft did the reverse (NT kernel translating Linux syscalls),
  proving cross-kernel syscall translation works at production quality
- **[DXVK](https://github.com/doitsujin/dxvk) / [vkd3d-proton](https://github.com/HansKristian-Work/vkd3d-proton)** —
  mature DirectX → Vulkan translation

## Project Structure

```
veles/
├── src/                    # Userspace Rust binary (CLI placeholder)
│   └── main.rs
├── veles-kernel/           # Rust kernel module
│   ├── Kbuild
│   └── *.c / *.o           # Generated module files
├── Cargo.toml
└── CLAUDE.md               # Development context and architecture notes
```

## Tech Stack

- **Language:** Rust (kernel module + userspace)
- **Kernel:** Linux 6.18 (Gentoo, custom-built with Rust support)
- **Build:** Kbuild + LLVM
- **Target:** x86_64

## License

TBD

---

*Slavic roots, world-class ambition.*
