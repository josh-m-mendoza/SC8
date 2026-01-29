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

//! Message definitions from file `"mppt.dbc"`
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
    /// PowerInput
    PowerInput(PowerInput),
    /// PowerOutput
    PowerOutput(PowerOutput),
    /// Temperature
    Temperature(Temperature),
    /// AuxillaryPowerSupply
    AuxillaryPowerSupply(AuxillaryPowerSupply),
    /// Limits
    Limits(Limits),
    /// Status
    Status(Status),
    /// PowerConnector
    PowerConnector(PowerConnector),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        let res = match id {
            PowerInput::MESSAGE_ID => Messages::PowerInput(PowerInput::try_from(payload)?),
            PowerOutput::MESSAGE_ID => Messages::PowerOutput(PowerOutput::try_from(payload)?),
            Temperature::MESSAGE_ID => Messages::Temperature(Temperature::try_from(payload)?),
            AuxillaryPowerSupply::MESSAGE_ID => {
                Messages::AuxillaryPowerSupply(AuxillaryPowerSupply::try_from(payload)?)
            }
            Limits::MESSAGE_ID => Messages::Limits(Limits::try_from(payload)?),
            Status::MESSAGE_ID => Messages::Status(Status::try_from(payload)?),
            PowerConnector::MESSAGE_ID => {
                Messages::PowerConnector(PowerConnector::try_from(payload)?)
            }
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// PowerInput
///
/// - Standard ID: 1536 (0x600)
/// - Size: 8 bytes
/// - Transmitter: ElmarSolarMPPT
#[derive(Clone, Copy)]
pub struct PowerInput {
    raw: [u8; 8],
}

impl PowerInput {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x600) });

    pub const INPUT_VOLTAGE_MIN: i32 = 0_i32;
    pub const INPUT_VOLTAGE_MAX: i32 = 0_i32;
    pub const INPUT_CURRENT_MIN: i32 = 0_i32;
    pub const INPUT_CURRENT_MAX: i32 = 0_i32;

    /// Construct new PowerInput from values
    pub fn new(input_voltage: i32, input_current: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_input_voltage(input_voltage)?;
        res.set_input_current(input_current)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// InputVoltage
    ///
    /// Input Voltage on the MPPT
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn input_voltage(&self) -> i32 {
        self.input_voltage_raw()
    }

    /// Get raw value of InputVoltage
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn input_voltage_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of InputVoltage
    #[inline(always)]
    pub fn set_input_voltage(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: PowerInput::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// InputCurrent
    ///
    /// Input Current on the MPPT
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn input_current(&self) -> i32 {
        self.input_current_raw()
    }

    /// Get raw value of InputCurrent
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn input_current_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of InputCurrent
    #[inline(always)]
    pub fn set_input_current(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: PowerInput::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for PowerInput {
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

impl embedded_can::Frame for PowerInput {
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
impl core::fmt::Debug for PowerInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("PowerInput")
                .field("input_voltage", &self.input_voltage())
                .field("input_current", &self.input_current())
                .finish()
        } else {
            f.debug_tuple("PowerInput").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for PowerInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PowerInput {{ InputVoltage={:?} InputCurrent={:?} }}",
            self.input_voltage(),
            self.input_current(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for PowerInput {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let input_voltage = u.int_in_range(0..=0)?;
        let input_current = u.int_in_range(0..=0)?;
        PowerInput::new(input_voltage, input_current).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// PowerOutput
///
/// - Standard ID: 1537 (0x601)
/// - Size: 8 bytes
/// - Transmitter: ElmarSolarMPPT
#[derive(Clone, Copy)]
pub struct PowerOutput {
    raw: [u8; 8],
}

impl PowerOutput {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x601) });

    pub const OUTPUT_VOLTAGE_MIN: i32 = 0_i32;
    pub const OUTPUT_VOLTAGE_MAX: i32 = 0_i32;
    pub const OUTPUT_CURRENT_MIN: i32 = 0_i32;
    pub const OUTPUT_CURRENT_MAX: i32 = 0_i32;

    /// Construct new PowerOutput from values
    pub fn new(output_voltage: i32, output_current: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_output_voltage(output_voltage)?;
        res.set_output_current(output_current)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// OutputVoltage
    ///
    /// Output Voltage from the MPPT
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn output_voltage(&self) -> i32 {
        self.output_voltage_raw()
    }

    /// Get raw value of OutputVoltage
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn output_voltage_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of OutputVoltage
    #[inline(always)]
    pub fn set_output_voltage(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: PowerOutput::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// OutputCurrent
    ///
    /// Output Current from the MPPT
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn output_current(&self) -> i32 {
        self.output_current_raw()
    }

    /// Get raw value of OutputCurrent
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn output_current_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of OutputCurrent
    #[inline(always)]
    pub fn set_output_current(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: PowerOutput::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for PowerOutput {
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

impl embedded_can::Frame for PowerOutput {
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
impl core::fmt::Debug for PowerOutput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("PowerOutput")
                .field("output_voltage", &self.output_voltage())
                .field("output_current", &self.output_current())
                .finish()
        } else {
            f.debug_tuple("PowerOutput").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for PowerOutput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PowerOutput {{ OutputVoltage={:?} OutputCurrent={:?} }}",
            self.output_voltage(),
            self.output_current(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for PowerOutput {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let output_voltage = u.int_in_range(0..=0)?;
        let output_current = u.int_in_range(0..=0)?;
        PowerOutput::new(output_voltage, output_current)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Temperature
///
/// - Standard ID: 1538 (0x602)
/// - Size: 8 bytes
/// - Transmitter: ElmarSolarMPPT
#[derive(Clone, Copy)]
pub struct Temperature {
    raw: [u8; 8],
}

impl Temperature {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x602) });

    pub const MOSFET_TEMPERATURE_MIN: i32 = 0_i32;
    pub const MOSFET_TEMPERATURE_MAX: i32 = 0_i32;
    pub const CONTROLLER_TEMPERATURE_MIN: i32 = 0_i32;
    pub const CONTROLLER_TEMPERATURE_MAX: i32 = 0_i32;

    /// Construct new Temperature from values
    pub fn new(mosfet_temperature: i32, controller_temperature: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_mosfet_temperature(mosfet_temperature)?;
        res.set_controller_temperature(controller_temperature)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// MosfetTemperature
    ///
    /// Temperature as measured at the Mosfet
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn mosfet_temperature(&self) -> i32 {
        self.mosfet_temperature_raw()
    }

    /// Get raw value of MosfetTemperature
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn mosfet_temperature_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of MosfetTemperature
    #[inline(always)]
    pub fn set_mosfet_temperature(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Temperature::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// ControllerTemperature
    ///
    /// Temperature as measured at the controller
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn controller_temperature(&self) -> i32 {
        self.controller_temperature_raw()
    }

    /// Get raw value of ControllerTemperature
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn controller_temperature_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of ControllerTemperature
    #[inline(always)]
    pub fn set_controller_temperature(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Temperature::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Temperature {
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

impl embedded_can::Frame for Temperature {
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
impl core::fmt::Debug for Temperature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Temperature")
                .field("mosfet_temperature", &self.mosfet_temperature())
                .field("controller_temperature", &self.controller_temperature())
                .finish()
        } else {
            f.debug_tuple("Temperature").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Temperature {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Temperature {{ MosfetTemperature={:?} ControllerTemperature={:?} }}",
            self.mosfet_temperature(),
            self.controller_temperature(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Temperature {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let mosfet_temperature = u.int_in_range(0..=0)?;
        let controller_temperature = u.int_in_range(0..=0)?;
        Temperature::new(mosfet_temperature, controller_temperature)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// AuxillaryPowerSupply
///
/// - Standard ID: 1539 (0x603)
/// - Size: 8 bytes
/// - Transmitter: ElmarSolarMPPT
#[derive(Clone, Copy)]
pub struct AuxillaryPowerSupply {
    raw: [u8; 8],
}

impl AuxillaryPowerSupply {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x603) });

    pub const TWELVE_VOLT_MIN: i32 = 0_i32;
    pub const TWELVE_VOLT_MAX: i32 = 0_i32;
    pub const THREE_VOLT_MIN: i32 = 0_i32;
    pub const THREE_VOLT_MAX: i32 = 0_i32;

    /// Construct new AuxillaryPowerSupply from values
    pub fn new(twelve_volt: i32, three_volt: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_twelve_volt(twelve_volt)?;
        res.set_three_volt(three_volt)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// TwelveVolt
    ///
    /// Voltage as measured at the 12v power auxillary supply
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn twelve_volt(&self) -> i32 {
        self.twelve_volt_raw()
    }

    /// Get raw value of TwelveVolt
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn twelve_volt_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of TwelveVolt
    #[inline(always)]
    pub fn set_twelve_volt(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: AuxillaryPowerSupply::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// ThreeVolt
    ///
    /// Voltage as measured at the 3v power supply
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn three_volt(&self) -> i32 {
        self.three_volt_raw()
    }

    /// Get raw value of ThreeVolt
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn three_volt_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of ThreeVolt
    #[inline(always)]
    pub fn set_three_volt(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: AuxillaryPowerSupply::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for AuxillaryPowerSupply {
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

impl embedded_can::Frame for AuxillaryPowerSupply {
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
impl core::fmt::Debug for AuxillaryPowerSupply {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("AuxillaryPowerSupply")
                .field("twelve_volt", &self.twelve_volt())
                .field("three_volt", &self.three_volt())
                .finish()
        } else {
            f.debug_tuple("AuxillaryPowerSupply")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for AuxillaryPowerSupply {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxillaryPowerSupply {{ TwelveVolt={:?} ThreeVolt={:?} }}",
            self.twelve_volt(),
            self.three_volt(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for AuxillaryPowerSupply {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let twelve_volt = u.int_in_range(0..=0)?;
        let three_volt = u.int_in_range(0..=0)?;
        AuxillaryPowerSupply::new(twelve_volt, three_volt)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Limits
///
/// - Standard ID: 1540 (0x604)
/// - Size: 8 bytes
/// - Transmitter: ElmarSolarMPPT
#[derive(Clone, Copy)]
pub struct Limits {
    raw: [u8; 8],
}

impl Limits {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x604) });

    pub const MAX_OUTPUT_VOLTAGE_MIN: i32 = 0_i32;
    pub const MAX_OUTPUT_VOLTAGE_MAX: i32 = 0_i32;
    pub const MAX_INPUT_CURRENT_MIN: i32 = 0_i32;
    pub const MAX_INPUT_CURRENT_MAX: i32 = 0_i32;

    /// Construct new Limits from values
    pub fn new(max_output_voltage: i32, max_input_current: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_max_output_voltage(max_output_voltage)?;
        res.set_max_input_current(max_input_current)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// MaxOutputVoltage
    ///
    /// Maximim Output voltage configured for the device
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn max_output_voltage(&self) -> i32 {
        self.max_output_voltage_raw()
    }

    /// Get raw value of MaxOutputVoltage
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn max_output_voltage_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of MaxOutputVoltage
    #[inline(always)]
    pub fn set_max_output_voltage(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Limits::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// MaxInputCurrent
    ///
    /// Maximum Input current configured for the device
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn max_input_current(&self) -> i32 {
        self.max_input_current_raw()
    }

    /// Get raw value of MaxInputCurrent
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn max_input_current_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of MaxInputCurrent
    #[inline(always)]
    pub fn set_max_input_current(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Limits::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Limits {
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

impl embedded_can::Frame for Limits {
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
impl core::fmt::Debug for Limits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Limits")
                .field("max_output_voltage", &self.max_output_voltage())
                .field("max_input_current", &self.max_input_current())
                .finish()
        } else {
            f.debug_tuple("Limits").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Limits {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Limits {{ MaxOutputVoltage={:?} MaxInputCurrent={:?} }}",
            self.max_output_voltage(),
            self.max_input_current(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Limits {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let max_output_voltage = u.int_in_range(0..=0)?;
        let max_input_current = u.int_in_range(0..=0)?;
        Limits::new(max_output_voltage, max_input_current)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Status
///
/// - Standard ID: 1541 (0x605)
/// - Size: 8 bytes
/// - Transmitter: ElmarSolarMPPT
#[derive(Clone, Copy)]
pub struct Status {
    raw: [u8; 8],
}

impl Status {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x605) });

    pub const CAN_RX_ERROR_COUNTER_MIN: u8 = 0_u8;
    pub const CAN_RX_ERROR_COUNTER_MAX: u8 = 0_u8;
    pub const CAN_TX_ERROR_COUNTER_MIN: u8 = 0_u8;
    pub const CAN_TX_ERROR_COUNTER_MAX: u8 = 0_u8;
    pub const CAN_TX_OVERFLOW_COUNTER_MIN: u8 = 0_u8;
    pub const CAN_TX_OVERFLOW_COUNTER_MAX: u8 = 0_u8;
    pub const MODE_MIN: u8 = 0_u8;
    pub const MODE_MAX: u8 = 1_u8;
    pub const RESERVED_MIN: u8 = 0_u8;
    pub const RESERVED_MAX: u8 = 0_u8;
    pub const TEST_COUNTER_MIN: u8 = 0_u8;
    pub const TEST_COUNTER_MAX: u8 = 0_u8;

    /// Construct new Status from values
    pub fn new(
        error_reserved: bool,
        error_mosfet_overheat: bool,
        error_low_arrow_power: bool,
        error_hw_over_voltage: bool,
        error_hw_over_current: bool,
        error_battery_low: bool,
        error_battery_full: bool,
        error12v_undervoltage: bool,
        limit_output_voltage_max: bool,
        limit_mosfet_temperature: bool,
        limit_local_mppt: bool,
        limit_input_current_min: bool,
        limit_input_current_max: bool,
        limit_global_mppt: bool,
        limit_duty_cycle_max: bool,
        limit_dury_cycle_min: bool,
        can_rx_error_counter: u8,
        can_tx_error_counter: u8,
        can_tx_overflow_counter: u8,
        mode: u8,
        reserved: u8,
        test_counter: u8,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_error_reserved(error_reserved)?;
        res.set_error_mosfet_overheat(error_mosfet_overheat)?;
        res.set_error_low_arrow_power(error_low_arrow_power)?;
        res.set_error_hw_over_voltage(error_hw_over_voltage)?;
        res.set_error_hw_over_current(error_hw_over_current)?;
        res.set_error_battery_low(error_battery_low)?;
        res.set_error_battery_full(error_battery_full)?;
        res.set_error12v_undervoltage(error12v_undervoltage)?;
        res.set_limit_output_voltage_max(limit_output_voltage_max)?;
        res.set_limit_mosfet_temperature(limit_mosfet_temperature)?;
        res.set_limit_local_mppt(limit_local_mppt)?;
        res.set_limit_input_current_min(limit_input_current_min)?;
        res.set_limit_input_current_max(limit_input_current_max)?;
        res.set_limit_global_mppt(limit_global_mppt)?;
        res.set_limit_duty_cycle_max(limit_duty_cycle_max)?;
        res.set_limit_dury_cycle_min(limit_dury_cycle_min)?;
        res.set_can_rx_error_counter(can_rx_error_counter)?;
        res.set_can_tx_error_counter(can_tx_error_counter)?;
        res.set_can_tx_overflow_counter(can_tx_overflow_counter)?;
        res.set_mode(mode)?;
        res.set_reserved(reserved)?;
        res.set_test_counter(test_counter)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// ErrorReserved
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_reserved(&self) -> bool {
        self.error_reserved_raw()
    }

    /// Get raw value of ErrorReserved
    ///
    /// - Start bit: 29
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_reserved_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[29..30].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorReserved
    #[inline(always)]
    pub fn set_error_reserved(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[29..30].store_le(value);
        Ok(())
    }

    /// ErrorMosfetOverheat
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_mosfet_overheat(&self) -> bool {
        self.error_mosfet_overheat_raw()
    }

    /// Get raw value of ErrorMosfetOverheat
    ///
    /// - Start bit: 25
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_mosfet_overheat_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[25..26].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorMosfetOverheat
    #[inline(always)]
    pub fn set_error_mosfet_overheat(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[25..26].store_le(value);
        Ok(())
    }

    /// ErrorLowArrowPower
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_low_arrow_power(&self) -> bool {
        self.error_low_arrow_power_raw()
    }

    /// Get raw value of ErrorLowArrowPower
    ///
    /// - Start bit: 24
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_low_arrow_power_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[24..25].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorLowArrowPower
    #[inline(always)]
    pub fn set_error_low_arrow_power(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[24..25].store_le(value);
        Ok(())
    }

    /// ErrorHwOverVoltage
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_hw_over_voltage(&self) -> bool {
        self.error_hw_over_voltage_raw()
    }

    /// Get raw value of ErrorHwOverVoltage
    ///
    /// - Start bit: 31
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_hw_over_voltage_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[31..32].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorHwOverVoltage
    #[inline(always)]
    pub fn set_error_hw_over_voltage(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[31..32].store_le(value);
        Ok(())
    }

    /// ErrorHwOverCurrent
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_hw_over_current(&self) -> bool {
        self.error_hw_over_current_raw()
    }

    /// Get raw value of ErrorHwOverCurrent
    ///
    /// - Start bit: 30
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_hw_over_current_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[30..31].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorHwOverCurrent
    #[inline(always)]
    pub fn set_error_hw_over_current(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[30..31].store_le(value);
        Ok(())
    }

    /// ErrorBatteryLow
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_battery_low(&self) -> bool {
        self.error_battery_low_raw()
    }

    /// Get raw value of ErrorBatteryLow
    ///
    /// - Start bit: 26
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_battery_low_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[26..27].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorBatteryLow
    #[inline(always)]
    pub fn set_error_battery_low(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[26..27].store_le(value);
        Ok(())
    }

    /// ErrorBatteryFull
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_battery_full(&self) -> bool {
        self.error_battery_full_raw()
    }

    /// Get raw value of ErrorBatteryFull
    ///
    /// - Start bit: 27
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_battery_full_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[27..28].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorBatteryFull
    #[inline(always)]
    pub fn set_error_battery_full(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[27..28].store_le(value);
        Ok(())
    }

    /// Error12vUndervoltage
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error12v_undervoltage(&self) -> bool {
        self.error12v_undervoltage_raw()
    }

    /// Get raw value of Error12vUndervoltage
    ///
    /// - Start bit: 28
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error12v_undervoltage_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[28..29].load_le::<u8>();

        signal == 1
    }

    /// Set value of Error12vUndervoltage
    #[inline(always)]
    pub fn set_error12v_undervoltage(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[28..29].store_le(value);
        Ok(())
    }

    /// LimitOutputVoltageMax
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_output_voltage_max(&self) -> bool {
        self.limit_output_voltage_max_raw()
    }

    /// Get raw value of LimitOutputVoltageMax
    ///
    /// - Start bit: 34
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_output_voltage_max_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[34..35].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitOutputVoltageMax
    #[inline(always)]
    pub fn set_limit_output_voltage_max(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[34..35].store_le(value);
        Ok(())
    }

    /// LimitMosfetTemperature
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_mosfet_temperature(&self) -> bool {
        self.limit_mosfet_temperature_raw()
    }

    /// Get raw value of LimitMosfetTemperature
    ///
    /// - Start bit: 35
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_mosfet_temperature_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[35..36].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitMosfetTemperature
    #[inline(always)]
    pub fn set_limit_mosfet_temperature(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[35..36].store_le(value);
        Ok(())
    }

    /// LimitLocalMPPT
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_local_mppt(&self) -> bool {
        self.limit_local_mppt_raw()
    }

    /// Get raw value of LimitLocalMPPT
    ///
    /// - Start bit: 38
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_local_mppt_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[38..39].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitLocalMPPT
    #[inline(always)]
    pub fn set_limit_local_mppt(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[38..39].store_le(value);
        Ok(())
    }

    /// LimitInputCurrentMin
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_input_current_min(&self) -> bool {
        self.limit_input_current_min_raw()
    }

    /// Get raw value of LimitInputCurrentMin
    ///
    /// - Start bit: 32
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_input_current_min_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitInputCurrentMin
    #[inline(always)]
    pub fn set_limit_input_current_min(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
        Ok(())
    }

    /// LimitInputCurrentMax
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_input_current_max(&self) -> bool {
        self.limit_input_current_max_raw()
    }

    /// Get raw value of LimitInputCurrentMax
    ///
    /// - Start bit: 33
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_input_current_max_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[33..34].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitInputCurrentMax
    #[inline(always)]
    pub fn set_limit_input_current_max(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[33..34].store_le(value);
        Ok(())
    }

    /// LimitGlobalMPPT
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_global_mppt(&self) -> bool {
        self.limit_global_mppt_raw()
    }

    /// Get raw value of LimitGlobalMPPT
    ///
    /// - Start bit: 39
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_global_mppt_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[39..40].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitGlobalMPPT
    #[inline(always)]
    pub fn set_limit_global_mppt(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[39..40].store_le(value);
        Ok(())
    }

    /// LimitDutyCycleMax
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_duty_cycle_max(&self) -> bool {
        self.limit_duty_cycle_max_raw()
    }

    /// Get raw value of LimitDutyCycleMax
    ///
    /// - Start bit: 37
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_duty_cycle_max_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[37..38].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitDutyCycleMax
    #[inline(always)]
    pub fn set_limit_duty_cycle_max(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[37..38].store_le(value);
        Ok(())
    }

    /// LimitDuryCycleMin
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_dury_cycle_min(&self) -> bool {
        self.limit_dury_cycle_min_raw()
    }

    /// Get raw value of LimitDuryCycleMin
    ///
    /// - Start bit: 36
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_dury_cycle_min_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[36..37].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitDuryCycleMin
    #[inline(always)]
    pub fn set_limit_dury_cycle_min(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[36..37].store_le(value);
        Ok(())
    }

    /// CanRXErrorCounter
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn can_rx_error_counter(&self) -> u8 {
        self.can_rx_error_counter_raw()
    }

    /// Get raw value of CanRXErrorCounter
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn can_rx_error_counter_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of CanRXErrorCounter
    #[inline(always)]
    pub fn set_can_rx_error_counter(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Status::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }

    /// CanTXErrorCounter
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn can_tx_error_counter(&self) -> u8 {
        self.can_tx_error_counter_raw()
    }

    /// Get raw value of CanTXErrorCounter
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn can_tx_error_counter_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of CanTXErrorCounter
    #[inline(always)]
    pub fn set_can_tx_error_counter(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Status::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }

    /// CanTXOverflowCounter
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn can_tx_overflow_counter(&self) -> u8 {
        self.can_tx_overflow_counter_raw()
    }

    /// Get raw value of CanTXOverflowCounter
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn can_tx_overflow_counter_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of CanTXOverflowCounter
    #[inline(always)]
    pub fn set_can_tx_overflow_counter(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Status::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }

    /// Mode
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn mode(&self) -> u8 {
        self.mode_raw()
    }

    /// Get raw value of Mode
    ///
    /// - Start bit: 40
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mode_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Mode
    #[inline(always)]
    pub fn set_mode(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Status::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
        Ok(())
    }

    /// Reserved
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved(&self) -> u8 {
        self.reserved_raw()
    }

    /// Get raw value of Reserved
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn reserved_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Reserved
    #[inline(always)]
    pub fn set_reserved(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Status::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }

    /// TestCounter
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn test_counter(&self) -> u8 {
        self.test_counter_raw()
    }

    /// Get raw value of TestCounter
    ///
    /// - Start bit: 56
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn test_counter_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[56..64].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of TestCounter
    #[inline(always)]
    pub fn set_test_counter(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Status::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[56..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Status {
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

impl embedded_can::Frame for Status {
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
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Status")
                .field("error_reserved", &self.error_reserved())
                .field("error_mosfet_overheat", &self.error_mosfet_overheat())
                .field("error_low_arrow_power", &self.error_low_arrow_power())
                .field("error_hw_over_voltage", &self.error_hw_over_voltage())
                .field("error_hw_over_current", &self.error_hw_over_current())
                .field("error_battery_low", &self.error_battery_low())
                .field("error_battery_full", &self.error_battery_full())
                .field("error12v_undervoltage", &self.error12v_undervoltage())
                .field("limit_output_voltage_max", &self.limit_output_voltage_max())
                .field("limit_mosfet_temperature", &self.limit_mosfet_temperature())
                .field("limit_local_mppt", &self.limit_local_mppt())
                .field("limit_input_current_min", &self.limit_input_current_min())
                .field("limit_input_current_max", &self.limit_input_current_max())
                .field("limit_global_mppt", &self.limit_global_mppt())
                .field("limit_duty_cycle_max", &self.limit_duty_cycle_max())
                .field("limit_dury_cycle_min", &self.limit_dury_cycle_min())
                .field("can_rx_error_counter", &self.can_rx_error_counter())
                .field("can_tx_error_counter", &self.can_tx_error_counter())
                .field("can_tx_overflow_counter", &self.can_tx_overflow_counter())
                .field("mode", &self.mode())
                .field("reserved", &self.reserved())
                .field("test_counter", &self.test_counter())
                .finish()
        } else {
            f.debug_tuple("Status").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Status {{ ErrorReserved={:?} ErrorMosfetOverheat={:?} ErrorLowArrowPower={:?} ErrorHwOverVoltage={:?} ErrorHwOverCurrent={:?} ErrorBatteryLow={:?} ErrorBatteryFull={:?} Error12vUndervoltage={:?} LimitOutputVoltageMax={:?} LimitMosfetTemperature={:?} LimitLocalMPPT={:?} LimitInputCurrentMin={:?} LimitInputCurrentMax={:?} LimitGlobalMPPT={:?} LimitDutyCycleMax={:?} LimitDuryCycleMin={:?} CanRXErrorCounter={:?} CanTXErrorCounter={:?} CanTXOverflowCounter={:?} Mode={:?} Reserved={:?} TestCounter={:?} }}",
            self.error_reserved(),
            self.error_mosfet_overheat(),
            self.error_low_arrow_power(),
            self.error_hw_over_voltage(),
            self.error_hw_over_current(),
            self.error_battery_low(),
            self.error_battery_full(),
            self.error12v_undervoltage(),
            self.limit_output_voltage_max(),
            self.limit_mosfet_temperature(),
            self.limit_local_mppt(),
            self.limit_input_current_min(),
            self.limit_input_current_max(),
            self.limit_global_mppt(),
            self.limit_duty_cycle_max(),
            self.limit_dury_cycle_min(),
            self.can_rx_error_counter(),
            self.can_tx_error_counter(),
            self.can_tx_overflow_counter(),
            self.mode(),
            self.reserved(),
            self.test_counter(),
            );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Status {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let error_reserved = u.int_in_range(0..=1)? == 1;
        let error_mosfet_overheat = u.int_in_range(0..=1)? == 1;
        let error_low_arrow_power = u.int_in_range(0..=1)? == 1;
        let error_hw_over_voltage = u.int_in_range(0..=1)? == 1;
        let error_hw_over_current = u.int_in_range(0..=1)? == 1;
        let error_battery_low = u.int_in_range(0..=1)? == 1;
        let error_battery_full = u.int_in_range(0..=1)? == 1;
        let error12v_undervoltage = u.int_in_range(0..=1)? == 1;
        let limit_output_voltage_max = u.int_in_range(0..=1)? == 1;
        let limit_mosfet_temperature = u.int_in_range(0..=1)? == 1;
        let limit_local_mppt = u.int_in_range(0..=1)? == 1;
        let limit_input_current_min = u.int_in_range(0..=1)? == 1;
        let limit_input_current_max = u.int_in_range(0..=1)? == 1;
        let limit_global_mppt = u.int_in_range(0..=1)? == 1;
        let limit_duty_cycle_max = u.int_in_range(0..=1)? == 1;
        let limit_dury_cycle_min = u.int_in_range(0..=1)? == 1;
        let can_rx_error_counter = u.int_in_range(0..=0)?;
        let can_tx_error_counter = u.int_in_range(0..=0)?;
        let can_tx_overflow_counter = u.int_in_range(0..=0)?;
        let mode = u.int_in_range(0..=1)?;
        let reserved = u.int_in_range(0..=0)?;
        let test_counter = u.int_in_range(0..=0)?;
        Status::new(
            error_reserved,
            error_mosfet_overheat,
            error_low_arrow_power,
            error_hw_over_voltage,
            error_hw_over_current,
            error_battery_low,
            error_battery_full,
            error12v_undervoltage,
            limit_output_voltage_max,
            limit_mosfet_temperature,
            limit_local_mppt,
            limit_input_current_min,
            limit_input_current_max,
            limit_global_mppt,
            limit_duty_cycle_max,
            limit_dury_cycle_min,
            can_rx_error_counter,
            can_tx_error_counter,
            can_tx_overflow_counter,
            mode,
            reserved,
            test_counter,
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// PowerConnector
///
/// - Standard ID: 1542 (0x606)
/// - Size: 8 bytes
/// - Transmitter: ElmarSolarMPPT
#[derive(Clone, Copy)]
pub struct PowerConnector {
    raw: [u8; 8],
}

impl PowerConnector {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x606) });

    pub const OUTPUT_VOLTAGE_BATTERY_SIDE_MIN: i32 = 0_i32;
    pub const OUTPUT_VOLTAGE_BATTERY_SIDE_MAX: i32 = 0_i32;
    pub const POWER_CONNECTOR_TEMP_MIN: i32 = 0_i32;
    pub const POWER_CONNECTOR_TEMP_MAX: i32 = 0_i32;

    /// Construct new PowerConnector from values
    pub fn new(
        output_voltage_battery_side: i32,
        power_connector_temp: i32,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_output_voltage_battery_side(output_voltage_battery_side)?;
        res.set_power_connector_temp(power_connector_temp)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// OutputVoltageBatterySide
    ///
    /// Output Voltage (Battery side of fuse)
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn output_voltage_battery_side(&self) -> i32 {
        self.output_voltage_battery_side_raw()
    }

    /// Get raw value of OutputVoltageBatterySide
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn output_voltage_battery_side_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of OutputVoltageBatterySide
    #[inline(always)]
    pub fn set_output_voltage_battery_side(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: PowerConnector::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// PowerConnectorTemp
    ///
    /// Power connector temperature
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn power_connector_temp(&self) -> i32 {
        self.power_connector_temp_raw()
    }

    /// Get raw value of PowerConnectorTemp
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn power_connector_temp_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of PowerConnectorTemp
    #[inline(always)]
    pub fn set_power_connector_temp(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: PowerConnector::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for PowerConnector {
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

impl embedded_can::Frame for PowerConnector {
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
impl core::fmt::Debug for PowerConnector {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("PowerConnector")
                .field(
                    "output_voltage_battery_side",
                    &self.output_voltage_battery_side(),
                )
                .field("power_connector_temp", &self.power_connector_temp())
                .finish()
        } else {
            f.debug_tuple("PowerConnector").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for PowerConnector {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PowerConnector {{ OutputVoltageBatterySide={:?} PowerConnectorTemp={:?} }}",
            self.output_voltage_battery_side(),
            self.power_connector_temp(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for PowerConnector {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let output_voltage_battery_side = u.int_in_range(0..=0)?;
        let power_connector_temp = u.int_in_range(0..=0)?;
        PowerConnector::new(output_voltage_battery_side, power_connector_temp)
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
