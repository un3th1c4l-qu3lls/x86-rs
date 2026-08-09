#![no_std]

/*

// Should make separate crate for cpuid parsing and leaf definitions
        // please do
        // for leaf, subleaf support, check cpuid crate plz xD
        NOTE : there was a 16 bit protected mode...
        pub const GATE_16_INTERRUPT: u8 = 0x06;
pub const GATE_16_TRAP: u8 = 0x07;

pub const TSS_16_AVAILABLE: u8 = 0x01; // 0b00001
            pub const TSS_16_BUSY: u8 = 0x03; // 0b00011
*/

pub mod word {
    /*!
    # .code16

    Invalid opcodes will generate a #UD exception.
    */

    pub mod registers {

        /*!
        # x86-16 Registers

        ## General Purpose

        - **ax** : accumulator
        - **bx** : base
        - **cx** : counter
        - **dx** : data

        ## Indices

        - **di** : destination
        - **si** : source

        ## Pointers

        - **bp** : base
        - **ip** : instruction
        - **sp** : stack

        ## Segments

        - **cs** : code
        - **ds** : data
        - **es** : extension
        - **ss** : stack

        ## Status

        - **flags** : flags
        */

        pub use data::{ax, bx, cx, dx};
        pub use index::{di, si};
        pub use pointer::{bp, ip, sp};
        pub use segment::{cs, ds, es, ss};
        pub use status::flags;

        pub mod data {
            pub mod ax {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, ax",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov ax, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod bx {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, bx",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov bx, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod cx {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, cx",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov cx, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dx {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, dx",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov dx, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod index {

            pub mod di {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, di",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov di, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod si {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, si",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov si, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod pointer {

            pub mod bp {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, bp",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov bp, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod ip {
                /*!
                # Instruction Pointer

                The #IP register can be read, but not written to using this module.
                */
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "call 2f",
                            "2:",
                            "pop {0:x}",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }
            }

            pub mod sp {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, sp",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov sp, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod segment {

            pub mod cs {
                /*!
                # Code Segment

                The #CS register can be read, but not written to using this module.
                */
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, cs",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }
            }

            pub mod ds {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, ds",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov ds, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod es {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, es",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov es, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod ss {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov {0:x}, ss",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "mov ss, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod status {

            pub mod flags {

                pub const CF: u16 = 0x0001; // Carry
                pub const PF: u16 = 0x0004; // Parity
                pub const AF: u16 = 0x0010; // Auxiliary Carry
                pub const ZF: u16 = 0x0040; // Zero
                pub const SF: u16 = 0x0080; // Sign
                pub const TF: u16 = 0x0100; // Trap
                pub const IF: u16 = 0x0200; // Interrupt
                pub const DF: u16 = 0x0400; // Direction
                pub const OF: u16 = 0x0800; // Overflow
                pub const IOPL: u16 = 0x3000; // I/O Privilege Level
                pub const NT: u16 = 0x4000; // Nested Task

                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let flags: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "pushf",
                            "pop {0:x}",
                            out(reg) flags,
                            options(nostack)
                        );
                    }
                    flags
                }

                #[inline(always)]
                pub unsafe fn write(flags: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code16",
                            "push {0:x}",
                            "popf",
                            in(reg) flags,
                            options(nostack)
                        );
                    }
                }
            }
        }

        pub struct Registers {
            pub ax: u16,
            pub bx: u16,
            pub cx: u16,
            pub dx: u16,

            pub di: u16,
            pub si: u16,

            pub bp: u16,
            pub ip: u16,
            pub sp: u16,

            pub cs: u16,
            pub ds: u16,
            pub es: u16,
            pub ss: u16,

            pub flags: u16,
        }

        impl Default for Registers {
            fn default() -> Self {
                Self {
                    ax: 0,
                    bx: 0,
                    cx: 0,
                    dx: 0,
                    di: 0,
                    si: 0,
                    bp: 0,
                    ip: 0,
                    sp: 0,
                    cs: 0,
                    ds: 0,
                    es: 0,
                    ss: 0,
                    flags: 0,
                }
            }
        }
    }

    pub mod intrinsics {

