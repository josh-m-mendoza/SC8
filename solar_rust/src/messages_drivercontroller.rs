// Generated code!
#![allow(unused_comparisons, unreachable_patterns, unused_imports)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::useless_conversion, clippy::unnecessary_cast)]
#![allow(
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::absurd_extreme_comparisons,
    clippy::too_many_arguments
)]
#![deny(clippy::arithmetic_side_effects)]

//! Message definitions from file `"drivercontroller.dbc"`
//!
//! - Version: `Version("")`

#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};
use bitvec::prelude::*;
use core::ops::BitOr;
use embedded_can::{ExtendedId, Id, StandardId};

/// All messages
#[derive(Clone, Debug, defmt::Format)]
pub enum Messages {
    /// IDInfo
    IdInfo(IdInfo),
    /// Drive
    Drive(Drive),
    /// Power
    Power(Power),
    /// Switch
    Switch(Switch),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        let res = match id {
            IdInfo::MESSAGE_ID => Messages::IdInfo(IdInfo::try_from(payload)?),
            Drive::MESSAGE_ID => Messages::Drive(Drive::try_from(payload)?),
            Power::MESSAGE_ID => Messages::Power(Power::try_from(payload)?),
            Switch::MESSAGE_ID => Messages::Switch(Switch::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// IDInfo
///
/// - Standard ID: 1280 (0x500)
/// - Size: 8 bytes
/// - Transmitter: DriverControl
#[derive(Clone, Copy)]
pub struct IdInfo {
    raw: [u8; 8],
}

impl IdInfo {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x500) });

    pub const TRITIUM_ID_MIN: u32 = 0_u32;
    pub const TRITIUM_ID_MAX: u32 = 0_u32;
    pub const SERIAL_NUMBER_MIN: u32 = 0_u32;
    pub const SERIAL_NUMBER_MAX: u32 = 0_u32;

