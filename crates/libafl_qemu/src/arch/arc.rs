use std::sync::OnceLock;

use enum_map::{EnumMap, enum_map};
use num_enum::{IntoPrimitive, TryFromPrimitive};
#[cfg(feature = "python")]
use pyo3::prelude::*;
pub use strum_macros::EnumIter;

use crate::{CallingConvention, QemuRWError, QemuRWErrorKind, sync_exit::ExitArgs};

#[expect(non_upper_case_globals)]
impl CallingConvention {
    pub const Default: CallingConvention = CallingConvention::Arc;
}

/// Registers for the ARC instruction set.
#[derive(IntoPrimitive, TryFromPrimitive, Debug, Copy, Clone, EnumIter)]
#[repr(i32)]
pub enum Regs {
    R0 = 0, R1 = 1, R2 = 2, R3 = 3, R4 = 4, R5 = 5,
    R6 = 6, R7 = 7, R8 = 8, R9 = 9, R10 = 10,
    R11 = 11, R12 = 12, R13 = 13, R14 = 14, R15 = 15,
    R16 = 16, R17 = 17, R18 = 18, R19 = 19, R20 = 20,
    R21 = 21, R22 = 22, R23 = 23, R24 = 24, R25 = 25,
    R26 = 26, R27 = 27, R28 = 28, R29 = 29, R30 = 30,
    R31 = 31, R58 = 32, R59 = 33, R60 = 34, R63 = 35,
}


static EXIT_ARCH_REGS: OnceLock<EnumMap<ExitArgs, Regs>> = OnceLock::new();

pub fn get_exit_arch_regs() -> &'static EnumMap<ExitArgs, Regs> {
    EXIT_ARCH_REGS.get_or_init(|| {
        enum_map! {
            ExitArgs::Ret  => Regs::R0,
            ExitArgs::Cmd  => Regs::R0,
            ExitArgs::Arg1 => Regs::R0,
            ExitArgs::Arg2 => Regs::R1,
            ExitArgs::Arg3 => Regs::R2,
            ExitArgs::Arg4 => Regs::R3,
            ExitArgs::Arg5 => Regs::R4,
            ExitArgs::Arg6 => Regs::R5,
            // ExitArgs::Arg7 => Regs::R6,
            // ExitArgs::Arg8 => Regs::R7,
        }
    })
}

/// alias registers
#[expect(non_upper_case_globals)]
impl Regs {
    pub const Gp: Regs = Regs::R26;
    pub const Fp: Regs = Regs::R27;
    pub const Sp: Regs = Regs::R28;
    pub const Ilink: Regs = Regs::R29;
    pub const Blink: Regs = Regs::R31;
    pub const Lp: Regs = Regs::R60;
    pub const Pcl: Regs = Regs::R63;
    pub const Pc: Regs = Regs::R63;
}

// /// Return an ARC ArchCapstoneBuilder
// pub fn capstone() -> capstone::arch::arc::ArchCapstoneBuilder {
//     capstone::Capstone::new().arc()
// }

pub type GuestReg = u32;

impl crate::ArchExtras for crate::CPU {
    fn read_return_address(&self) -> Result<GuestReg, QemuRWError> {
        self.read_reg(Regs::Blink)
    }

    fn write_return_address<T>(&self, val: T) -> Result<(), QemuRWError>
    where
        T: Into<GuestReg>,
    {
        self.write_reg(Regs::Blink, val)
    }

    fn read_function_argument_with_cc(
        &self,
        idx: u8,
        conv: CallingConvention,
    ) -> Result<GuestReg, QemuRWError> {
        QemuRWError::check_conv(QemuRWErrorKind::Read, CallingConvention::MipsO32, conv)?;

        let reg_id = match idx {
            0 => Regs::R0,
            1 => Regs::R1,
            2 => Regs::R2,
            3 => Regs::R3,
            4 => Regs::R4,
            5 => Regs::R5,
            6 => Regs::R6,
            7 => Regs::R7,
            r => return Err(QemuRWError::new_argument_error(QemuRWErrorKind::Read, r)),
        };

        self.read_reg(reg_id)
    }

    fn write_function_argument_with_cc<T>(
        &self,
        idx: u8,
        val: T,
        conv: CallingConvention,
    ) -> Result<(), QemuRWError>
    where
        T: Into<GuestReg>,
    {
        QemuRWError::check_conv(QemuRWErrorKind::Write, CallingConvention::MipsO32, conv)?;

        let val: GuestReg = val.into();
        let reg_id = match idx {
            0 => Regs::R0,
            1 => Regs::R1,
            2 => Regs::R2,
            3 => Regs::R3,
            4 => Regs::R4,
            5 => Regs::R5,
            6 => Regs::R6,
            7 => Regs::R7,
            r => return Err(QemuRWError::new_argument_error(QemuRWErrorKind::Read, r)),
        };

        self.write_reg(reg_id, val)
    }
}