        #[inline(always)]
        pub unsafe fn cli() {
            unsafe {
                core::arch::asm!(".code16", "cli", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn sti() {
            unsafe {
                core::arch::asm!(".code16", "sti", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn stc() {
            unsafe {
                core::arch::asm!(".code16", "stc", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn clc() {
            unsafe {
                core::arch::asm!(".code16", "clc", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn cmc() {
            unsafe { core::arch::asm!(".code16", "cmc", options(nostack)) }
        }

        #[inline(always)]
        pub unsafe fn cld() {
            unsafe {
                core::arch::asm!(".code16", "cld", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn std() {
            unsafe {
                core::arch::asm!(".code16", "std", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn hlt() {
            unsafe {
                core::arch::asm!(".code16", "hlt", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn nop() {
            unsafe {
                core::arch::asm!(".code16", "nop", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn wait() {
            unsafe {
                core::arch::asm!(".code16", "wait", options(nostack));
            }
        }
    }

    pub mod stack {

        #[inline(always)]
        pub unsafe fn popw() -> u16 {
            let value: u16;
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "pop {0:x}",
                    out(reg) value,
                    options(preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn pushw(value: u16) {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "push {0:x}",
                    in(reg) value,
                    options(preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn popa() {
            unsafe {
                core::arch::asm!(".code16", "popa", options(preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn pusha() {
            unsafe {
                core::arch::asm!(".code16", "pusha", options(preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn popf() {
            unsafe {
                core::arch::asm!(".code16", "popf", options(preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn pushf() {
            unsafe {
                core::arch::asm!(".code16", "pushf", options(preserves_flags));
            }
        }
    }

    pub mod interrupts {
        /*!
        # Interrupt Frame

        On interrupt triggers (`int`), the CPU pushes :
        - #FLAGS,
        - #CS,
        - #IP,
        onto the stack, popped when `iret` is executed.
        */

        pub struct InterruptFrame {
            pub flags: u16,
            pub cs: u16,
            pub ip: u16,
        }

        #[inline(always)]
        pub unsafe fn int(vector: u8) {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "int {0}",
                    in(reg_byte) vector,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn iret() -> ! {
            unsafe { core::arch::asm!(".code16", "iret", options(nostack, preserves_flags)) }

            unreachable!();
        }
    }

    pub mod io {

        #[inline(always)]
        pub unsafe fn outb_reg(port: u16, value: u8) {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "mov dx, {0:x}",
                    "mov al, {1}",
                    "out dx, al",
                    in(reg) port,
                    in(reg_byte) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn inb_reg(port: u16) -> u8 {
            let value: u8;
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "mov dx, {0:x}",
                    "in al, dx",
                    "mov {1}, al",
                    in(reg) port,
                    out(reg_byte) value,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[macro_export]
        macro_rules! outb_imm_16 {
			($port:expr, $value:expr) => {

				unsafe {
					core::arch::asm!(
						".code16",
						"mov al, {1}",
						"out {0}, al",
						const $port,
						in(reg_byte) $value,
						options(nostack, preserves_flags)
					);
				}

			};
		}

        #[macro_export]
        macro_rules! inb_imm_16 {
			($port:expr) => {

				let value: u8;
				unsafe {
					core::arch::asm!(
						".code16",
						"in al, {0}",
						"mov {1}, al",
						const $port,
						out(reg_byte) value,
						options(nostack, preserves_flags)
					);
				}

				value
			};
		}

        #[inline(always)]
        pub unsafe fn outw_reg(port: u16, value: u16) {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "mov dx, {0:x}",
                    "mov ax, {1:x}",
                    "out dx, ax",
                    in(reg) port,
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn inw_reg(port: u16) -> u16 {
            let value: u16;
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "mov dx, {0:x}",
                    "in ax, dx",
                    "mov {1:x}, ax",
                    in(reg) port,
                    out(reg) value,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[macro_export]
        macro_rules! outw_imm_16 {
			($port:expr, $value:expr) => {

				unsafe {
					core::arch::asm!(
						".code16",
						"mov ax, {1:x}",
						"out {0}, ax",
						const $port,
						in(reg) $value,
						options(nostack, preserves_flags)
					);
				}

			};
		}

        #[macro_export]
        macro_rules! inw_imm_16 {
			($port:expr) => {

				let value: u16;

				unsafe {
					core::arch::asm!(
						".code16",
						"in ax, {0}",
						"mov {1:x}, ax",
						const $port,
						out(reg) value,
						options(nostack, preserves_flags)
					);
				}

				value
			};
		}
    }

    #[repr(C, packed)]
    pub struct FarPtr {
        pub offset: u16,
        pub segment: u16,
    }

    impl FarPtr {
        pub fn new(segment: u16, offset: u16) -> Self {
            Self {
                offset: offset,
                segment: segment,
            }
        }

        pub fn to_physical(&self) -> u32 {
            ((self.segment as u32) << 4) + (self.offset as u32)
        }

        pub const fn from_physical(address: u32) -> Self {
            Self {
                segment: (address >> 4) as u16,
                offset: (address & 0xF) as u16,
            }
        }
    }

    pub mod memory {

        #[inline(always)]
        pub unsafe fn loadb(ptr: super::FarPtr) -> u8 {
            let value: u8;
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov {0}, [{2:x}]",
                    "pop ds",
                    out(reg_byte) value,
                    in(reg) ptr.segment,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn storeb(ptr: super::FarPtr, value: u8) {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov [{2:x}], {0}",
                    "pop ds",
                    in(reg_byte) value,
                    in(reg) ptr.segment,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn loadw(ptr: super::FarPtr) -> u16 {
            let value: u16;
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov {0:x}, [{2:x}]",
                    "pop ds",
                    out(reg) value,
                    in(reg) ptr.segment,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn storew(ptr: super::FarPtr, value: u16) {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov [{2:x}], {0:x}",
                    "pop ds",
                    in(reg) value,
                    in(reg) ptr.segment,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }
        }
    }

    pub mod control {
        #[inline(always)]
        pub unsafe fn call_reg(value: u16) {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "call {0:x}",
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[macro_export]
        macro_rules! call_imm_16 {
            ($value:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code16",
                        "call {0}",
                        const $value,
                        options(nostack, preserves_flags)
                    );
                }
            };
        }

        #[inline(always)]
        pub unsafe fn call_mem(address: u16) {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "call [{0:x}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[macro_export]
        macro_rules! callfar_imm_16 {
            ($segment:expr, $offset:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code16",
                        "call far {0}:{1}",
                        const $segment,
                        const $offset,
                        options(nostack, preserves_flags)
                    );
                }
            };
        }

        #[inline(always)]
        pub unsafe fn callfar_mem(address: u16) {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "call far [{0:x}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn jmp_reg(offset: u16) -> ! {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "jmp {0:x}",
                    in(reg) offset,
                    options(nostack, preserves_flags)
                );
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! jmp_imm_16 {
            ($offset:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code16",
                        "jmp {0}",
                        const $offset,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn jmp_mem(address: u16) -> ! {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "jmp [{0:x}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
            unreachable!();
        }

        #[inline(always)]
        pub unsafe fn jmpfar_mem(address: u16) -> ! {
            unsafe {
                core::arch::asm!(
                    ".code16",
                    "jmp far [{0:x}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! jmpfar_imm_16 {
            ($segment:expr, $offset:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code16",
                        "jmp far {0}:{1}",
                        const $segment,
                        const $offset,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn ret() -> ! {
            unsafe {
                core::arch::asm!(".code16", "ret", options(nostack, preserves_flags));
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! ret_imm_16 {
            ($value:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code16",
                        "ret {0:x}",
                        const $value,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn retf() -> ! {
            unsafe {
                core::arch::asm!(".code16", "retf", options(nostack, preserves_flags));
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! retf_imm_16 {
            ($value:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code16",
                        "retf {0:x}",
                        const $value,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }
    }
}

pub mod dword {
    //! .code32

    pub mod registers {

        /*!
            # x86-32 Registers

            ## General Purpose

            - **eax** : accumulator
            - **ebx** : base
            - **ecx** : counter
            - **edx** : data

            ## Indices

            - **edi** : destination
            - **esi** : source

            ## Pointers

            - **ebp** : base
            - **eip** : instruction
            - **esp** : stack

            ## Segments

            - **cs** : code
            - **ds** : data
            - **es** : extension
            - **fs** : extension
            - **gs** : extension
            - **ss** : stack

            ## Status

            - **eflags** : flags

            ## Control

            - **cr0** : control register 0
            - **cr2** : control register 2
            - **cr3** : control register 3
            - **cr4** : control register 4

            ## Debug

            - **dr0 - dr3** : debug address
            - **dr6** : debug status
            - **dr7** : debug control
        */

        pub use control::{cr0, cr2, cr3, cr4};
        pub use data::{eax, ebx, ecx, edx};
        pub use debug::{dr0, dr1, dr2, dr3, dr6, dr7};
        pub use index::{edi, esi};
        pub use pointer::{ebp, eip, esp};
        pub use segment::{cs, ds, es, ss};
        pub use status::eflags;

        pub mod data {
            pub mod eax {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, eax",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov eax, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod ebx {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, ebx",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov ebx, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod ecx {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, ecx",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov ecx, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod edx {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, edx",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov edx, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod index {

            pub mod edi {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, edi",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov edi, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod esi {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, esi",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov esi, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod pointer {

            pub mod ebp {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, ebp",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov ebp, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod eip {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "call 2f",
                            "2:",
                            "pop {0:e}",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }
            }

            pub mod esp {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, esp",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov esp, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod segment {

            pub mod cs {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:x}, cs",
                            out(reg) value,
                            options(nostack, preserves_flags),
                        );
                    }
                    value
                }
            }

            pub mod ds {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:x}, ds",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov ds, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod es {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:x}, es",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov es, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod fs {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:x}, fs",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov fs, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod gs {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:x}, gs",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov gs, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod ss {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:x}, ss",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov ss, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod status {

            pub mod eflags {

                pub const CF: u32 = 0x0001; // Carry
                pub const PF: u32 = 0x0004; // Parity
                pub const AF: u32 = 0x0010; // Auxiliary Carry
                pub const ZF: u32 = 0x0040; // Zero
                pub const SF: u32 = 0x0080; // Sign
                pub const TF: u32 = 0x0100; // Trap
                pub const IF: u32 = 0x0200; // Interrupt
                pub const DF: u32 = 0x0400; // Direction
                pub const OF: u32 = 0x0800; // Overflow
                pub const IOPL: u32 = 0x3000; // I/O Privilege Level
                pub const NT: u32 = 0x4000; // Nested Task

                pub const RF: u32 = 0x10000; // Resume
                pub const VM: u32 = 0x20000; // Virtual 8086 Mode
                pub const AC: u32 = 0x40000; // Alignment Check
                pub const VIF: u32 = 0x80000; // Virtual Interrupt
                pub const VIP: u32 = 0x100000; // Virtual Interrupt Pending
                pub const ID: u32 = 0x200000; // ID

                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let eflags: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "pushf",
                            "pop {0:e}",
                            out(reg) eflags,
                            options(nostack)
                        );
                    }
                    eflags
                }

                #[inline(always)]
                pub unsafe fn write(eflags: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "push {0:e}",
                            "popf",
                            in(reg) eflags,
                            options(nostack)
                        );
                    }
                }
            }
        }

        pub mod control {
            pub mod cr0 {

                /// Protection enable.
                pub const PE: u32 = 0x1;
                /// Paging enable.
                pub const PG: u32 = 0x8000_0000;
                /// Write protect.
                pub const WP: u32 = 0x0001_0000;
                /// Numeric error.
                pub const NE: u32 = 0x0000_0020;
                /// Extension type.
                pub const ET: u32 = 0x0000_0010;
                /// Task switched.
                pub const TS: u32 = 0x0000_0008;
                /// Emulate (x87 FPU).
                pub const EM: u32 = 0x0000_0004;
                /// Monitor coprocessor.
                pub const MP: u32 = 0x0000_0002;
                /// Cache disable.
                pub const CD: u32 = 0x4000_0000;
                /// No write-through.
                pub const NW: u32 = 0x2000_0000;
                /// Alignment check.
                pub const AM: u32 = 0x0004_0000;

                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, cr0",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov cr0, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod cr2 {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, cr2",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov cr2, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod cr3 {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, cr3",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov cr3, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod cr4 {
                /// Virtual 8086 extensions.
                pub const VME: u32 = 0x0000_0001;
                /// Protected virtual interrupts.
                pub const PVI: u32 = 0x0000_0002;
                /// Time stamp disable.
                pub const TSD: u32 = 0x0000_0004;
                /// Debugging extensions
                pub const DE: u32 = 0x0000_0008;
                /// Page size extension.
                pub const PSE: u32 = 0x0000_0010;
                /// Physical address extension.
                pub const PAE: u32 = 0x0000_0020;
                /// Machine check exception.
                pub const MCE: u32 = 0x0000_0040;
                /// Page global enable.
                pub const PGE: u32 = 0x0000_0080;
                ///
                pub const PCE: u32 = 0x0000_0100;
                ///
                pub const OSFXSR: u32 = 0x0000_0200;
                ///
                pub const OSXSAVE: u32 = 0x0000_0400;
                ///
                pub const UMIP: u32 = 0x0000_0800;
                ///
                pub const LA57: u32 = 0x0000_1000;
                ///
                pub const VMXE: u32 = 0x0000_2000;
                ///
                pub const SMXE: u32 = 0x0000_4000;
                ///
                pub const FSGSBASE: u32 = 0x0001_0000;
                ///
                pub const PCID: u32 = 0x0002_0000;
                ///
                pub const XSAVE: u32 = 0x0004_0000;
                ///
                pub const SMEP: u32 = 0x0010_0000;
                ///
                pub const SMAP: u32 = 0x0020_0000;
                ///
                pub const PKE: u32 = 0x0040_0000;
                ///
                pub const CET: u32 = 0x0080_0000;
                ///
                pub const PKS: u32 = 0x0100_0000;

                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, cr4",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov cr4, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod debug {
            pub mod dr0 {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, dr0",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov dr0, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dr1 {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, dr1",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov dr1, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dr2 {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, dr2",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov dr2, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dr3 {
                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, dr3",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov dr3, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dr6 {
                ///
                pub const B0: u32 = 0x0000_0001;
                ///
                pub const B1: u32 = 0x0000_0002;
                ///
                pub const B2: u32 = 0x0000_0004;
                ///
                pub const B3: u32 = 0x0000_0008;
                ///
                pub const B4: u32 = 0x0000_0010;
                ///
                pub const B5: u32 = 0x0000_0020;
                ///
                pub const B6: u32 = 0x0000_0040;
                ///
                pub const B7: u32 = 0x0000_0080;
                ///
                pub const BD: u32 = 0x0000_0200;
                ///
                pub const BS: u32 = 0x0000_4000;
                ///
                pub const BT: u32 = 0x0000_8000;

                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, dr6",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov dr6, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dr7 {

                pub const L0: u32 = 0x0000_0001;
                pub const G0: u32 = 0x0000_0002;
                pub const L1: u32 = 0x0000_0004;
                pub const G1: u32 = 0x0000_0008;
                pub const L2: u32 = 0x0000_0010;
                pub const G2: u32 = 0x0000_0020;
                pub const L3: u32 = 0x0000_0040;
                pub const G3: u32 = 0x0000_0080;

                ///
                pub const LE: u32 = 0x0000_0100;
                ///
                pub const GE: u32 = 0x0000_0200;
                ///
                pub const GD: u32 = 0x0000_0400;

                #[inline(always)]
                pub unsafe fn read() -> u32 {
                    let value: u32;
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov {0:e}, dr7",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u32) {
                    unsafe {
                        core::arch::asm!(
                            ".code32",
                            "mov dr7, {0:e}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod msr {
            /// Extended Feature Enable Register
            pub const IA32_EFER: u32 = 0xC0000080;
            /// System Call Target Address
            pub const IA32_STAR: u32 = 0xC0000081;
            /// Long STAR
            pub const IA32_LSTAR: u32 = 0xC0000082;
            /// Compatibility STAR
            pub const IA32_CSTAR: u32 = 0xC0000083;
            /// SYSCALL/SYSRET Flag Mask
            pub const IA32_SYSCALL_MASK: u32 = 0xC0000084;
            /// FS Base
            pub const IA32_FS_BASE: u32 = 0xC0000100;
            /// GS Base
            pub const IA32_GS_BASE: u32 = 0xC0000101;
            /// Kernel GS Base
            pub const IA32_KERNEL_GS_BASE: u32 = 0xC0000102;
            /// Auxiliary TSC
            pub const IA32_TSC_AUX: u32 = 0xC0000103;
            /// SYSENTER CS
            pub const IA32_SYSENTER_CS: u32 = 0x00000174;
            /// SYSENTER ESP
            pub const IA32_SYSENTER_ESP: u32 = 0x00000175;
            /// SYSENTER EIP
            pub const IA32_SYSENTER_EIP: u32 = 0x00000176;
            /// Debug Control
            pub const IA32_DEBUGCTL: u32 = 0x000001D9;
            /// Last Branch From IP
            pub const IA32_LASTBRANCHFROMIP: u32 = 0x000001DB;
            /// Last Branch To IP
            pub const IA32_LASTBRANCHTOIP: u32 = 0x000001DC;
            /// Last Interrupt From IP
            pub const IA32_LASTINTFROMIP: u32 = 0x000001DD;
            /// Last Interrupt to IP
            pub const IA32_LASTINTTOIP: u32 = 0x000001DE;
            /// Process Address Space ID
            pub const IA32_PASID: u32 = 0x00000D93;
            /// Power Control
            pub const IA32_POWER_CTL: u32 = 0x000001FC;
            /// Speculation Control
            pub const IA32_SPEC_CTRL: u32 = 0x00000048;
            /// Prediction Command
            pub const IA32_PRED_CMD: u32 = 0x00000049;
            /// Architecture Capabilities
            pub const IA32_ARCH_CAPABILITIES: u32 = 0x0000010A;
            /// Flush Command
            pub const IA32_FLUSH_CMD: u32 = 0x0000010B;
            /// TSX Control
            pub const IA32_TSX_CTRL: u32 = 0x00000122;
            /// Microcode Update Control
            pub const IA32_MCU_OPT_CTRL: u32 = 0x00000123;

            /// Machine Check Global Capability
            pub const IA32_MCG_CAP: u32 = 0x00000179;
            /// Machine Check Global Status
            pub const IA32_MCG_STATUS: u32 = 0x0000017A;
            /// Machine Check Global Control
            pub const IA32_MCG_CTL: u32 = 0x0000017B;
            /// Machine Check Extended Control
            pub const IA32_MCG_EXT_CTL: u32 = 0x000004D0;
            /// Machine Check Bank 0 Control
            pub const IA32_MC0_CTL: u32 = 0x00000400;
            /// Machine Check Bank 0 Status
            pub const IA32_MC0_STATUS: u32 = 0x00000401;
            /// Machine Check Bank 0 Address
            pub const IA32_MC0_ADDR: u32 = 0x00000402;
            /// Machine Check Bank 0 Miscellaneous
            pub const IA32_MC0_MISC: u32 = 0x00000403;

            /// Performance Counter
            pub const IA32_PERFCTR0: u32 = 0x000000C1;
            /// PEBS Enable
            pub const IA32_PEBS_ENABLE: u32 = 0x000003F1;
            /// PEBS Base
            pub const IA32_PEBS_BASE: u32 = 0x000003F4;
            /// DS Area
            pub const IA32_DS_AREA: u32 = 0x00000600;
            /// Performance Capabilities
            pub const IA32_PERF_CAPABILITIES: u32 = 0x00000345;
            /// Processor Trace Control
            pub const IA32_RTIT_CTL: u32 = 0x00000570;
            /// Processor Trace Status
            pub const IA32_RTIT_STATUS: u32 = 0x00000571;
            /// Processor Trace Output Base
            pub const IA32_RTIT_OUTPUT_BASE: u32 = 0x00000560;
            /// Processor Trace Output Mask
            pub const IA32_RTIT_OUTPUT_MASK: u32 = 0x00000561;

            /// MTRR Capability
            pub const IA32_MTRRCAP: u32 = 0x000000FE;
            /// MTRR Default Type
            pub const IA32_MTRR_DEF_TYPE: u32 = 0x000002FF;
            /// MTRR Physical Mask
            pub const IA32_MTRR_PHYSMASK0: u32 = 0x00000200;
            /// MTRR Physical Base
            pub const IA32_MTRR_PHYSBASE0: u32 = 0x00000201;
            /// Fixed MTRR 64KB
            pub const IA32_MTRR_FIX64K_00000: u32 = 0x00000250;
            /// Fixed MTRR 16KB
            pub const IA32_MTRR_FIX16K_80000: u32 = 0x00000258;
            /// Fixed MTRR 4KB
            pub const IA32_MTRR_FIX4K_C0000: u32 = 0x00000268;
            /// Page Attribute Table
            pub const IA32_CR_PAT: u32 = 0x00000277;

            /// Package C3 Residency
            pub const IA32_PKG_C3_RESIDENCY: u32 = 0x000003F8;
            /// Package C6 Residency
            pub const IA32_PKG_C6_RESIDENCY: u32 = 0x000003F9;
            /// Core C3 Residency
            pub const IA32_CORE_C3_RESIDENCY: u32 = 0x000003FC;
            /// Core C6 Residency
            pub const IA32_CORE_C6_RESIDENCY: u32 = 0x000003FD;
            /// Package C8 Residency
            pub const IA32_PKG_C8_RESIDENCY: u32 = 0x00000630;

            /// FRED Stack Pointer for Level 0
            pub const IA32_FRED_RSP0: u32 = 0x000001CC;
            /// FRED Stack Pointer for Level 1
            pub const IA32_FRED_RSP1: u32 = 0x000001CD;
            /// FRED Stack Pointer for Level 2
            pub const IA32_FRED_RSP2: u32 = 0x000001CE;
            /// FRED Stack Pointer for Level 3
            pub const IA32_FRED_RSP3: u32 = 0x000001CF;
            /// FRED Stack Levels
            pub const IA32_FRED_STKLVLS: u32 = 0x000001D0;
            /// FRED Shadow Stack Pointer for Level 0
            pub const IA32_FRED_SSP0: u32 = 0x000001D1;
            /// FRED Configuration
            pub const IA32_FRED_CONFIG: u32 = 0x000001D4;

            #[inline(always)]
            pub unsafe fn read(msr: u32) -> u64 {
                let (high, low): (u32, u32);
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov ecx,{0:e}",
                        "rdmsr",
                        "mov {1:e},edx",
                        "mov {2:e},eax",
                        in(reg) msr,
                        out(reg) high,
                        out(reg) low,
                        options(nostack, preserves_flags)
                    );
                }
                ((high as u64) << 32) | (low as u64)
            }

            #[inline(always)]
            pub unsafe fn write(msr: u32, value: u64) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov edx:{0:e}",
                        "mov eax,{1:e}",
                        "mov ecx,{2:e}",
                        "wrmsr",
                        in(reg) (value >> 32) as u32,
                        in(reg) value as u32,
                        in(reg) msr,
                        options(nostack, preserves_flags)
                    );
                }
            }
        }

        pub struct Registers {
            pub eax: u32,
            pub ebx: u32,
            pub ecx: u32,
            pub edx: u32,

            pub edi: u32,
            pub esi: u32,

            pub ebp: u32,
            pub eip: u32,
            pub esp: u32,

            pub cs: u16,
            pub ds: u16,
            pub es: u16,
            pub fs: u16,
            pub gs: u16,
            pub ss: u16,

            pub eflags: u32,

            pub cr0: u32,
            pub cr2: u32,
            pub cr3: u32,
            pub cr4: u32,

            pub dr0: u32,
            pub dr1: u32,
            pub dr2: u32,
            pub dr3: u32,
            pub dr6: u32,
            pub dr7: u32,
        }

        impl Default for Registers {
            fn default() -> Self {
                Self {
                    eax: 0,
                    ebx: 0,
                    ecx: 0,
                    edx: 0,
                    edi: 0,
                    esi: 0,
                    ebp: 0,
                    eip: 0,
                    esp: 0,
                    cs: 0,
                    ds: 0,
                    es: 0,
                    fs: 0,
                    gs: 0,
                    ss: 0,
                    eflags: 0,
                    cr0: 0,
                    cr2: 0,
                    cr3: 0,
                    cr4: 0,
                    dr0: 0,
                    dr1: 0,
                    dr2: 0,
                    dr3: 0,
                    dr6: 0,
                    dr7: 0,
                }
            }
        }
    }

    pub mod intrinsics {

        pub use self::cache::{invd, invlpg, wbinvd};
        pub use self::fence::*;
        pub use self::fpu::*;
        pub use self::power::*;
        pub use self::simd::*;
        pub use self::timing::{rdpmc, rdtsc, rdtscp};
        pub use self::r#virtual::*;

        #[inline(always)]
        pub unsafe fn cli() {
            unsafe {
                core::arch::asm!(".code32", "cli", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn sti() {
            unsafe {
                core::arch::asm!(".code32", "sti", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn stc() {
            unsafe {
                core::arch::asm!(".code32", "stc", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn clc() {
            unsafe {
                core::arch::asm!(".code32", "clc", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn cmc() {
            unsafe { core::arch::asm!(".code32", "cmc", options(nostack)) }
        }

        #[inline(always)]
        pub unsafe fn cld() {
            unsafe {
                core::arch::asm!(".code32", "cld", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn std() {
            unsafe {
                core::arch::asm!(".code32", "std", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn hlt() {
            unsafe {
                core::arch::asm!(".code32", "hlt", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn nop() {
            unsafe {
                core::arch::asm!(".code32", "nop", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn wait() {
            unsafe {
                core::arch::asm!(".code32", "wait", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn cpuid(eax: u32, ecx: u32) -> (u32, u32, u32, u32) {
            let mut eabcdx: (u32, u32, u32, u32) = (0, 0, 0, 0);
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "mov eax, {0:e}",
                    "mov ecx, {1:e}",
                    "cpuid",
                    "mov {0:e}, eax",
                    "mov {2:e}, ebx",
                    "mov {1:e}, ecx",
                    "mov {3:e}, edx",
                    inout(reg) eax => eabcdx.0,
                    inout(reg) ecx => eabcdx.2,
                    out(reg) eabcdx.1,
                    out(reg) eabcdx.3,
                    options(nostack, preserves_flags)
                );
            }
            eabcdx
        }

        pub mod cache {
            #[inline(always)]
            pub unsafe fn invlpg(address: u32) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "invlpg [{0:e}]",
                        in(reg) address,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn wbinvd() {
                unsafe {
                    core::arch::asm!(".code32", "wbinvd", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn invd() {
                unsafe {
                    core::arch::asm!(".code32", "invd", options(nostack, preserves_flags));
                }
            }
        }

        pub mod timing {

            #[inline(always)]
            pub unsafe fn rdtsc() -> u64 {
                let (high, low): (u32, u32);
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "rdtsc",
                        out("edx") high,
                        out("eax") low,
                        options(nostack, preserves_flags)
                    );
                }
                ((high as u64) << 32) | (low as u64)
            }

            #[inline(always)]
            pub unsafe fn rdtscp() -> (u64, u32) {
                let (high, low, aux): (u32, u32, u32);
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "rdtscp",
                        out("edx") high,
                        out("eax") low,
                        out("ecx") aux,
                        options(nostack, preserves_flags)
                    );
                }
                (((high as u64) << 32) | (low as u64), aux)
            }

            #[inline(always)]
            pub unsafe fn rdpmc(counter: u32) -> u64 {
                let (low, high): (u32, u32);
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov ecx, {0:e}",
                        "rdpmc",
                        in(reg) counter,
                        out("edx") high,
                        out("eax") low,
                        options(nostack, preserves_flags)
                    );
                }
                ((high as u64) << 32) | (low as u64)
            }
        }

        pub mod fpu {
            //! Legacy x87 FPU

            #[inline(always)]
            pub unsafe fn fninit() {
                unsafe {
                    core::arch::asm!(".code32", "fninit", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn fxsave(pointer: *mut [u8; 512]) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "fxsave [{0:e}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn fxrstor(pointer: *const [u8; 512]) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "fxrstor [{0:e}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn fstcw() -> u16 {
                let value: u16;
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "sub esp, 2",
                        "fstcw word ptr [esp]",
                        "pop {0:x}",
                        out(reg) value,
                        options(nostack, preserves_flags)
                    );
                }
                value
            }

            #[inline(always)]
            pub unsafe fn fldcw(value: u16) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "push {0:x}",
                        "fldcw word ptr [esp]",
                        "add esp, 2",
                        in(reg) value,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn fwait() {
                unsafe {
                    core::arch::asm!(".code32", "fwait", options(nostack, preserves_flags));
                }
            }
        }

        pub mod simd {
            //! AVX & AVX-512

            #[inline(always)]
            pub unsafe fn xgetbv(xcr: u32) -> u64 {
                let (high, low): (u32, u32);
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov ecx, {0:e}",
                        "xgetbv",
                        in(reg) xcr,
                        out("edx") high,
                        out("eax") low,
                        options(nostack, preserves_flags)
                    );
                }
                ((high as u64) << 32) | (low as u64)
            }

            #[inline(always)]
            pub unsafe fn xsetbv(xcr: u32, value: u64) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov edx, {0:e}",
                        "mov eax, {1:e}",
                        "mov ecx, {2:e}",
                        "xsetbv",
                        in(reg) (value >> 32) as u32,
                        in(reg) value as u32,
                        in(reg) xcr,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn xsave(pointer: *mut u8, mask: u64) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov edx, {0:e}",
                        "mov eax, {1:e}",
                        "xsave [{2:e}]",
                        in(reg) (mask >> 32) as u32,
                        in(reg) mask as u32,
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn xrstor(pointer: *const u8, mask: u64) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov edx, {0:e}",
                        "mov eax, {1:e}",
                        "xrstor [{2:e}]",
                        in(reg) (mask >> 32) as u32,
                        in(reg) mask as u32,
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn emms() {
                unsafe {
                    core::arch::asm!(".code32", "emms", options(nostack, preserves_flags));
                }
            }
        }

        pub mod r#virtual {
            #[inline(always)]
            pub unsafe fn vmxon(pointer: *const u8) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "vmxon [{0:e}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmxoff() {
                unsafe {
                    core::arch::asm!(".code32", "vmxoff", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn vmcall() {
                unsafe {
                    core::arch::asm!(".code32", "vmcall", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn vmlaunch() {
                unsafe {
                    core::arch::asm!(".code32", "vmlaunch", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn vmresume() {
                unsafe {
                    core::arch::asm!(".code32", "vmresume", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn invvpid(r#type: u32, descriptor: *const u64) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov eax, {0:e}",
                        "mov edx, 0",
                        "invvpid [{1:e}]",
                        in(reg) r#type,
                        in(reg) descriptor,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn invept(r#type: u32, descriptor: *const u64) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov eax, {0:e}",
                        "mov edx, 0",
                        "invept [{1:e}]",
                        in(reg) r#type,
                        in(reg) descriptor,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmptrld(pointer: *const u64) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "vmptrld [{0:e}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmptrst(pointer: *mut u64) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "vmptrst [{0:e}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmclear(pointer: *const u64) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "vmclear [{0:e}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmread(field: u32) -> u32 {
                let value: u32;
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov ecx, {0:e}",
                        "vmread eax, ecx",
                        "mov {1:e}, eax",
                        in(reg) field,
                        out(reg) value,
                        options(nostack, preserves_flags)
                    );
                }
                value
            }

            #[inline(always)]
            pub unsafe fn vmwrite(field: u32, value: u32) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov ecx, {0:e}",
                        "mov eax, {1:e}",
                        "vmwrite eax, ecx",
                        in(reg) field,
                        in(reg) value,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmrun(pointer: *const u8) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "vmrun [{0:e}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmsave(pointer: *const u8) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "vmsave [{0:e}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmload(pointer: *const u8) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "vmload [{0:e}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn clgi() {
                unsafe {
                    core::arch::asm!(".code32", "clgi", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn stgi() {
                unsafe {
                    core::arch::asm!(".code32", "stgi", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn invlpga(address: u32, asid: u32) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov eax, {0:e}",
                        "mov ecx, {1:e}",
                        "invlpga",
                        in(reg) address,
                        in(reg) asid,
                        options(nostack, preserves_flags)
                    );
                }
            }
        }

        pub mod fence {
            #[inline(always)]
            pub unsafe fn mfence() {
                unsafe {
                    core::arch::asm!(".code32", "mfence", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn lfence() {
                unsafe {
                    core::arch::asm!(".code32", "lfence", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn sfence() {
                unsafe {
                    core::arch::asm!(".code32", "sfence", options(nostack, preserves_flags));
                }
            }
        }

        pub mod power {
            #[inline(always)]
            pub unsafe fn monitor(address: *const u8, extensions: u32, hints: u32) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov eax, {0:e}",
                        "mov ecx, {1:e}",
                        "mov edx, {2:e}",
                        "monitor",
                        in(reg) address,
                        in(reg) extensions,
                        in(reg) hints,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn mwait(hints: u32, extensions: u32) {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov eax, {0:e}",
                        "mov ecx, {1:e}",
                        "mwait",
                        in(reg) hints,
                        in(reg) extensions,
                        options(nostack, preserves_flags)
                    );
                }
            }
        }
    }

    pub mod stack {

        #[inline(always)]
        pub unsafe fn popw() -> u16 {
            let value: u16;
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "pop {0:x}",
                    out(reg) value,
                    options(preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn pushw(value: u16) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "push {0:x}",
                    in(reg) value,
                    options(preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn popl() -> u32 {
            let value: u32;
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "pop {0:e}",
                    out(reg) value,
                    options(preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn pushl(value: u32) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "push {0:e}",
                    in(reg) value,
                    options(preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn popad() {
            unsafe {
                core::arch::asm!(".code32", "popad", options(preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn pushad() {
            unsafe {
                core::arch::asm!(".code32", "pushad", options(preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn popf() {
            unsafe {
                core::arch::asm!(".code32", "popf", options(preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn pushf() {
            unsafe {
                core::arch::asm!(".code32", "pushf", options(preserves_flags));
            }
        }
    }

    pub mod interrupts {
        /*!
        # Interrupts

        When an interrupt fires, the cpu pushes these onto the stack, depending on CPL (current priotirty level)
        stack : [eip] [cs] [eflags] ([esp]) ([ss]) ([error_code])
        Maybe write an actual doc comment bro
        */

        pub struct InterruptFrame {
            pub eip: u32,
            pub cs: u16,
            pub eflags: u32,
            pub esp: u32,
            pub ss: u16,
            pub error_code: u32,
        }

        #[inline(always)]
        pub unsafe fn int(vector: u8) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "int {0}",
                    in(reg_byte) vector,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn iret() -> ! {
            unsafe { core::arch::asm!(".code32", "iret", options(nostack, preserves_flags)) }

            unreachable!();
        }
    }

    pub mod io {

        #[inline(always)]
        pub unsafe fn outb_reg(port: u16, value: u8) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "mov dx, {0:x}",
                    "mov al, {1}",
                    "out dx, al",
                    in(reg) port,
                    in(reg_byte) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn inb_reg(port: u16) -> u8 {
            let value: u8;
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "mov dx, {0:x}",
                    "in al, dx",
                    "mov {1}, al",
                    in(reg) port,
                    out(reg_byte) value,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[macro_export]
        macro_rules! outb_imm_32 {
            ($port:expr, $value:expr) => {

                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov al, {1}",
                        "out {0}, al",
                        const $port,
                        in(reg_byte) $value,
                        options(nostack, preserves_flags)
                    );
                }

            };
        }

        #[macro_export]
        macro_rules! inb_imm_32 {
            ($port:expr) => {

                let value: u8;
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "in al, {0}",
                        "mov {1}, al",
                        const $port,
                        out(reg_byte) value,
                        options(nostack, preserves_flags)
                    );
                }

                value
            };
        }

        #[inline(always)]
        pub unsafe fn outw_reg(port: u16, value: u16) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "mov dx, {0:x}",
                    "mov ax, {1:x}",
                    "out dx, ax",
                    in(reg) port,
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn inw_reg(port: u16) -> u16 {
            let value: u16;
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "mov dx, {0:x}",
                    "in ax, dx",
                    "mov {1:x}, ax",
                    in(reg) port,
                    out(reg) value,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[macro_export]
        macro_rules! outw_imm_32 {
            ($port:expr, $value:expr) => {

                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov ax, {1:x}",
                        "out {0}, ax",
                        const $port,
                        in(reg) $value,
                        options(nostack, preserves_flags)
                    );
                }

            };
        }

        #[macro_export]
        macro_rules! inw_imm_32 {
            ($port:expr) => {

                let value: u16;

                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "in ax, {0}",
                        "mov {1:x}, ax",
                        const $port,
                        out(reg) value,
                        options(nostack, preserves_flags)
                    );
                }

                value
            };
        }

        #[inline(always)]
        pub unsafe fn outl_reg(port: u16, value: u32) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "mov dx, {0:x}",
                    "mov eax, {1:e}",
                    "out dx, eax",
                    in(reg) port,
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn inl_reg(port: u16) -> u32 {
            let value: u32;
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "mov dx, {0:x}",
                    "in eax, dx",
                    "mov {1:e}, eax",
                    in(reg) port,
                    out(reg) value,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[macro_export]
        macro_rules! outl_imm_32 {
            ($port:expr, $value:expr) => {

                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "mov eax, {1:e}",
                        "out {0}, eax",
                        const $port,
                        in(reg) $value,
                        options(nostack, preserves_flags)
                    );
                }

            };
        }

        #[macro_export]
        macro_rules! inl_imm_32 {
            ($port:expr) => {

                let value: u32;

                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "in eax, {0}",
                        "mov {1:e}, eax",
                        const $port,
                        out(reg) value,
                        options(nostack, preserves_flags)
                    );
                }

                value
            };
        }
    }

    #[repr(C, packed)]
    pub struct FarPtr {
        pub offset: u32,
        pub selector: u16,
    }

    impl FarPtr {
        pub fn new(selector: u16, offset: u32) -> Self {
            Self {
                offset: offset,
                selector: selector,
            }
        }
    }

    pub mod memory {
        #[inline(always)]
        pub unsafe fn loadb(ptr: super::FarPtr) -> u8 {
            let value: u8;
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov {0}, [{2:e}]",
                    "pop ds",
                    out(reg_byte) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn storeb(ptr: super::FarPtr, value: u8) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov [{2:e}], {0}",
                    "pop ds",
                    in(reg_byte) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn loadw(ptr: super::FarPtr) -> u16 {
            let value: u16;
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov {0:x}, [{2:e}]",
                    "pop ds",
                    out(reg) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn storew(ptr: super::FarPtr, value: u16) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov [{2:e}], {0:x}",
                    "pop ds",
                    in(reg) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn loadl(ptr: super::FarPtr) -> u32 {
            let value: u32;
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov {0:e}, [{2:e}]",
                    "pop ds",
                    out(reg) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn storel(ptr: super::FarPtr, value: u32) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov [{2:e}], {0:e}",
                    "pop ds",
                    in(reg) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }
        }
    }

    pub mod control {
        #[inline(always)]
        pub unsafe fn call_reg(value: u32) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "call {0:e}",
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[macro_export]
        macro_rules! call_imm_32 {
            ($value:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "call {0}",
                        const $value,
                        options(nostack, preserves_flags)
                    );
                }
            };
        }

        #[inline(always)]
        pub unsafe fn call_mem(address: u32) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "call [{0:e}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[macro_export]
        macro_rules! callfar_imm_32 {
            ($selector:expr, $offset:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "call far {0}:{1}",
                        const $selector,
                        const $offset,
                        options(nostack, preserves_flags)
                    );
                }
            };
        }

        #[inline(always)]
        pub unsafe fn callfar_mem(address: u32) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "call far [{0:e}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn jmp_reg(offset: u32) -> ! {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "jmp {0:e}",
                    in(reg) offset,
                    options(nostack, preserves_flags)
                );
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! jmp_imm_32 {
            ($offset:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "jmp {0}",
                        const $offset,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn jmp_mem(address: u32) -> ! {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "jmp [{0:e}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
            unreachable!();
        }

        #[inline(always)]
        pub unsafe fn jmpfar_mem(address: u32) -> ! {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "jmp far [{0:e}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! jmpfar_imm_32 {
            ($selector:expr, $offset:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "jmp far {0}:{1}",
                        const $selector,
                        const $offset,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn ret() -> ! {
            unsafe {
                core::arch::asm!(".code32", "ret", options(nostack, preserves_flags));
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! ret_imm_32 {
            ($value:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "ret {0:x}",
                        const $value,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn retf() -> ! {
            unsafe {
                core::arch::asm!(".code32", "retf", options(nostack, preserves_flags));
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! retf_imm_32 {
            ($value:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code32",
                        "retf {0:x}",
                        const $value,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn sysenter() {
            unsafe {
                core::arch::asm!(".code32", "sysenter", options(nostack, preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn sysexit() -> ! {
            unsafe {
                core::arch::asm!(".code32", "sysexit", options(nostack, preserves_flags));
            }
            unreachable!()
        }

        #[inline(always)]
        pub unsafe fn syscall() {
            unsafe {
                core::arch::asm!(".code32", "syscall", options(nostack, preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn sysret() -> ! {
            unsafe {
                core::arch::asm!(".code32", "sysret", options(nostack, preserves_flags));
            }
            unreachable!();
        }
    }

    #[repr(C, packed)]
    pub struct Descriptor {
        pub limit_low: u16,
        pub base_low: u16,
        pub base_mid: u8,
        pub access: u8,
        pub granularity: u8,
        pub base_high: u8,
    }

    impl Default for Descriptor {
        fn default() -> Self {
            Self {
                limit_low: 0,
                base_low: 0,
                base_mid: 0,
                access: 0,
                granularity: 0,
                base_high: 0,
            }
        }
    }

    impl Descriptor {
        pub const ACCESS_TYPE: u8 = 0xF;
        pub const ACCESS_S: u8 = 0x10;
        pub const ACCESS_DPL: u8 = 0x60;
        pub const ACCESS_P: u8 = 0x80;

        pub const GRANULARITY_G: u8 = 0x80;
        pub const GRANULARITY_DB: u8 = 0x40;
        pub const GRANULARITY_L: u8 = 0x20;
        pub const GRANULARITY_AVL: u8 = 0x10;
        /// Limit high.
        pub const GRANULARITY_LH: u8 = 0x0F;
    }

    #[repr(C, packed)]
    pub struct Dtr {
        pub limit: u16,
        pub base: u32,
    }

    impl Default for Dtr {
        fn default() -> Self {
            Self { limit: 0, base: 0 }
        }
    }

    pub mod gdt {
        /*!
        # Global Descriptor Table
        */

        // [S=1] Code segment types
        pub const CODE_EXECUTE_ONLY: u8 = 0x08;
        pub const CODE_EXECUTE_READ: u8 = 0x0A;
        pub const CODE_CONFORMING: u8 = 0x0C;
        pub const CODE_CONFORMING_READ: u8 = 0x0E;

        // [S=1] Data segment types
        pub const DATA_READ_ONLY: u8 = 0x00;
        pub const DATA_READ_WRITE: u8 = 0x02;
        pub const DATA_EXPAND_DOWN: u8 = 0x04;
        pub const DATA_EXPAND_DOWN_RW: u8 = 0x06;

        // [S=0] System descriptor types
        pub const CALL_GATE: u8 = 0x0C;

        #[inline(always)]
        pub unsafe fn lgdt(gdtr: &super::Dtr) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "lgdt [{0:e}]",
                    in(reg) gdtr,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn sgdt(gdtr: &mut super::Dtr) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "sgdt [{0:e}]",
                    in(reg) gdtr,
                    options(nostack, preserves_flags)
                );
            }
        }

        /*
        // Build the GDT
        let mut gdt = [Descriptor::null(); 6];
        gdt[1] = Descriptor::code_segment(0, 0xFFFFF, 0);
        gdt[2] = Descriptor::data_segment(0, 0xFFFFF, 0);
        gdt[3] = Descriptor::code_segment(0, 0xFFFFF, 3);
        gdt[4] = Descriptor::data_segment(0, 0xFFFFF, 3);
        // Note: TSS and LDT descriptors would be placed here if needed.

        // Load it
        let gdtr = Gdtr {
            limit: (core::mem::size_of_val(&gdt) - 1) as u16,
            base: &gdt as *const _ as u32,
        };
        unsafe { lgdt(&gdtr) };
        */
    }

    pub mod ldt {
        /*!
        # Local Descriptor Table
        */

        pub const TYPE: u8 = 0x02;

        #[inline(always)]
        pub unsafe fn lldt(selector: u16) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "lldt {0:x}",
                    in(reg) selector,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn sldt() -> u16 {
            let selector: u16;
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "sldt {0:x}",
                    out(reg) selector,
                    options(nostack, preserves_flags)
                );
            }
            selector
        }
    }

    pub mod idt {
        /*!
        # Interrupt Descriptor Table

        ...
        */
        #[repr(C, packed)]
        pub struct Gate {
            pub offset_low: u16,
            pub selector: u16,
            pub reserved: u8, // Must be 0.
            pub flags: u8,
            pub offset_high: u16,
        }

        impl Default for Gate {
            fn default() -> Self {
                Self {
                    offset_low: 0,
                    selector: 0,
                    reserved: 0,
                    flags: 0,
                    offset_high: 0,
                }
            }
        }

        impl Gate {
            pub const TYPE_INTERRUPT: u8 = 0x0E;
            pub const TYPE_TRAP: u8 = 0x0F;
            pub const TYPE_TASK: u8 = 0x05;
        }

        #[inline(always)]
        pub unsafe fn lidt(idtr: &super::Dtr) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "lidt [{0:e}]",
                    in(reg) idtr,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn sidt(idtr: &mut super::Dtr) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "sidt [{0:e}]",
                    in(reg) idtr,
                    options(nostack, preserves_flags)
                );
            }
        }
    }

    pub mod tss {
        /*!
        # Task State Segment

        yea idk
        */

        #[repr(C, packed)]
        pub struct Tss {
            pub prev_task_link: u16,
            pub reserved0: u16,
            pub esp0: u32,
            pub ss0: u16,
            pub reserved1: u16,
            pub esp1: u32,
            pub ss1: u16,
            pub reserved2: u16,
            pub esp2: u32,
            pub ss2: u16,
            pub reserved3: u16,
            pub cr3: u32,
            pub eip: u32,
            pub eflags: u32,
            pub eax: u32,
            pub ecx: u32,
            pub edx: u32,
            pub ebx: u32,
            pub esp: u32,
            pub ebp: u32,
            pub esi: u32,
            pub edi: u32,
            pub es: u16,
            pub reserved4: u16,
            pub cs: u16,
            pub reserved5: u16,
            pub ss: u16,
            pub reserved6: u16,
            pub ds: u16,
            pub reserved7: u16,
            pub fs: u16,
            pub reserved8: u16,
            pub gs: u16,
            pub reserved9: u16,
            pub ldt_selector: u16,
            pub reserved10: u16,
            pub iomap_base: u16,
        }

        impl Default for Tss {
            fn default() -> Self {
                Self {
                    prev_task_link: 0,
                    reserved0: 0,
                    esp0: 0,
                    ss0: 0,
                    reserved1: 0,
                    esp1: 0,
                    ss1: 0,
                    reserved2: 0,
                    esp2: 0,
                    ss2: 0,
                    reserved3: 0,
                    cr3: 0,
                    eip: 0,
                    eflags: 0,
                    eax: 0,
                    ecx: 0,
                    edx: 0,
                    ebx: 0,
                    esp: 0,
                    ebp: 0,
                    esi: 0,
                    edi: 0,
                    es: 0,
                    reserved4: 0,
                    cs: 0,
                    reserved5: 0,
                    ss: 0,
                    reserved6: 0,
                    ds: 0,
                    reserved7: 0,
                    fs: 0,
                    reserved8: 0,
                    gs: 0,
                    reserved9: 0,
                    ldt_selector: 0,
                    reserved10: 0,
                    iomap_base: 0,
                }
            }
        }

        impl Tss {
            pub const TYPE_AVAILABLE: u8 = 0x09;
            pub const TYPE_BUSY: u8 = 0x0B;
        }

        #[inline(always)]
        pub unsafe fn ltr(selector: u16) {
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "ltr {0:x}",
                    in(reg) selector,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn str() -> u16 {
            let selector: u16;
            unsafe {
                core::arch::asm!(
                    ".code32",
                    "str {0:x}",
                    out(reg) selector,
                    options(nostack, preserves_flags)
                );
            }
            selector
        }
    }

    pub mod paging {
        /*!
        # Paging

        Write here what the paging mechanic is, and how to implement it / Whats expected
        And PDE / PTE difference
        and PAE extension
        and formats

        idk about this module tho.
        */

        pub const P: u32 = 0x1; // Present.
        pub const RW: u32 = 0x2; // Read/Write.
        pub const US: u32 = 0x4; // User/Supervisor.
        pub const PWT: u32 = 0x8; // Write through.
        pub const PCD: u32 = 0x10; // Cache disable.
        pub const A: u32 = 0x20; // Accessed.
        pub const PS: u32 = 0x80; // Page size.

        pub const PD: u32 = 0x40; // Dirty.
        pub const G: u32 = 0x100; // Global.
    }
}

pub mod quad {
    //! .code64

    pub mod registers {

        /*!
        # x86-64 Registers

        ## General Purpose

        - **rax** : accumulator
        - **rbx** : base
        - **rcx** : counter
        - **rdx** : data
        - **r8** : any
        - **r9** : any
        - **r11** : any
        - **r12** : any
        - **r13** : any
        - **r14** : any
        - **r15** : any

        ## Indices

        - **rdi** : destination
        - **rsi** : source

        ## Pointers

        - **rbp** : base
        - **rip** : instruction
        - **rsp** : stack

        ## Segments

        - **cs** : code
        - **ds** : data
        - **es** : extension
        - **fs** : extension
        - **gs** : extension
        - **ss** : stack

        ## Status

        - **rflags** : flags

        ## Control

        - **cr0** : control register 0
        - **cr2** : control register 2
        - **cr3** : control register 3
        - **cr4** : control register 4
        - **cr8** : control register 8

        ## Debug

        - **dr0 - dr3** : debug address
        - **dr6** : debug status
        - **dr7** : debug control
        */

        pub use control::{cr0, cr2, cr3, cr4, cr8};
        pub use data::{r8, r9, r10, r11, r12, r13, r14, r15, rax, rbx, rcx, rdx};
        pub use debug::{dr0, dr1, dr2, dr3, dr6, dr7};
        pub use index::{rdi, rsi};
        pub use pointer::{rbp, rip, rsp};
        pub use segment::{cs, ds, es, ss};
        pub use status::rflags;

        pub mod data {
            pub mod rax {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, rax",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov rax, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod rbx {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, rbx",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov rbx, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod rcx {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, rcx",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov rcx, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod rdx {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, rdx",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov rdx, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod r8 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, r8",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov r8, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod r9 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, r9",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov r9, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod r10 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, r10",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov r10, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod r11 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, r11",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov r11, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod r12 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, r12",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov r12, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod r13 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, r13",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov r13, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod r14 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, r14",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov r14, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod r15 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, r15",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov r15, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod index {

            pub mod rdi {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, rdi",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov rdi, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod rsi {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, rsi",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov rsi, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod pointer {

            pub mod rbp {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, rbp",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov rbp, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod rip {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "call 2f",
                            "2:",
                            "pop {0:r}",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }
            }

            pub mod rsp {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, rsp",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov rsp, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod segment {

            pub mod cs {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:x}, cs",
                            out(reg) value,
                            options(nostack, preserves_flags),
                        );
                    }
                    value
                }
            }

            pub mod ds {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:x}, ds",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov ds, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod es {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:x}, es",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov es, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod fs {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:x}, fs",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov fs, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod gs {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:x}, gs",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov gs, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod ss {
                #[inline(always)]
                pub unsafe fn read() -> u16 {
                    let value: u16;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:x}, ss",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u16) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov ss, {0:x}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod status {

            pub mod rflags {

                pub const CF: u32 = 0x0001; // Carry
                pub const PF: u32 = 0x0004; // Parity
                pub const AF: u32 = 0x0010; // Auxiliary Carry
                pub const ZF: u32 = 0x0040; // Zero
                pub const SF: u32 = 0x0080; // Sign
                pub const TF: u32 = 0x0100; // Trap
                pub const IF: u32 = 0x0200; // Interrupt
                pub const DF: u32 = 0x0400; // Direction
                pub const OF: u32 = 0x0800; // Overflow
                pub const IOPL: u32 = 0x3000; // I/O Privilege Level
                pub const NT: u32 = 0x4000; // Nested Task

                pub const RF: u32 = 0x10000; // Resume
                pub const VM: u32 = 0x20000; // Virtual 8086 Mode
                pub const AC: u32 = 0x40000; // Alignment Check
                pub const VIF: u32 = 0x80000; // Virtual Interrupt
                pub const VIP: u32 = 0x100000; // Virtual Interrupt Pending
                pub const ID: u32 = 0x200000; // ID

                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let rflags: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "pushf",
                            "pop {0:r}",
                            out(reg) rflags,
                            options(nostack)
                        );
                    }
                    rflags
                }

                #[inline(always)]
                pub unsafe fn write(rflags: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "push {0:r}",
                            "popf",
                            in(reg) rflags,
                            options(nostack)
                        );
                    }
                }
            }
        }

        pub mod control {
            pub mod cr0 {

                /// Protection enable.
                pub const PE: u64 = 0x1;
                /// Paging enable.
                pub const PG: u64 = 0x8000_0000;
                /// Write protect.
                pub const WP: u64 = 0x0001_0000;
                /// Numeric error.
                pub const NE: u64 = 0x0000_0020;
                /// Extension type.
                pub const ET: u64 = 0x0000_0010;
                /// Task switched.
                pub const TS: u64 = 0x0000_0008;
                /// Emulate (x87 FPU).
                pub const EM: u64 = 0x0000_0004;
                /// Monitor coprocessor.
                pub const MP: u64 = 0x0000_0002;
                /// Cache disable.
                pub const CD: u64 = 0x4000_0000;
                /// No write-through.
                pub const NW: u64 = 0x2000_0000;
                /// Alignment check.
                pub const AM: u64 = 0x0004_0000;

                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, cr0",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov cr0, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod cr2 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, cr2",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov cr2, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod cr3 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, cr3",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov cr3, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod cr4 {
                /// Virtual 8086 extensions.
                pub const VME: u64 = 0x0000_0001;
                /// Protected virtual interrupts.
                pub const PVI: u64 = 0x0000_0002;
                /// Time stamp disable.
                pub const TSD: u64 = 0x0000_0004;
                /// Debugging extensions
                pub const DE: u64 = 0x0000_0008;
                /// Page size extension.
                pub const PSE: u64 = 0x0000_0010;
                /// Physical address extension.
                pub const PAE: u64 = 0x0000_0020;
                /// Machine check exception.
                pub const MCE: u64 = 0x0000_0040;
                /// Page global enable.
                pub const PGE: u64 = 0x0000_0080;
                ///
                pub const PCE: u64 = 0x0000_0100;
                ///
                pub const OSFXSR: u64 = 0x0000_0200;
                ///
                pub const OSXSAVE: u64 = 0x0000_0400;
                ///
                pub const UMIP: u64 = 0x0000_0800;
                ///
                pub const LA57: u64 = 0x0000_1000;
                ///
                pub const VMXE: u64 = 0x0000_2000;
                ///
                pub const SMXE: u64 = 0x0000_4000;
                ///
                pub const FSGSBASE: u64 = 0x0001_0000;
                ///
                pub const PCID: u64 = 0x0002_0000;
                ///
                pub const XSAVE: u64 = 0x0004_0000;
                ///
                pub const SMEP: u64 = 0x0010_0000;
                ///
                pub const SMAP: u64 = 0x0020_0000;
                ///
                pub const PKE: u64 = 0x0040_0000;
                ///
                pub const CET: u64 = 0x0080_0000;
                ///
                pub const PKS: u64 = 0x0100_0000;

                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, cr4",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov cr4, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod cr8 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, cr8",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov cr8, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        pub mod debug {
            pub mod dr0 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, dr0",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov dr0, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dr1 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, dr1",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov dr1, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dr2 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, dr2",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov dr2, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dr3 {
                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, dr3",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov dr3, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dr6 {
                ///
                pub const B0: u32 = 0x0000_0001;
                ///
                pub const B1: u32 = 0x0000_0002;
                ///
                pub const B2: u32 = 0x0000_0004;
                ///
                pub const B3: u32 = 0x0000_0008;
                ///
                pub const B4: u32 = 0x0000_0010;
                ///
                pub const B5: u32 = 0x0000_0020;
                ///
                pub const B6: u32 = 0x0000_0040;
                ///
                pub const B7: u32 = 0x0000_0080;
                ///
                pub const BD: u32 = 0x0000_0200;
                ///
                pub const BS: u32 = 0x0000_4000;
                ///
                pub const BT: u32 = 0x0000_8000;

                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, dr6",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov dr6, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }

            pub mod dr7 {

                pub const L0: u32 = 0x0000_0001;
                pub const G0: u32 = 0x0000_0002;
                pub const L1: u32 = 0x0000_0004;
                pub const G1: u32 = 0x0000_0008;
                pub const L2: u32 = 0x0000_0010;
                pub const G2: u32 = 0x0000_0020;
                pub const L3: u32 = 0x0000_0040;
                pub const G3: u32 = 0x0000_0080;

                ///
                pub const LE: u32 = 0x0000_0100;
                ///
                pub const GE: u32 = 0x0000_0200;
                ///
                pub const GD: u32 = 0x0000_0400;

                #[inline(always)]
                pub unsafe fn read() -> u64 {
                    let value: u64;
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov {0:r}, dr7",
                            out(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                    value
                }

                #[inline(always)]
                pub unsafe fn write(value: u64) {
                    unsafe {
                        core::arch::asm!(
                            ".code64",
                            "mov dr7, {0:r}",
                            in(reg) value,
                            options(nostack, preserves_flags)
                        );
                    }
                }
            }
        }

        /// same as protected ?
        pub mod msr {
            /// Extended Feature Enable Register
            pub const IA32_EFER: u32 = 0xC0000080;
            /// System Call Target Address
            pub const IA32_STAR: u32 = 0xC0000081;
            /// Long STAR
            pub const IA32_LSTAR: u32 = 0xC0000082;
            /// Compatibility STAR
            pub const IA32_CSTAR: u32 = 0xC0000083;
            /// SYSCALL/SYSRET Flag Mask
            pub const IA32_SYSCALL_MASK: u32 = 0xC0000084;
            /// FS Base
            pub const IA32_FS_BASE: u32 = 0xC0000100;
            /// GS Base
            pub const IA32_GS_BASE: u32 = 0xC0000101;
            /// Kernel GS Base
            pub const IA32_KERNEL_GS_BASE: u32 = 0xC0000102;
            /// Auxiliary TSC
            pub const IA32_TSC_AUX: u32 = 0xC0000103;
            /// SYSENTER CS
            pub const IA32_SYSENTER_CS: u32 = 0x00000174;
            /// SYSENTER ESP
            pub const IA32_SYSENTER_ESP: u32 = 0x00000175;
            /// SYSENTER EIP
            pub const IA32_SYSENTER_EIP: u32 = 0x00000176;
            /// Debug Control
            pub const IA32_DEBUGCTL: u32 = 0x000001D9;
            /// Last Branch From IP
            pub const IA32_LASTBRANCHFROMIP: u32 = 0x000001DB;
            /// Last Branch To IP
            pub const IA32_LASTBRANCHTOIP: u32 = 0x000001DC;
            /// Last Interrupt From IP
            pub const IA32_LASTINTFROMIP: u32 = 0x000001DD;
            /// Last Interrupt to IP
            pub const IA32_LASTINTTOIP: u32 = 0x000001DE;
            /// Process Address Space ID
            pub const IA32_PASID: u32 = 0x00000D93;
            /// Power Control
            pub const IA32_POWER_CTL: u32 = 0x000001FC;
            /// Speculation Control
            pub const IA32_SPEC_CTRL: u32 = 0x00000048;
            /// Prediction Command
            pub const IA32_PRED_CMD: u32 = 0x00000049;
            /// Architecture Capabilities
            pub const IA32_ARCH_CAPABILITIES: u32 = 0x0000010A;
            /// Flush Command
            pub const IA32_FLUSH_CMD: u32 = 0x0000010B;
            /// TSX Control
            pub const IA32_TSX_CTRL: u32 = 0x00000122;
            /// Microcode Update Control
            pub const IA32_MCU_OPT_CTRL: u32 = 0x00000123;

            /// Machine Check Global Capability
            pub const IA32_MCG_CAP: u32 = 0x00000179;
            /// Machine Check Global Status
            pub const IA32_MCG_STATUS: u32 = 0x0000017A;
            /// Machine Check Global Control
            pub const IA32_MCG_CTL: u32 = 0x0000017B;
            /// Machine Check Extended Control
            pub const IA32_MCG_EXT_CTL: u32 = 0x000004D0;
            /// Machine Check Bank 0 Control
            pub const IA32_MC0_CTL: u32 = 0x00000400;
            /// Machine Check Bank 0 Status
            pub const IA32_MC0_STATUS: u32 = 0x00000401;
            /// Machine Check Bank 0 Address
            pub const IA32_MC0_ADDR: u32 = 0x00000402;
            /// Machine Check Bank 0 Miscellaneous
            pub const IA32_MC0_MISC: u32 = 0x00000403;

            /// Performance Counter
            pub const IA32_PERFCTR0: u32 = 0x000000C1;
            /// PEBS Enable
            pub const IA32_PEBS_ENABLE: u32 = 0x000003F1;
            /// PEBS Base
            pub const IA32_PEBS_BASE: u32 = 0x000003F4;
            /// DS Area
            pub const IA32_DS_AREA: u32 = 0x00000600;
            /// Performance Capabilities
            pub const IA32_PERF_CAPABILITIES: u32 = 0x00000345;
            /// Processor Trace Control
            pub const IA32_RTIT_CTL: u32 = 0x00000570;
            /// Processor Trace Status
            pub const IA32_RTIT_STATUS: u32 = 0x00000571;
            /// Processor Trace Output Base
            pub const IA32_RTIT_OUTPUT_BASE: u32 = 0x00000560;
            /// Processor Trace Output Mask
            pub const IA32_RTIT_OUTPUT_MASK: u32 = 0x00000561;

            /// MTRR Capability
            pub const IA32_MTRRCAP: u32 = 0x000000FE;
            /// MTRR Default Type
            pub const IA32_MTRR_DEF_TYPE: u32 = 0x000002FF;
            /// MTRR Physical Mask
            pub const IA32_MTRR_PHYSMASK0: u32 = 0x00000200;
            /// MTRR Physical Base
            pub const IA32_MTRR_PHYSBASE0: u32 = 0x00000201;
            /// Fixed MTRR 64KB
            pub const IA32_MTRR_FIX64K_00000: u32 = 0x00000250;
            /// Fixed MTRR 16KB
            pub const IA32_MTRR_FIX16K_80000: u32 = 0x00000258;
            /// Fixed MTRR 4KB
            pub const IA32_MTRR_FIX4K_C0000: u32 = 0x00000268;
            /// Page Attribute Table
            pub const IA32_CR_PAT: u32 = 0x00000277;

            /// Package C3 Residency
            pub const IA32_PKG_C3_RESIDENCY: u32 = 0x000003F8;
            /// Package C6 Residency
            pub const IA32_PKG_C6_RESIDENCY: u32 = 0x000003F9;
            /// Core C3 Residency
            pub const IA32_CORE_C3_RESIDENCY: u32 = 0x000003FC;
            /// Core C6 Residency
            pub const IA32_CORE_C6_RESIDENCY: u32 = 0x000003FD;
            /// Package C8 Residency
            pub const IA32_PKG_C8_RESIDENCY: u32 = 0x00000630;

            /// FRED Stack Pointer for Level 0
            pub const IA32_FRED_RSP0: u32 = 0x000001CC;
            /// FRED Stack Pointer for Level 1
            pub const IA32_FRED_RSP1: u32 = 0x000001CD;
            /// FRED Stack Pointer for Level 2
            pub const IA32_FRED_RSP2: u32 = 0x000001CE;
            /// FRED Stack Pointer for Level 3
            pub const IA32_FRED_RSP3: u32 = 0x000001CF;
            /// FRED Stack Levels
            pub const IA32_FRED_STKLVLS: u32 = 0x000001D0;
            /// FRED Shadow Stack Pointer for Level 0
            pub const IA32_FRED_SSP0: u32 = 0x000001D1;
            /// FRED Configuration
            pub const IA32_FRED_CONFIG: u32 = 0x000001D4;

            #[inline(always)]
            pub unsafe fn read(msr: u32) -> u64 {
                let (high, low): (u32, u32);
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov ecx,{0:e}",
                        "rdmsr",
                        "mov {1:e},edx",
                        "mov {2:e},eax",
                        in(reg) msr,
                        out(reg) high,
                        out(reg) low,
                        options(nostack, preserves_flags)
                    );
                }
                ((high as u64) << 32) | (low as u64)
            }

            #[inline(always)]
            pub unsafe fn write(msr: u32, value: u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov edx,{0:e}",
                        "mov eax,{1:e}",
                        "mov ecx,{2:e}",
                        "wrmsr",
                        in(reg) (value >> 32) as u32,
                        in(reg) value as u32,
                        in(reg) msr,
                        options(nostack, preserves_flags)
                    );
                }
            }
        }

        pub struct Registers {
            pub rax: u64,
            pub rbx: u64,
            pub rcx: u64,
            pub rdx: u64,
            pub r8: u64,
            pub r9: u64,
            pub r10: u64,
            pub r11: u64,
            pub r12: u64,
            pub r13: u64,
            pub r14: u64,
            pub r15: u64,

            pub rdi: u64,
            pub rsi: u64,

            pub rbp: u64,
            pub rip: u64,
            pub rsp: u64,

            pub cs: u16,
            pub ds: u16,
            pub es: u16,
            pub fs: u16,
            pub gs: u16,
            pub ss: u16,

            pub rflags: u64,

            pub cr0: u64,
            pub cr2: u64,
            pub cr3: u64,
            pub cr4: u64,
            pub cr8: u64,

            pub dr0: u64,
            pub dr1: u64,
            pub dr2: u64,
            pub dr3: u64,
            pub dr6: u64,
            pub dr7: u64,
        }

        impl Default for Registers {
            fn default() -> Self {
                Self {
                    rax: 0,
                    rbx: 0,
                    rcx: 0,
                    rdx: 0,
                    r8: 0,
                    r9: 0,
                    r10: 0,
                    r11: 0,
                    r12: 0,
                    r13: 0,
                    r14: 0,
                    r15: 0,
                    rdi: 0,
                    rsi: 0,
                    rbp: 0,
                    rip: 0,
                    rsp: 0,
                    cs: 0,
                    ds: 0,
                    es: 0,
                    fs: 0,
                    gs: 0,
                    ss: 0,
                    rflags: 0,
                    cr0: 0,
                    cr2: 0,
                    cr3: 0,
                    cr4: 0,
                    cr8: 0,
                    dr0: 0,
                    dr1: 0,
                    dr2: 0,
                    dr3: 0,
                    dr6: 0,
                    dr7: 0,
                }
            }
        }
    }

    pub mod intrinsics {

        pub use self::cache::{invd, invlpg, wbinvd};
        pub use self::fence::*;
        pub use self::fpu::*;
        pub use self::power::*;
        pub use self::simd::*;
        pub use self::timing::{rdpmc, rdtsc, rdtscp};
        pub use self::r#virtual::*;

        #[inline(always)]
        pub unsafe fn cli() {
            unsafe {
                core::arch::asm!(".code64", "cli", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn sti() {
            unsafe {
                core::arch::asm!(".code64", "sti", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn stc() {
            unsafe {
                core::arch::asm!(".code64", "stc", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn clc() {
            unsafe {
                core::arch::asm!(".code64", "clc", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn cmc() {
            unsafe { core::arch::asm!(".code64", "cmc", options(nostack)) }
        }

        #[inline(always)]
        pub unsafe fn cld() {
            unsafe {
                core::arch::asm!(".code64", "cld", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn std() {
            unsafe {
                core::arch::asm!(".code64", "std", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn hlt() {
            unsafe {
                core::arch::asm!(".code64", "hlt", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn nop() {
            unsafe {
                core::arch::asm!(".code64", "nop", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn wait() {
            unsafe {
                core::arch::asm!(".code64", "wait", options(nostack));
            }
        }

        #[inline(always)]
        pub unsafe fn cpuid(eax: u32, ecx: u32) -> (u32, u32, u32, u32) {
            let mut eabcdx: (u32, u32, u32, u32) = (0, 0, 0, 0);
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "mov eax, {0:e}",
                    "mov ecx, {1:e}",
                    "cpuid",
                    "mov {0:e}, eax",
                    "mov {2:e}, ebx",
                    "mov {1:e}, ecx",
                    "mov {3:e}, edx",
                    inout(reg) eax => eabcdx.0,
                    inout(reg) ecx => eabcdx.2,
                    out(reg) eabcdx.1,
                    out(reg) eabcdx.3,
                    options(nostack, preserves_flags)
                );
            }
            eabcdx
        }

        pub mod cache {
            #[inline(always)]
            pub unsafe fn invlpg(address: u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "invlpg [{0:r}]",
                        in(reg) address,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn wbinvd() {
                unsafe {
                    core::arch::asm!(".code64", "wbinvd", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn invd() {
                unsafe {
                    core::arch::asm!(".code64", "invd", options(nostack, preserves_flags));
                }
            }
        }

        pub mod timing {

            #[inline(always)]
            pub unsafe fn rdtsc() -> u64 {
                let (high, low): (u32, u32);
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "rdtsc",
                        out("edx") high,
                        out("eax") low,
                        options(nostack, preserves_flags)
                    );
                }
                ((high as u64) << 32) | (low as u64)
            }

            #[inline(always)]
            pub unsafe fn rdtscp() -> (u64, u32) {
                let (high, low, aux): (u32, u32, u32);
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "rdtscp",
                        out("edx") high,
                        out("eax") low,
                        out("ecx") aux,
                        options(nostack, preserves_flags)
                    );
                }
                (((high as u64) << 32) | (low as u64), aux)
            }

            #[inline(always)]
            pub unsafe fn rdpmc(counter: u32) -> u64 {
                let (low, high): (u32, u32);
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov ecx, {0:e}",
                        "rdpmc",
                        in(reg) counter,
                        out("edx") high,
                        out("eax") low,
                        options(nostack, preserves_flags)
                    );
                }
                ((high as u64) << 32) | (low as u64)
            }
        }

        pub mod fpu {
            //! Legacy x87 FPU

            #[inline(always)]
            pub unsafe fn fninit() {
                unsafe {
                    core::arch::asm!(".code64", "fninit", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn fxsave(pointer: *mut [u8; 512]) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "fxsave [{0:r}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn fxrstor(pointer: *const [u8; 512]) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "fxrstor [{0:r}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn fstcw() -> u16 {
                let value: u16;
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "sub esp, 2",
                        "fstcw word ptr [esp]",
                        "pop {0:x}",
                        out(reg) value,
                        options(nostack, preserves_flags)
                    );
                }
                value
            }

            #[inline(always)]
            pub unsafe fn fldcw(value: u16) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "push {0:x}",
                        "fldcw word ptr [esp]",
                        "add esp, 2",
                        in(reg) value,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn fwait() {
                unsafe {
                    core::arch::asm!(".code64", "fwait", options(nostack, preserves_flags));
                }
            }
        }

        pub mod simd {
            //! AVX & AVX-512

            #[inline(always)]
            pub unsafe fn xgetbv(xcr: u32) -> u64 {
                let (high, low): (u32, u32);
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov ecx, {0:e}",
                        "xgetbv",
                        in(reg) xcr,
                        out("edx") high,
                        out("eax") low,
                        options(nostack, preserves_flags)
                    );
                }
                ((high as u64) << 32) | (low as u64)
            }

            #[inline(always)]
            pub unsafe fn xsetbv(xcr: u32, value: u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov edx, {0:e}",
                        "mov eax, {1:e}",
                        "mov ecx, {2:e}",
                        "xsetbv",
                        in(reg) (value >> 32) as u32,
                        in(reg) value as u32,
                        in(reg) xcr,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn xsave(pointer: *mut u8, mask: u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov edx, {0:e}",
                        "mov eax, {1:e}",
                        "xsave [{2:e}]",
                        in(reg) (mask >> 32) as u32,
                        in(reg) mask as u32,
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn xrstor(pointer: *const u8, mask: u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov edx, {0:e}",
                        "mov eax, {1:e}",
                        "xrstor [{2:e}]",
                        in(reg) (mask >> 32) as u32,
                        in(reg) mask as u32,
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn emms() {
                unsafe {
                    core::arch::asm!(".code64", "emms", options(nostack, preserves_flags));
                }
            }
        }

        pub mod r#virtual {
            #[inline(always)]
            pub unsafe fn vmxon(pointer: *const u8) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "vmxon [{0:r}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmxoff() {
                unsafe {
                    core::arch::asm!(".code64", "vmxoff", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn vmcall() {
                unsafe {
                    core::arch::asm!(".code64", "vmcall", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn vmlaunch() {
                unsafe {
                    core::arch::asm!(".code64", "vmlaunch", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn vmresume() {
                unsafe {
                    core::arch::asm!(".code64", "vmresume", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn invvpid(r#type: u64, descriptor: *const u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "invvpid [{1:r}], {0:r}",
                        in(reg) r#type,
                        in(reg) descriptor,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn invept(r#type: u64, descriptor: *const u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "invept [{1:r}], {0:r}",
                        in(reg) r#type,
                        in(reg) descriptor,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmptrld(pointer: *const u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "vmptrld [{0:r}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmptrst(pointer: *mut u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "vmptrst [{0:r}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmclear(pointer: *const u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "vmclear [{0:r}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmread(field: u64) -> u64 {
                let value: u64;
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov ecx, {0:e}",
                        "vmread rax, rcx",
                        "mov {1:r}, rax",
                        in(reg) field,
                        out(reg) value,
                        options(nostack, preserves_flags)
                    );
                }
                value
            }

            #[inline(always)]
            pub unsafe fn vmwrite(field: u32, value: u64) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov ecx, {0:e}",
                        "mov rax, {1:r}",
                        "vmwrite rax, rcx",
                        in(reg) field,
                        in(reg) value,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmrun(pointer: *const u8) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "vmrun [{0:r}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmsave(pointer: *const u8) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "vmsave [{0:r}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn vmload(pointer: *const u8) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "vmload [{0:r}]",
                        in(reg) pointer,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn clgi() {
                unsafe {
                    core::arch::asm!(".code64", "clgi", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn stgi() {
                unsafe {
                    core::arch::asm!(".code64", "stgi", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn invlpga(address: u64, asid: u32) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov rax, {0:r}",
                        "mov ecx, {1:e}",
                        "invlpga",
                        in(reg) address,
                        in(reg) asid,
                        options(nostack, preserves_flags)
                    );
                }
            }
        }

        pub mod fence {
            #[inline(always)]
            pub unsafe fn mfence() {
                unsafe {
                    core::arch::asm!(".code64", "mfence", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn lfence() {
                unsafe {
                    core::arch::asm!(".code64", "lfence", options(nostack, preserves_flags));
                }
            }

            #[inline(always)]
            pub unsafe fn sfence() {
                unsafe {
                    core::arch::asm!(".code64", "sfence", options(nostack, preserves_flags));
                }
            }
        }

        pub mod power {
            #[inline(always)]
            pub unsafe fn monitor(address: *const u8, extensions: u32, hints: u32) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov eax, {0:e}",
                        "mov ecx, {1:e}",
                        "mov edx, {2:e}",
                        "monitor",
                        in(reg) address,
                        in(reg) extensions,
                        in(reg) hints,
                        options(nostack, preserves_flags)
                    );
                }
            }

            #[inline(always)]
            pub unsafe fn mwait(hints: u32, extensions: u32) {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov eax, {0:e}",
                        "mov ecx, {1:e}",
                        "mwait",
                        in(reg) hints,
                        in(reg) extensions,
                        options(nostack, preserves_flags)
                    );
                }
            }
        }
    }

    pub mod stack {

        #[inline(always)]
        pub unsafe fn popw() -> u16 {
            let value: u16;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "pop {0:x}",
                    out(reg) value,
                    options(preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn pushw(value: u16) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push {0:x}",
                    in(reg) value,
                    options(preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn popl() -> u32 {
            let value: u32;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "pop {0:e}",
                    out(reg) value,
                    options(preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn pushl(value: u32) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push {0:e}",
                    in(reg) value,
                    options(preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn popq() -> u64 {
            let value: u64;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "pop {0:r}",
                    out(reg) value,
                    options(preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn pushq(value: u64) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push {0:r}",
                    in(reg) value,
                    options(preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn popad() {
            unsafe {
                core::arch::asm!(".code64", "popad", options(preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn pushad() {
            unsafe {
                core::arch::asm!(".code64", "pushad", options(preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn popf() {
            unsafe {
                core::arch::asm!(".code64", "popf", options(preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn pushf() {
            unsafe {
                core::arch::asm!(".code64", "pushf", options(preserves_flags));
            }
        }
    }

    pub mod interrupts {
        /*!
        # Interrupts

        When an interrupt fires, the cpu pushes these onto the stack, depending on CPL (current priotirty level)
        stack : [eip] [cs] [eflags] ([esp]) ([ss]) ([error_code])
        Maybe write an actual doc comment bro
        */

        pub struct InterruptFrame {
            pub rip: u64,
            pub cs: u16,
            pub rflags: u64,
            pub rsp: u64,
            pub ss: u16,
            pub error_code: u64,
        }

        #[inline(always)]
        pub unsafe fn int(vector: u8) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "int {0}",
                    in(reg_byte) vector,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn iretq() -> ! {
            unsafe { core::arch::asm!(".code64", "iretq", options(nostack, preserves_flags)) }

            unreachable!();
        }
    }

    pub mod io {

        #[inline(always)]
        pub unsafe fn outb_reg(port: u16, value: u8) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "mov dx, {0:x}",
                    "mov al, {1}",
                    "out dx, al",
                    in(reg) port,
                    in(reg_byte) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn inb_reg(port: u16) -> u8 {
            let value: u8;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "mov dx, {0:x}",
                    "in al, dx",
                    "mov {1}, al",
                    in(reg) port,
                    out(reg_byte) value,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[macro_export]
        macro_rules! outb_imm_64 {
            ($port:expr, $value:expr) => {

                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov al, {1}",
                        "out {0}, al",
                        const $port,
                        in(reg_byte) $value,
                        options(nostack, preserves_flags)
                    );
                }

            };
        }

        #[macro_export]
        macro_rules! inb_imm_64 {
            ($port:expr) => {

                let value: u8;
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "in al, {0}",
                        "mov {1}, al",
                        const $port,
                        out(reg_byte) value,
                        options(nostack, preserves_flags)
                    );
                }

                value
            };
        }

        #[inline(always)]
        pub unsafe fn outw_reg(port: u16, value: u16) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "mov dx, {0:x}",
                    "mov ax, {1:x}",
                    "out dx, ax",
                    in(reg) port,
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn inw_reg(port: u16) -> u16 {
            let value: u16;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "mov dx, {0:x}",
                    "in ax, dx",
                    "mov {1:x}, ax",
                    in(reg) port,
                    out(reg) value,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[macro_export]
        macro_rules! outw_imm_64 {
            ($port:expr, $value:expr) => {

                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov ax, {1:x}",
                        "out {0}, ax",
                        const $port,
                        in(reg) $value,
                        options(nostack, preserves_flags)
                    );
                }

            };
        }

        #[macro_export]
        macro_rules! inw_imm_64 {
            ($port:expr) => {

                let value: u16;

                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "in ax, {0}",
                        "mov {1:x}, ax",
                        const $port,
                        out(reg) value,
                        options(nostack, preserves_flags)
                    );
                }

                value
            };
        }

        #[inline(always)]
        pub unsafe fn outl_reg(port: u16, value: u32) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "mov dx, {0:x}",
                    "mov eax, {1:e}",
                    "out dx, eax",
                    in(reg) port,
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn inl_reg(port: u16) -> u32 {
            let value: u32;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "mov dx, {0:x}",
                    "in eax, dx",
                    "mov {1:e}, eax",
                    in(reg) port,
                    out(reg) value,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[macro_export]
        macro_rules! outl_imm_64 {
            ($port:expr, $value:expr) => {

                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov eax, {1:e}",
                        "out {0}, eax",
                        const $port,
                        in(reg) $value,
                        options(nostack, preserves_flags)
                    );
                }

            };
        }

        #[macro_export]
        macro_rules! inl_imm_64 {
            ($port:expr) => {

                let value: u32;

                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "in eax, {0}",
                        "mov {1:e}, eax",
                        const $port,
                        out(reg) value,
                        options(nostack, preserves_flags)
                    );
                }

                value
            };
        }

        #[inline(always)]
        pub unsafe fn outq_reg(port: u16, value: u64) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "mov dx, {0:x}",
                    "mov rax, {1:r}",
                    "out dx, rax",
                    in(reg) port,
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn inq_reg(port: u16) -> u64 {
            let value: u64;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "mov dx, {0:x}",
                    "in rax, dx",
                    "mov {1:r}, rax",
                    in(reg) port,
                    out(reg) value,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[macro_export]
        macro_rules! outq_imm_64 {
            ($port:expr, $value:expr) => {

                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "mov rax, {1:r}",
                        "out {0}, rax",
                        const $port,
                        in(reg) $value,
                        options(nostack, preserves_flags)
                    );
                }

            };
        }

        #[macro_export]
        macro_rules! inq_imm_64 {
            ($port:expr) => {

                let value: u64;

                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "in rax, {0:r}",
                        "mov {1:r}, rax",
                        const $port,
                        out(reg) value,
                        options(nostack, preserves_flags)
                    );
                }

                value
            };
        }
    }

    #[repr(C, packed)]
    pub struct FarPtr {
        pub offset: u64,
        pub selector: u16,
    }

    impl FarPtr {
        pub fn new(selector: u16, offset: u64) -> Self {
            Self {
                offset: offset,
                selector: selector,
            }
        }
    }

    pub mod memory {
        #[inline(always)]
        pub unsafe fn loadb(ptr: super::FarPtr) -> u8 {
            let value: u8;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov {0}, [{2:r}]",
                    "pop ds",
                    out(reg_byte) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn storeb(ptr: super::FarPtr, value: u8) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov [{2:r}], {0}",
                    "pop ds",
                    in(reg_byte) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn loadw(ptr: super::FarPtr) -> u16 {
            let value: u16;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov {0:x}, [{2:r}]",
                    "pop ds",
                    out(reg) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn storew(ptr: super::FarPtr, value: u16) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov [{2:r}], {0:x}",
                    "pop ds",
                    in(reg) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn loadl(ptr: super::FarPtr) -> u32 {
            let value: u32;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov {0:e}, [{2:r}]",
                    "pop ds",
                    out(reg) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn storel(ptr: super::FarPtr, value: u32) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov [{2:r}], {0:e}",
                    "pop ds",
                    in(reg) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn loadq(ptr: super::FarPtr) -> u64 {
            let value: u64;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov {0:r}, [{2:r}]",
                    "pop ds",
                    out(reg) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }

            value
        }

        #[inline(always)]
        pub unsafe fn storeq(ptr: super::FarPtr, value: u64) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "push ds",
                    "mov ds, {1:x}",
                    "mov [{2:r}], {0:r}",
                    "pop ds",
                    in(reg) value,
                    in(reg) ptr.selector,
                    in(reg) ptr.offset,
                    options(nostack, preserves_flags)
                );
            }
        }
    }

    pub mod control {
        #[inline(always)]
        pub unsafe fn call_reg(value: u64) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "call {0:r}",
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[macro_export]
        macro_rules! call_imm_64 {
            ($value:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "call {0}",
                        const $value,
                        options(nostack, preserves_flags)
                    );
                }
            };
        }

        #[inline(always)]
        pub unsafe fn call_mem(address: u64) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "call [{0:r}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[macro_export]
        macro_rules! callfar_imm_64 {
            ($selector:expr, $offset:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "call far {0}:{1}",
                        const $selector,
                        const $offset,
                        options(nostack, preserves_flags)
                    );
                }
            };
        }

        #[inline(always)]
        pub unsafe fn callfar_mem(address: u64) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "call far [{0:r}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn jmp_reg(offset: u64) -> ! {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "jmp {0:r}",
                    in(reg) offset,
                    options(nostack, preserves_flags)
                );
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! jmp_imm_64 {
            ($offset:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "jmp {0}",
                        const $offset,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn jmp_mem(address: u64) -> ! {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "jmp [{0:r}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
            unreachable!();
        }

        #[inline(always)]
        pub unsafe fn jmpfar_mem(address: u64) -> ! {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "jmp far [{0:r}]",
                    in(reg) address,
                    options(nostack, preserves_flags)
                );
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! jmpfar_imm_64 {
            ($selector:expr, $offset:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "jmp far {0}:{1}",
                        const $selector,
                        const $offset,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn ret() -> ! {
            unsafe {
                core::arch::asm!(".code64", "ret", options(nostack, preserves_flags));
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! ret_imm_64 {
            ($value:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "ret {0:r}",
                        const $value,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn retf() -> ! {
            unsafe {
                core::arch::asm!(".code64", "retf", options(nostack, preserves_flags));
            }
            unreachable!();
        }

        #[macro_export]
        macro_rules! retf_imm_64 {
            ($value:expr) => {
                unsafe {
                    core::arch::asm!(
                        ".code64",
                        "retf {0:x}",
                        const $value,
                        options(nostack, preserves_flags)
                    );
                }
                unreachable!();
            };
        }

        #[inline(always)]
        pub unsafe fn syscall() {
            unsafe {
                core::arch::asm!(".code64", "syscall", options(nostack, preserves_flags));
            }
        }

        #[inline(always)]
        pub unsafe fn sysret() -> ! {
            unsafe {
                core::arch::asm!(".code64", "sysret", options(nostack, preserves_flags));
            }
            unreachable!();
        }

        #[inline(always)]
        pub unsafe fn swapgs() {
            unsafe {
                core::arch::asm!(".code64", "swapgs", options(nostack, preserves_flags));
            }
        }
    }

    #[repr(C, packed)]
    pub struct Descriptor {
        pub limit_low: u16,
        pub base_low: u16,
        pub base_mid: u8,
        pub access: u8,
        pub granularity: u8,
        pub base_high: u8,
        pub base_upper: u32,
        pub reserved: u32,
    }

    impl Default for Descriptor {
        fn default() -> Self {
            Self {
                limit_low: 0,
                base_low: 0,
                base_mid: 0,
                access: 0,
                granularity: 0,
                base_high: 0,
                base_upper: 0,
                reserved: 0,
            }
        }
    }

    impl Descriptor {
        pub const ACCESS_TYPE: u8 = 0xF;
        pub const ACCESS_S: u8 = 0x10;
        pub const ACCESS_DPL: u8 = 0x60;
        pub const ACCESS_P: u8 = 0x80;

        pub const GRANULARITY_G: u8 = 0x80;
        pub const GRANULARITY_DB: u8 = 0x40;
        pub const GRANULARITY_L: u8 = 0x20;
        pub const GRANULARITY_AVL: u8 = 0x10;
        /// Limit high.
        pub const GRANULARITY_LH: u8 = 0x0F;
    }

    #[repr(C, packed)]
    pub struct Dtr {
        pub limit: u16,
        pub base: u64,
    }

    impl Default for Dtr {
        fn default() -> Self {
            Self { limit: 0, base: 0 }
        }
    }

    pub mod gdt {
        /*!
        # Global Descriptor Table
        */

        // [S=1] Code segment types
        pub const CODE_EXECUTE_ONLY: u8 = 0x08;
        pub const CODE_EXECUTE_READ: u8 = 0x0A;
        pub const CODE_CONFORMING: u8 = 0x0C;
        pub const CODE_CONFORMING_READ: u8 = 0x0E;

        // [S=1] Data segment types
        pub const DATA_READ_ONLY: u8 = 0x00;
        pub const DATA_READ_WRITE: u8 = 0x02;
        pub const DATA_EXPAND_DOWN: u8 = 0x04;
        pub const DATA_EXPAND_DOWN_RW: u8 = 0x06;

        // [S=0] System descriptor types
        pub const CALL_GATE: u8 = 0x0C;

        #[inline(always)]
        pub unsafe fn lgdt(gdtr: &super::Dtr) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "lgdt [{0:r}]",
                    in(reg) gdtr,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn sgdt(gdtr: &mut super::Dtr) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "sgdt [{0:r}]",
                    in(reg) gdtr,
                    options(nostack, preserves_flags)
                );
            }
        }
    }

    pub mod ldt {
        /*!
        # Local Descriptor Table
        */

        pub const TYPE: u8 = 0x02;

        #[inline(always)]
        pub unsafe fn lldt(selector: u16) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "lldt {0:x}",
                    in(reg) selector,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn sldt() -> u16 {
            let selector: u16;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "sldt {0:x}",
                    out(reg) selector,
                    options(nostack, preserves_flags)
                );
            }
            selector
        }
    }

    pub mod idt {
        /*!
        # Interrupt Descriptor Table

        ...
        */
        #[repr(C, packed)]
        pub struct Gate {
            pub offset_low: u16,
            pub selector: u16,
            pub ist: u8, // Must be 0.
            pub reserved1: u8,
            pub flags: u8,
            pub offset_mid: u16,
            pub offset_high: u32,
            pub reserved2: u32,
        }

        impl Default for Gate {
            fn default() -> Self {
                Self {
                    offset_low: 0,
                    selector: 0,
                    ist: 0,
                    reserved1: 0,
                    flags: 0,
                    offset_mid: 0,
                    offset_high: 0,
                    reserved2: 0,
                }
            }
        }

        impl Gate {
            pub const TYPE_INTERRUPT: u8 = 0x0E;
            pub const TYPE_TRAP: u8 = 0x0F;
            pub const TYPE_TASK: u8 = 0x05;
        }

        #[inline(always)]
        pub unsafe fn lidt(idtr: &super::Dtr) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "lidt [{0:r}]",
                    in(reg) idtr,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn sidt(idtr: &mut super::Dtr) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "sidt [{0:r}]",
                    in(reg) idtr,
                    options(nostack, preserves_flags)
                );
            }
        }
    }

    pub mod tss {
        /*!
        # Task State Segment

        yea idk
        */

        #[repr(C, packed)]
        pub struct Tss {
            pub reserved0: u32,
            pub rsp0: u64,
            pub rsp1: u64,
            pub rsp2: u64,
            pub reserved1: u64,
            pub ist1: u64,
            pub ist2: u64,
            pub ist3: u64,
            pub ist4: u64,
            pub ist5: u64,
            pub ist6: u64,
            pub ist7: u64,
            pub reserved2: u64,
            pub reserved3: u16,
            pub iomap_base: u16,
        }

        impl Default for Tss {
            fn default() -> Self {
                Self {
                    reserved0: 0,
                    rsp0: 0,
                    rsp1: 0,
                    rsp2: 0,
                    reserved1: 0,
                    ist1: 0,
                    ist2: 0,
                    ist3: 0,
                    ist4: 0,
                    ist5: 0,
                    ist6: 0,
                    ist7: 0,
                    reserved2: 0,
                    reserved3: 0,
                    iomap_base: 0,
                }
            }
        }

        impl Tss {
            pub const TYPE_AVAILABLE: u8 = 0x09;
            pub const TYPE_BUSY: u8 = 0x0B;

            pub const IOMAP_DISABLE: u16 = 0x68;
        }

        #[inline(always)]
        pub unsafe fn ltr(selector: u16) {
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "ltr {0:x}",
                    in(reg) selector,
                    options(nostack, preserves_flags)
                );
            }
        }

        #[inline(always)]
        pub unsafe fn str() -> u16 {
            let selector: u16;
            unsafe {
                core::arch::asm!(
                    ".code64",
                    "str {0:x}",
                    out(reg) selector,
                    options(nostack, preserves_flags)
                );
            }
            selector
        }
    }

    pub mod paging {
        /*!
        # Paging

        Write here what the paging mechanic is, and how to implement it / Whats expected
        And PDE / PTE difference
        and PAE extension
        and formats

        idk about this module tho.
        */

        pub const P: u64 = 0x1; // Present.
        pub const RW: u64 = 0x2; // Read/Write.
        pub const US: u64 = 0x4; // User/Supervisor.
        pub const PWT: u64 = 0x8; // Write through.
        pub const PCD: u64 = 0x10; // Cache disable.
        pub const A: u64 = 0x20; // Accessed.
        pub const PS: u64 = 0x80; // Page size.

        pub const PD: u64 = 0x40; // Dirty.
        pub const G: u64 = 0x100; // Global.
    }
}