    /// Construct new IDInfo from values
    pub fn new(tritium_id: u32, serial_number: u32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_tritium_id(tritium_id)?;
        res.set_serial_number(serial_number)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// TritiumID
    ///
    /// Device identifier. 0x00004003
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tritium_id(&self) -> u32 {
        self.tritium_id_raw()
    }

    /// Get raw value of TritiumID
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tritium_id_raw(&self) -> u32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();

        let factor = 1;
        u32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of TritiumID
    #[inline(always)]
    pub fn set_tritium_id(&mut self, value: u32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: IdInfo::MESSAGE_ID,
        })?;
        let value = (value / factor) as u32;

        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// SerialNumber
    ///
    /// Device serial number, allocated at manufacture.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn serial_number(&self) -> u32 {
        self.serial_number_raw()
    }

    /// Get raw value of SerialNumber
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn serial_number_raw(&self) -> u32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();

        let factor = 1;
        u32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of SerialNumber
    #[inline(always)]
    pub fn set_serial_number(&mut self, value: u32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: IdInfo::MESSAGE_ID,
        })?;
        let value = (value / factor) as u32;

        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for IdInfo {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for IdInfo {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
impl core::fmt::Debug for IdInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("IdInfo")
                .field("tritium_id", &self.tritium_id())
                .field("serial_number", &self.serial_number())
                .finish()
        } else {
            f.debug_tuple("IdInfo").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for IdInfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IdInfo {{ TritiumID={:?} SerialNumber={:?} }}",
            self.tritium_id(),
            self.serial_number(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for IdInfo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let tritium_id = u.int_in_range(0..=0)?;
        let serial_number = u.int_in_range(0..=0)?;
        IdInfo::new(tritium_id, serial_number).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Drive
///
/// - Standard ID: 1281 (0x501)
/// - Size: 8 bytes
/// - Transmitter: DriverControl
#[derive(Clone, Copy)]
pub struct Drive {
    raw: [u8; 8],
}

impl Drive {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x501) });

    pub const SETPOINT_MOTOR_VELOCITY_MIN: i32 = 0_i32;
    pub const SETPOINT_MOTOR_VELOCITY_MAX: i32 = 0_i32;
    pub const SETPOINT_MOTOR_CURRENT_MIN: i64 = 0_i64;
    pub const SETPOINT_MOTOR_CURRENT_MAX: i64 = 0_i64;

    /// Construct new Drive from values
    pub fn new(
        setpoint_motor_velocity: i32,
        setpoint_motor_current: i64,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_setpoint_motor_velocity(setpoint_motor_velocity)?;
        res.set_setpoint_motor_current(setpoint_motor_current)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// SetpointMotorVelocity
    ///
    /// Desired motor velocity set point in rpm
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "rpm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn setpoint_motor_velocity(&self) -> i32 {
        self.setpoint_motor_velocity_raw()
    }

    /// Get raw value of SetpointMotorVelocity
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn setpoint_motor_velocity_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of SetpointMotorVelocity
    #[inline(always)]
    pub fn set_setpoint_motor_velocity(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Drive::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// SetpointMotorCurrent
    ///
    /// Desired motor current set point as a percentage of maximum current setting.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn setpoint_motor_current(&self) -> i64 {
        self.setpoint_motor_current_raw()
    }

    /// Get raw value of SetpointMotorCurrent
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 100
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn setpoint_motor_current_raw(&self) -> i64 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 100;
        i64::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of SetpointMotorCurrent
    #[inline(always)]
    pub fn set_setpoint_motor_current(&mut self, value: i64) -> Result<(), CanError> {
        let factor = 100;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Drive::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Drive {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Drive {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
impl core::fmt::Debug for Drive {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Drive")
                .field("setpoint_motor_velocity", &self.setpoint_motor_velocity())
                .field("setpoint_motor_current", &self.setpoint_motor_current())
                .finish()
        } else {
            f.debug_tuple("Drive").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Drive {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Drive {{ SetpointMotorVelocity={:?} SetpointMotorCurrent={:?} }}",
            self.setpoint_motor_velocity(),
            self.setpoint_motor_current(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Drive {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let setpoint_motor_velocity = u.int_in_range(0..=0)?;
        let setpoint_motor_current = u.int_in_range(0..=0)?;
        Drive::new(setpoint_motor_velocity, setpoint_motor_current)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Power
///
/// - Standard ID: 1282 (0x502)
/// - Size: 8 bytes
/// - Transmitter: DriverControl
#[derive(Clone, Copy)]
pub struct Power {
    raw: [u8; 8],
}

impl Power {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x502) });

    pub const RESERVED_MIN: i32 = 0_i32;
    pub const RESERVED_MAX: i32 = 0_i32;
    pub const SETPOINT_BUS_CURRENT_MIN: i64 = 0_i64;
    pub const SETPOINT_BUS_CURRENT_MAX: i64 = 0_i64;

    /// Construct new Power from values
    pub fn new(reserved: i32, setpoint_bus_current: i64) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_reserved(reserved)?;
        res.set_setpoint_bus_current(setpoint_bus_current)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Reserved
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved(&self) -> i32 {
        self.reserved_raw()
    }

    /// Get raw value of Reserved
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn reserved_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Reserved
    #[inline(always)]
    pub fn set_reserved(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Power::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// SetpointBusCurrent
    ///
    /// Desired set point of current drawn from the bus by the controller  as a percentage of absolute bus current limit.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn setpoint_bus_current(&self) -> i64 {
        self.setpoint_bus_current_raw()
    }

    /// Get raw value of SetpointBusCurrent
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 100
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn setpoint_bus_current_raw(&self) -> i64 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 100;
        i64::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of SetpointBusCurrent
    #[inline(always)]
    pub fn set_setpoint_bus_current(&mut self, value: i64) -> Result<(), CanError> {
        let factor = 100;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Power::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Power {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Power {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
impl core::fmt::Debug for Power {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Power")
                .field("reserved", &self.reserved())
                .field("setpoint_bus_current", &self.setpoint_bus_current())
                .finish()
        } else {
            f.debug_tuple("Power").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Power {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Power {{ Reserved={:?} SetpointBusCurrent={:?} }}",
            self.reserved(),
            self.setpoint_bus_current(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Power {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let reserved = u.int_in_range(0..=0)?;
        let setpoint_bus_current = u.int_in_range(0..=0)?;
        Power::new(reserved, setpoint_bus_current).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Switch
///
/// - Standard ID: 1285 (0x505)
/// - Size: 8 bytes
/// - Transmitter: DriverControl
#[derive(Clone, Copy)]
pub struct Switch {
    raw: [u8; 8],
}

impl Switch {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x505) });

    pub const FLAGS_MIN: u8 = 0_u8;
    pub const FLAGS_MAX: u8 = 0_u8;
    pub const STATE_MIN: u8 = 0_u8;
    pub const STATE_MAX: u8 = 0_u8;

    /// Construct new Switch from values
    pub fn new(
        ignition_run: bool,
        flags: u8,
        state: u8,
        brake: bool,
        mode_regen: bool,
        charge_port: bool,
        ignition_start: bool,
        ignition_accesories: bool,
        mode_drive: bool,
        mode_netural: bool,
        mode_reverse: bool,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_ignition_run(ignition_run)?;
        res.set_flags(flags)?;
        res.set_state(state)?;
        res.set_brake(brake)?;
        res.set_mode_regen(mode_regen)?;
        res.set_charge_port(charge_port)?;
        res.set_ignition_start(ignition_start)?;
        res.set_ignition_accesories(ignition_accesories)?;
        res.set_mode_drive(mode_drive)?;
        res.set_mode_netural(mode_netural)?;
        res.set_mode_reverse(mode_reverse)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// IgnitionRun
    ///
    /// Ignition key is in the run position
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Selected"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ignition_run(&self) -> bool {
        self.ignition_run_raw()
    }

    /// Get raw value of IgnitionRun
    ///
    /// - Start bit: 5
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ignition_run_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[5..6].load_le::<u8>();

        signal == 1
    }

    /// Set value of IgnitionRun
    #[inline(always)]
    pub fn set_ignition_run(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[5..6].store_le(value);
        Ok(())
    }

    /// Flags
    ///
    /// Flags currently being reported by the Driver Controller, check the code for more details.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn flags(&self) -> u8 {
        self.flags_raw()
    }

    /// Get raw value of Flags
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn flags_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Flags
    #[inline(always)]
    pub fn set_flags(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Switch::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }

    /// State
    ///
    /// Latest state as being reported by the Driver Controller, check the code for more detail on valid states
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn state(&self) -> u8 {
        self.state_raw()
    }

    /// Get raw value of State
    ///
    /// - Start bit: 56
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn state_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[56..64].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of State
    #[inline(always)]
    pub fn set_state(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Switch::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[56..64].store_le(value);
        Ok(())
    }

    /// Brake
    ///
    /// Brake pedal is currently being pressed if flag is set (1 set / 0 unset)
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake(&self) -> bool {
        self.brake_raw()
    }

    /// Get raw value of Brake
    ///
    /// - Start bit: 7
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[7..8].load_le::<u8>();

        signal == 1
    }

    /// Set value of Brake
    #[inline(always)]
    pub fn set_brake(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[7..8].store_le(value);
        Ok(())
    }

    /// ModeRegen
    ///
    /// Car is regerating power from the motor if set (1 set / 0 unset)
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Selected"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn mode_regen(&self) -> bool {
        self.mode_regen_raw()
    }

    /// Get raw value of ModeRegen
    ///
    /// - Start bit: 2
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mode_regen_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();

        signal == 1
    }

    /// Set value of ModeRegen
    #[inline(always)]
    pub fn set_mode_regen(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
        Ok(())
    }

    /// ChargePort
    ///
    /// Charge point is currently open if set (1 set / 0 unset), port must be closed for car to drive.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Connected / Disconnected"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn charge_port(&self) -> bool {
        self.charge_port_raw()
    }

    /// Get raw value of ChargePort
    ///
    /// - Start bit: 8
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn charge_port_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[8..9].load_le::<u8>();

        signal == 1
    }

    /// Set value of ChargePort
    #[inline(always)]
    pub fn set_charge_port(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[8..9].store_le(value);
        Ok(())
    }

    /// IgnitionStart
    ///
    /// Ignition key is in the start position
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Selected"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ignition_start(&self) -> bool {
        self.ignition_start_raw()
    }

    /// Get raw value of IgnitionStart
    ///
    /// - Start bit: 6
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ignition_start_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[6..7].load_le::<u8>();

        signal == 1
    }

    /// Set value of IgnitionStart
    #[inline(always)]
    pub fn set_ignition_start(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[6..7].store_le(value);
        Ok(())
    }

    /// IgnitionAccesories
    ///
    /// Ignition key is in the accesories position
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Selected"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn ignition_accesories(&self) -> bool {
        self.ignition_accesories_raw()
    }

    /// Get raw value of IgnitionAccesories
    ///
    /// - Start bit: 4
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn ignition_accesories_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[4..5].load_le::<u8>();

        signal == 1
    }

    /// Set value of IgnitionAccesories
    #[inline(always)]
    pub fn set_ignition_accesories(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[4..5].store_le(value);
        Ok(())
    }

    /// ModeDrive
    ///
    /// Car is in drive mode if set (1 set / 0 unset)
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Selected"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn mode_drive(&self) -> bool {
        self.mode_drive_raw()
    }

    /// Get raw value of ModeDrive
    ///
    /// - Start bit: 3
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mode_drive_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[3..4].load_le::<u8>();

        signal == 1
    }

    /// Set value of ModeDrive
    #[inline(always)]
    pub fn set_mode_drive(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[3..4].store_le(value);
        Ok(())
    }

    /// ModeNetural
    ///
    /// Car is in netural if set (1 set / 0 unset)
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Selected"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn mode_netural(&self) -> bool {
        self.mode_netural_raw()
    }

    /// Get raw value of ModeNetural
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mode_netural_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();

        signal == 1
    }

    /// Set value of ModeNetural
    #[inline(always)]
    pub fn set_mode_netural(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }

    /// ModeReverse
    ///
    /// Car is in reverse if set (1 set / 0 unset)
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Selected"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn mode_reverse(&self) -> bool {
        self.mode_reverse_raw()
    }

    /// Get raw value of ModeReverse
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mode_reverse_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();

        signal == 1
    }

    /// Set value of ModeReverse
    #[inline(always)]
    pub fn set_mode_reverse(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Switch {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Switch {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
impl core::fmt::Debug for Switch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Switch")
                .field("ignition_run", &self.ignition_run())
                .field("flags", &self.flags())
                .field("state", &self.state())
                .field("brake", &self.brake())
                .field("mode_regen", &self.mode_regen())
                .field("charge_port", &self.charge_port())
                .field("ignition_start", &self.ignition_start())
                .field("ignition_accesories", &self.ignition_accesories())
                .field("mode_drive", &self.mode_drive())
                .field("mode_netural", &self.mode_netural())
                .field("mode_reverse", &self.mode_reverse())
                .finish()
        } else {
            f.debug_tuple("Switch").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Switch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Switch {{ IgnitionRun={:?} Flags={:?} State={:?} Brake={:?} ModeRegen={:?} ChargePort={:?} IgnitionStart={:?} IgnitionAccesories={:?} ModeDrive={:?} ModeNetural={:?} ModeReverse={:?} }}",
            self.ignition_run(),
            self.flags(),
            self.state(),
            self.brake(),
            self.mode_regen(),
            self.charge_port(),
            self.ignition_start(),
            self.ignition_accesories(),
            self.mode_drive(),
            self.mode_netural(),
            self.mode_reverse(),
            );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Switch {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let ignition_run = u.int_in_range(0..=1)? == 1;
        let flags = u.int_in_range(0..=0)?;
        let state = u.int_in_range(0..=0)?;
        let brake = u.int_in_range(0..=1)? == 1;
        let mode_regen = u.int_in_range(0..=1)? == 1;
        let charge_port = u.int_in_range(0..=1)? == 1;
        let ignition_start = u.int_in_range(0..=1)? == 1;
        let ignition_accesories = u.int_in_range(0..=1)? == 1;
        let mode_drive = u.int_in_range(0..=1)? == 1;
        let mode_netural = u.int_in_range(0..=1)? == 1;
        let mode_reverse = u.int_in_range(0..=1)? == 1;
        Switch::new(
            ignition_run,
            flags,
            state,
            brake,
            mode_regen,
            charge_port,
            ignition_start,
            ignition_accesories,
            mode_drive,
            mode_netural,
            mode_reverse,
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanError {
    UnknownMessageId(embedded_can::Id),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: embedded_can::Id,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: embedded_can::Id,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

impl core::fmt::Display for CanError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for CanError {}
#[cfg(feature = "arb")]
trait UnstructuredFloatExt {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32>;
}

#[cfg(feature = "arb")]
impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32> {
        let min = range.start();
        let max = range.end();
        let steps = u32::MAX;
        let factor = (max - min) / (steps as f32);
        let random_int: u32 = self.int_in_range(0..=steps)?;
        let random = min + factor * (random_int as f32);
        Ok(random)
    }
}
