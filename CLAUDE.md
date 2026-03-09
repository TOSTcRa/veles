# Chimera — Hybrid NT/Linux Kernel Compatibility Layer

## What This Is

Chimera is a project to build a Linux distribution with native Windows binary compatibility. Not through emulation (Wine), not through virtualization (VM) — through a hybrid kernel that natively understands both Linux (ELF/.so) and Windows (PE/.exe/.dll) binaries.

The CPU doesn't care about file formats. Chimera bridges the four layers between a Windows binary and the processor: binary format, syscalls, Win32 API, and graphics/drivers.

## Architecture

### Phase 1 — Kernel Module (Rust)
A loadable kernel module that adds NT syscall dispatch to the Linux kernel.

Components:
- **PE Loader** (`binfmt` handler) — registers PE format with the kernel, parses PE/COFF headers, maps sections into memory, resolves imports
- **NT Syscall Table** — dispatch table for NT syscalls (NtCreateFile, NtReadFile, NtWriteFile, NtQuerySystemInformation, etc.)
- **NT → Linux Translator** — translates NT syscall semantics to Linux equivalents (handle-based I/O → fd-based I/O, Unicode paths → UTF-8, backslash → forward slash, registry queries → filesystem reads)
- **NT Object Manager** — minimal implementation of the NT object namespace (\Device, \Registry, \BaseNamedObjects)
- **Process Environment** — PEB/TEB structures for NT processes, so Windows binaries find what they expect in memory

### Phase 2 — Userspace Support Layer
- **Registry Emulation** — file-backed Windows registry stored in ~/.chimera/registry/ as a hive file
- **Wine DLL Integration** — use Wine's reimplemented DLLs (kernel32, ntdll, user32, etc.) as the Win32 API layer on top of our kernel NT interface
- **Path Translation** — seamless mapping between Linux filesystem and Windows drive letters (C:\ → configurable mount point)

### Phase 3 — Driver Compatibility
- **WDM Subset** — enough of the Windows Driver Model to load basic drivers (needed for anti-cheat compatibility later)
- **DXGI/DirectX Bridge** — leverage DXVK/vkd3d-proton for graphics translation

### Phase 4 — Deep Compatibility
- **NtQuerySystemInformation** — full implementation of all info classes so inspection tools see a convincing NT environment
- **Kernel structure emulation** — SSDT, PEB/TEB, loaded module lists that look correct to introspective software
- **Certificate/signing infrastructure** — for software that validates system file signatures

## Technical Decisions

### Why Kernel Module, Not Userspace
- seccomp-bpf/ptrace userspace interception adds latency to every syscall — unacceptable for games
- binfmt_misc can launch our loader but the actual syscall path stays in userspace
- A kernel module gives true native syscall dispatch speed — NT syscalls handled at the same level as Linux syscalls
- This is what Longene tried and what WSL1 achieved (in reverse)

### Why Rust
- Linux kernel supports Rust modules since 6.1
- Memory safety critical for kernel code — one bug = kernel panic
- LO and ENI work primarily in Rust (see: vigil project)
- Better tooling, testing, and AI assistance compared to C kernel modules

### Why Not Just Use Wine
- Wine is userspace — every Windows API call goes through translation overhead
- Wine can't load Windows kernel drivers (.sys)
- Wine's architecture limits how deeply it can integrate with the system
- Chimera's kernel module + Wine DLLs = best of both worlds (native syscall speed + mature API coverage)

## Prior Art & References

- **Longene (Linux Unified Kernel)** — Chinese project ~2006, proved the concept works, abandoned due to team size not technical impossibility
- **ReactOS** — full NT kernel reimplementation, open source, 28 years of NT internals documentation
- **Wine** — 30+ years of Win32 API reimplementation, MIT licensed, we use their DLLs
- **WSL1** — Microsoft did the reverse: NT kernel translating Linux syscalls. Proof that cross-kernel syscall translation works at production quality
- **DXVK/vkd3d-proton** — DirectX → Vulkan translation, mature and performant

## Project Structure

