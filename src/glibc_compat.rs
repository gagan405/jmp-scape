use core::marker::PhantomData;

// glibc `__jmp_buf` is architecture-specific (see e.g. sysdeps/*/bits/setjmp.h).
// Linux aarch64 uses `unsigned long long __jmp_buf[22]`; x86_64 uses `long long[8]`.
// This module is only used as `struct_defs` on Linux (`lib.rs`); other OSes still
// compile it with a placeholder length.
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const JMP_BUF_U64_LEN: usize = 22;
#[cfg(all(target_os = "linux", not(target_arch = "aarch64")))]
const JMP_BUF_U64_LEN: usize = 8;
#[cfg(not(target_os = "linux"))]
const JMP_BUF_U64_LEN: usize = 8;

/// `JmpBufFields` are the accessible fields when viewed via a JmpBuf pointer.
/// But also: You shouldn't be poking at these!
#[repr(C)]
pub struct JmpBufFields {
    _buf: [u64; JMP_BUF_U64_LEN],
    _neither_send_nor_sync: PhantomData<*const u8>,
}

/// `SigJmpBufFields` are the accessible fields when viewed via a SigJmpBuf pointer.
/// But also: You shouldn't be poking at these!
#[repr(C)]
pub struct SigJmpBufFields {
    // This *must* be the first field. We allow `SigJmpBuf` to be transmuted to
    // a `JmpBuf` and then back again depending on the host libc. (e.g. glibc
    // has setjmp as a shallow wrapper around sigsetjmp, and will write to
    // fields past the `__jmp_buf`).
    __jmp_buf: JmpBufFields,
    __mask_was_saved: isize,
    __saved_mask: libc::sigset_t,
}

/// This is the type you use to allocate a JmpBuf on the stack.
/// (Glibc puns the two.)
pub type JmpBufStruct = SigJmpBufFields;

/// This is the type you use to allocate a SigJmpBuf on the stack.
pub type SigJmpBufStruct = SigJmpBufFields;