```
chimera/
├── chimera-kernel/          # Rust kernel module
│   ├── src/
│   │   ├── lib.rs           # Module entry point, init/cleanup
│   │   ├── binfmt.rs        # PE binary format handler
│   │   ├── pe/
│   │   │   ├── mod.rs
│   │   │   ├── parser.rs    # PE/COFF header parsing
│   │   │   ├── loader.rs    # Section mapping, relocation
│   │   │   └── imports.rs   # Import table resolution
│   │   ├── nt/
│   │   │   ├── mod.rs
│   │   │   ├── syscalls.rs  # NT syscall dispatch table
│   │   │   ├── translate.rs # NT → Linux syscall translation
│   │   │   ├── objects.rs   # NT Object Manager
│   │   │   └── process.rs   # PEB/TEB, NT process structures
│   │   └── fs/
│   │       ├── mod.rs
│   │       └── paths.rs     # NT path → Linux path translation
│   └── Kbuild
├── chimera-registry/        # Registry emulation daemon
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── hive.rs          # Registry hive file format
│       └── keys.rs          # Standard registry keys/values
├── chimera-rt/              # Userspace runtime support
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── wine.rs          # Wine DLL loading/integration
│       └── env.rs           # Environment setup for NT processes
├── chimera-cli/             # CLI tool for managing chimera
│   ├── Cargo.toml
│   └── src/
│       └── main.rs          # chimera run, chimera status, chimera config
└── docs/
    ├── nt-syscalls.md       # NT syscall documentation and mapping
    ├── pe-format.md         # PE format reference
    └── architecture.md      # Detailed architecture decisions
```

## Kernel Requirements

The host kernel MUST be compiled with Rust support:
```
CONFIG_RUST=y
CONFIG_HAVE_RUST=y
```

Current status: `CONFIG_HAVE_RUST=y` but `CONFIG_RUST` not enabled.
Kernel source: `/usr/src/linux-6.18.12-gentoo-dist`
Kernel: Gentoo dist-kernel 6.18.12

### Steps to enable Rust in kernel:
1. Install `rust-src` component: `rustup component add rust-src`
2. Install `bindgen-cli`: `cargo install bindgen-cli`
3. Verify with: `make LLVM=1 rustavailable` in kernel source dir
4. Enable CONFIG_RUST=y in kernel config
5. Rebuild kernel

## Development Rules

### Optimization Over Speed
Always take the most optimal path, not the quickest or laziest. If two approaches both have flaws, synthesize a third that takes the best from each. Never cut corners on architecture — this is kernel code, mistakes crash the system.

### Transparency About Limitations
If ENI cannot perform an action (downloading, hardware testing, kernel compilation, system reboot), she MUST:
1. Clearly state what needs to be done
2. Provide the exact commands LO needs to run
3. Explain WHY those commands are needed
4. Warn about any risks (kernel panic, data loss, boot failure)

### Code Standards
- Every syscall translation must document the semantic differences between NT and Linux
- No silent failures — if a translation is imperfect or incomplete, log it explicitly
- Test with real Windows binaries, not just synthetic tests
- Comments explain the NT internals being emulated, with references to ReactOS/Wine source where applicable

### NT Syscall Implementation Priority
Start with the minimum set to run a basic .exe:
1. NtCreateFile / NtOpenFile / NtReadFile / NtWriteFile / NtClose
2. NtCreateProcess / NtTerminateProcess
3. NtAllocateVirtualMemory / NtFreeVirtualMemory / NtProtectVirtualMemory
4. NtQueryInformationProcess / NtQuerySystemInformation (basic classes)
5. NtCreateThread / NtTerminateThread
6. NtWaitForSingleObject / NtCreateEvent / NtSetEvent

This minimal set should be enough to run a statically-linked PE that does file I/O and basic process management.

## LO's Vision

This is not a hobby project that should "work good enough." The end goal is a Linux distribution where Windows and Linux software coexist as first-class citizens. Where `./game.exe` runs at native speed next to `./server` without either knowing or caring about the other.

The path there is long. But every line of code should be written as if the final product is shipping tomorrow.
