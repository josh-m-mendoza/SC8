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

//! Message definitions from file `"motorcontroller.dbc"`
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
    /// Status
    Status(Status),
    /// BusMeasurement
    BusMeasurement(BusMeasurement),
    /// VelocityMeasurement
    VelocityMeasurement(VelocityMeasurement),
    /// PhaseCurrentMeasurement
    PhaseCurrentMeasurement(PhaseCurrentMeasurement),
    /// MotorVoltageVectorMeasurement
    MotorVoltageVectorMeasurement(MotorVoltageVectorMeasurement),
    /// MotorCurrentVectorMeasurement
    MotorCurrentVectorMeasurement(MotorCurrentVectorMeasurement),
    /// BackEMFMeasurementPrediction
    BackEmfMeasurementPrediction(BackEmfMeasurementPrediction),
    /// VoltageRail15VMeasurement
    VoltageRail15VMeasurement(VoltageRail15VMeasurement),
    /// VoltageRail3V31V9Measurement
    VoltageRail3V31v9Measurement(VoltageRail3V31v9Measurement),
    /// Reserved0A
    Reserved0A(Reserved0A),
    /// HeatsinkMotorTempMeasurement
    HeatsinkMotorTempMeasurement(HeatsinkMotorTempMeasurement),
    /// DspBoardTempMeasurement
    DspBoardTempMeasurement(DspBoardTempMeasurement),
    /// Reserved0D
    Reserved0D(Reserved0D),
    /// OdometerBusAhMeasurement
    OdometerBusAhMeasurement(OdometerBusAhMeasurement),
    /// SlipSpeedMeasurement
    SlipSpeedMeasurement(SlipSpeedMeasurement),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        let res = match id {
            IdInfo::MESSAGE_ID => Messages::IdInfo(IdInfo::try_from(payload)?),
            Status::MESSAGE_ID => Messages::Status(Status::try_from(payload)?),
            BusMeasurement::MESSAGE_ID => {
                Messages::BusMeasurement(BusMeasurement::try_from(payload)?)
            }
            VelocityMeasurement::MESSAGE_ID => {
                Messages::VelocityMeasurement(VelocityMeasurement::try_from(payload)?)
            }
            PhaseCurrentMeasurement::MESSAGE_ID => {
                Messages::PhaseCurrentMeasurement(PhaseCurrentMeasurement::try_from(payload)?)
            }
            MotorVoltageVectorMeasurement::MESSAGE_ID => Messages::MotorVoltageVectorMeasurement(
                MotorVoltageVectorMeasurement::try_from(payload)?,
            ),
            MotorCurrentVectorMeasurement::MESSAGE_ID => Messages::MotorCurrentVectorMeasurement(
                MotorCurrentVectorMeasurement::try_from(payload)?,
            ),
            BackEmfMeasurementPrediction::MESSAGE_ID => Messages::BackEmfMeasurementPrediction(
                BackEmfMeasurementPrediction::try_from(payload)?,
            ),
            VoltageRail15VMeasurement::MESSAGE_ID => {
                Messages::VoltageRail15VMeasurement(VoltageRail15VMeasurement::try_from(payload)?)
            }
            VoltageRail3V31v9Measurement::MESSAGE_ID => Messages::VoltageRail3V31v9Measurement(
                VoltageRail3V31v9Measurement::try_from(payload)?,
            ),
            Reserved0A::MESSAGE_ID => Messages::Reserved0A(Reserved0A::try_from(payload)?),
            HeatsinkMotorTempMeasurement::MESSAGE_ID => Messages::HeatsinkMotorTempMeasurement(
                HeatsinkMotorTempMeasurement::try_from(payload)?,
            ),
            DspBoardTempMeasurement::MESSAGE_ID => {
                Messages::DspBoardTempMeasurement(DspBoardTempMeasurement::try_from(payload)?)
            }
            Reserved0D::MESSAGE_ID => Messages::Reserved0D(Reserved0D::try_from(payload)?),
            OdometerBusAhMeasurement::MESSAGE_ID => {
                Messages::OdometerBusAhMeasurement(OdometerBusAhMeasurement::try_from(payload)?)
            }
            SlipSpeedMeasurement::MESSAGE_ID => {
                Messages::SlipSpeedMeasurement(SlipSpeedMeasurement::try_from(payload)?)
            }
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// IDInfo
///
/// - Standard ID: 128 (0x80)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct IdInfo {
    raw: [u8; 8],
}

impl IdInfo {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x80) });

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

/// Status
///
/// - Standard ID: 129 (0x81)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct Status {
    raw: [u8; 8],
}

impl Status {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x81) });

    pub const LIMIT_RESERVED_MIN: u16 = 0_u16;
    pub const LIMIT_RESERVED_MAX: u16 = 511_u16;
    pub const ERROR_RESERVED_MIN: u8 = 0_u8;
    pub const ERROR_RESERVED_MAX: u8 = 0_u8;
    pub const ACTIVE_MOTOR_MIN: u16 = 0_u16;
    pub const ACTIVE_MOTOR_MAX: u16 = 0_u16;
    pub const TX_ERROR_COUNT_MIN: u8 = 0_u8;
    pub const TX_ERROR_COUNT_MAX: u8 = 0_u8;
    pub const RX_ERROR_COUNT_MIN: u8 = 0_u8;
    pub const RX_ERROR_COUNT_MAX: u8 = 0_u8;

    /// Construct new Status from values
    pub fn new(
        limit_reserved: u16,
        limit_ipm_or_motor_temp: bool,
        limit_bus_voltage_lower: bool,
        limit_bus_voltage_upper: bool,
        limit_bus_current: bool,
        limit_velocity: bool,
        limit_motor_current: bool,
        limit_output_voltage_pwm: bool,
        error_reserved: u8,
        error_motor_over_speed: bool,
        error_desaturation_fault: bool,
        error15v_rail_under_voltage: bool,
        error_config_read: bool,
        error_watchdog_caused_last_reset: bool,
        error_bad_motor_position_hall_seq: bool,
        error_dc_bus_over_voltage: bool,
        error_software_over_current: bool,
        error_hardware_over_current: bool,
        active_motor: u16,
        tx_error_count: u8,
        rx_error_count: u8,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_limit_reserved(limit_reserved)?;
        res.set_limit_ipm_or_motor_temp(limit_ipm_or_motor_temp)?;
        res.set_limit_bus_voltage_lower(limit_bus_voltage_lower)?;
        res.set_limit_bus_voltage_upper(limit_bus_voltage_upper)?;
        res.set_limit_bus_current(limit_bus_current)?;
        res.set_limit_velocity(limit_velocity)?;
        res.set_limit_motor_current(limit_motor_current)?;
        res.set_limit_output_voltage_pwm(limit_output_voltage_pwm)?;
        res.set_error_reserved(error_reserved)?;
        res.set_error_motor_over_speed(error_motor_over_speed)?;
        res.set_error_desaturation_fault(error_desaturation_fault)?;
        res.set_error15v_rail_under_voltage(error15v_rail_under_voltage)?;
        res.set_error_config_read(error_config_read)?;
        res.set_error_watchdog_caused_last_reset(error_watchdog_caused_last_reset)?;
        res.set_error_bad_motor_position_hall_seq(error_bad_motor_position_hall_seq)?;
        res.set_error_dc_bus_over_voltage(error_dc_bus_over_voltage)?;
        res.set_error_software_over_current(error_software_over_current)?;
        res.set_error_hardware_over_current(error_hardware_over_current)?;
        res.set_active_motor(active_motor)?;
        res.set_tx_error_count(tx_error_count)?;
        res.set_rx_error_count(rx_error_count)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// LimitReserved
    ///
    /// - Min: 0
    /// - Max: 511
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_reserved(&self) -> u16 {
        self.limit_reserved_raw()
    }

    /// Get raw value of LimitReserved
    ///
    /// - Start bit: 7
    /// - Signal size: 9 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_reserved_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[7..16].load_le::<u16>();

        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of LimitReserved
    #[inline(always)]
    pub fn set_limit_reserved(&mut self, value: u16) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Status::MESSAGE_ID,
        })?;
        let value = (value / factor) as u16;

        self.raw.view_bits_mut::<Lsb0>()[7..16].store_le(value);
        Ok(())
    }

    /// LimitIpmOrMotorTemp
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_ipm_or_motor_temp(&self) -> bool {
        self.limit_ipm_or_motor_temp_raw()
    }

    /// Get raw value of LimitIpmOrMotorTemp
    ///
    /// - Start bit: 6
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_ipm_or_motor_temp_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[6..7].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitIpmOrMotorTemp
    #[inline(always)]
    pub fn set_limit_ipm_or_motor_temp(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[6..7].store_le(value);
        Ok(())
    }

    /// LimitBusVoltageLower
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_bus_voltage_lower(&self) -> bool {
        self.limit_bus_voltage_lower_raw()
    }

    /// Get raw value of LimitBusVoltageLower
    ///
    /// - Start bit: 5
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_bus_voltage_lower_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[5..6].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitBusVoltageLower
    #[inline(always)]
    pub fn set_limit_bus_voltage_lower(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[5..6].store_le(value);
        Ok(())
    }

    /// LimitBusVoltageUpper
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_bus_voltage_upper(&self) -> bool {
        self.limit_bus_voltage_upper_raw()
    }

    /// Get raw value of LimitBusVoltageUpper
    ///
    /// - Start bit: 4
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_bus_voltage_upper_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[4..5].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitBusVoltageUpper
    #[inline(always)]
    pub fn set_limit_bus_voltage_upper(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[4..5].store_le(value);
        Ok(())
    }

    /// LimitBusCurrent
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_bus_current(&self) -> bool {
        self.limit_bus_current_raw()
    }

    /// Get raw value of LimitBusCurrent
    ///
    /// - Start bit: 3
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_bus_current_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[3..4].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitBusCurrent
    #[inline(always)]
    pub fn set_limit_bus_current(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[3..4].store_le(value);
        Ok(())
    }

    /// LimitVelocity
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_velocity(&self) -> bool {
        self.limit_velocity_raw()
    }

    /// Get raw value of LimitVelocity
    ///
    /// - Start bit: 2
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_velocity_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitVelocity
    #[inline(always)]
    pub fn set_limit_velocity(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
        Ok(())
    }

    /// LimitMotorCurrent
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_motor_current(&self) -> bool {
        self.limit_motor_current_raw()
    }

    /// Get raw value of LimitMotorCurrent
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_motor_current_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitMotorCurrent
    #[inline(always)]
    pub fn set_limit_motor_current(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }

    /// LimitOutputVoltagePWM
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn limit_output_voltage_pwm(&self) -> bool {
        self.limit_output_voltage_pwm_raw()
    }

    /// Get raw value of LimitOutputVoltagePWM
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn limit_output_voltage_pwm_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();

        signal == 1
    }

    /// Set value of LimitOutputVoltagePWM
    #[inline(always)]
    pub fn set_limit_output_voltage_pwm(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }

    /// ErrorReserved
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_reserved(&self) -> u8 {
        self.error_reserved_raw()
    }

    /// Get raw value of ErrorReserved
    ///
    /// - Start bit: 25
    /// - Signal size: 7 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_reserved_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[25..32].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of ErrorReserved
    #[inline(always)]
    pub fn set_error_reserved(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Status::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[25..32].store_le(value);
        Ok(())
    }

    /// ErrorMotorOverSpeed
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_motor_over_speed(&self) -> bool {
        self.error_motor_over_speed_raw()
    }

    /// Get raw value of ErrorMotorOverSpeed
    ///
    /// - Start bit: 24
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_motor_over_speed_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[24..25].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorMotorOverSpeed
    #[inline(always)]
    pub fn set_error_motor_over_speed(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[24..25].store_le(value);
        Ok(())
    }

    /// ErrorDesaturationFault
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_desaturation_fault(&self) -> bool {
        self.error_desaturation_fault_raw()
    }

    /// Get raw value of ErrorDesaturationFault
    ///
    /// - Start bit: 23
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_desaturation_fault_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[23..24].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorDesaturationFault
    #[inline(always)]
    pub fn set_error_desaturation_fault(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[23..24].store_le(value);
        Ok(())
    }

    /// Error15vRailUnderVoltage
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error15v_rail_under_voltage(&self) -> bool {
        self.error15v_rail_under_voltage_raw()
    }

    /// Get raw value of Error15vRailUnderVoltage
    ///
    /// - Start bit: 22
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error15v_rail_under_voltage_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[22..23].load_le::<u8>();

        signal == 1
    }

    /// Set value of Error15vRailUnderVoltage
    #[inline(always)]
    pub fn set_error15v_rail_under_voltage(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[22..23].store_le(value);
        Ok(())
    }

    /// ErrorConfigRead
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_config_read(&self) -> bool {
        self.error_config_read_raw()
    }

    /// Get raw value of ErrorConfigRead
    ///
    /// - Start bit: 21
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_config_read_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[21..22].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorConfigRead
    #[inline(always)]
    pub fn set_error_config_read(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[21..22].store_le(value);
        Ok(())
    }

    /// ErrorWatchdogCausedLastReset
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_watchdog_caused_last_reset(&self) -> bool {
        self.error_watchdog_caused_last_reset_raw()
    }

    /// Get raw value of ErrorWatchdogCausedLastReset
    ///
    /// - Start bit: 20
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_watchdog_caused_last_reset_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[20..21].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorWatchdogCausedLastReset
    #[inline(always)]
    pub fn set_error_watchdog_caused_last_reset(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[20..21].store_le(value);
        Ok(())
    }

    /// ErrorBadMotorPositionHallSeq
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_bad_motor_position_hall_seq(&self) -> bool {
        self.error_bad_motor_position_hall_seq_raw()
    }

    /// Get raw value of ErrorBadMotorPositionHallSeq
    ///
    /// - Start bit: 19
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_bad_motor_position_hall_seq_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[19..20].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorBadMotorPositionHallSeq
    #[inline(always)]
    pub fn set_error_bad_motor_position_hall_seq(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[19..20].store_le(value);
        Ok(())
    }

    /// ErrorDcBusOverVoltage
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_dc_bus_over_voltage(&self) -> bool {
        self.error_dc_bus_over_voltage_raw()
    }

    /// Get raw value of ErrorDcBusOverVoltage
    ///
    /// - Start bit: 18
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_dc_bus_over_voltage_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[18..19].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorDcBusOverVoltage
    #[inline(always)]
    pub fn set_error_dc_bus_over_voltage(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[18..19].store_le(value);
        Ok(())
    }

    /// ErrorSoftwareOverCurrent
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_software_over_current(&self) -> bool {
        self.error_software_over_current_raw()
    }

    /// Get raw value of ErrorSoftwareOverCurrent
    ///
    /// - Start bit: 17
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_software_over_current_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[17..18].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorSoftwareOverCurrent
    #[inline(always)]
    pub fn set_error_software_over_current(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[17..18].store_le(value);
        Ok(())
    }

    /// ErrorHardwareOverCurrent
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "On / Off"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_hardware_over_current(&self) -> bool {
        self.error_hardware_over_current_raw()
    }

    /// Get raw value of ErrorHardwareOverCurrent
    ///
    /// - Start bit: 16
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_hardware_over_current_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[16..17].load_le::<u8>();

        signal == 1
    }

    /// Set value of ErrorHardwareOverCurrent
    #[inline(always)]
    pub fn set_error_hardware_over_current(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[16..17].store_le(value);
        Ok(())
    }

    /// ActiveMotor
    ///
    /// The index of the active motor currently being used.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn active_motor(&self) -> u16 {
        self.active_motor_raw()
    }

    /// Get raw value of ActiveMotor
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn active_motor_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();

        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of ActiveMotor
    #[inline(always)]
    pub fn set_active_motor(&mut self, value: u16) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Status::MESSAGE_ID,
        })?;
        let value = (value / factor) as u16;

        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }

    /// TxErrorCount
    ///
    /// The DSP CAN transmission error counter (CAN 2.0)
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tx_error_count(&self) -> u8 {
        self.tx_error_count_raw()
    }

    /// Get raw value of TxErrorCount
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tx_error_count_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of TxErrorCount
    #[inline(always)]
    pub fn set_tx_error_count(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Status::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }

    /// RxErrorCount
    ///
    /// The DSP CAN receive error counter (CAN 2.0)
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rx_error_count(&self) -> u8 {
        self.rx_error_count_raw()
    }

    /// Get raw value of RxErrorCount
    ///
    /// - Start bit: 56
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rx_error_count_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[56..64].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of RxErrorCount
    #[inline(always)]
    pub fn set_rx_error_count(&mut self, value: u8) -> Result<(), CanError> {
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
                .field("limit_reserved", &self.limit_reserved())
                .field("limit_ipm_or_motor_temp", &self.limit_ipm_or_motor_temp())
                .field("limit_bus_voltage_lower", &self.limit_bus_voltage_lower())
                .field("limit_bus_voltage_upper", &self.limit_bus_voltage_upper())
                .field("limit_bus_current", &self.limit_bus_current())
                .field("limit_velocity", &self.limit_velocity())
                .field("limit_motor_current", &self.limit_motor_current())
                .field("limit_output_voltage_pwm", &self.limit_output_voltage_pwm())
                .field("error_reserved", &self.error_reserved())
                .field("error_motor_over_speed", &self.error_motor_over_speed())
                .field("error_desaturation_fault", &self.error_desaturation_fault())
                .field(
                    "error15v_rail_under_voltage",
                    &self.error15v_rail_under_voltage(),
                )
                .field("error_config_read", &self.error_config_read())
                .field(
                    "error_watchdog_caused_last_reset",
                    &self.error_watchdog_caused_last_reset(),
                )
                .field(
                    "error_bad_motor_position_hall_seq",
                    &self.error_bad_motor_position_hall_seq(),
                )
                .field(
                    "error_dc_bus_over_voltage",
                    &self.error_dc_bus_over_voltage(),
                )
                .field(
                    "error_software_over_current",
                    &self.error_software_over_current(),
                )
                .field(
                    "error_hardware_over_current",
                    &self.error_hardware_over_current(),
                )
                .field("active_motor", &self.active_motor())
                .field("tx_error_count", &self.tx_error_count())
                .field("rx_error_count", &self.rx_error_count())
                .finish()
        } else {
            f.debug_tuple("Status").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Status {{ LimitReserved={:?} LimitIpmOrMotorTemp={:?} LimitBusVoltageLower={:?} LimitBusVoltageUpper={:?} LimitBusCurrent={:?} LimitVelocity={:?} LimitMotorCurrent={:?} LimitOutputVoltagePWM={:?} ErrorReserved={:?} ErrorMotorOverSpeed={:?} ErrorDesaturationFault={:?} Error15vRailUnderVoltage={:?} ErrorConfigRead={:?} ErrorWatchdogCausedLastReset={:?} ErrorBadMotorPositionHallSeq={:?} ErrorDcBusOverVoltage={:?} ErrorSoftwareOverCurrent={:?} ErrorHardwareOverCurrent={:?} ActiveMotor={:?} TxErrorCount={:?} RxErrorCount={:?} }}",
            self.limit_reserved(),
            self.limit_ipm_or_motor_temp(),
            self.limit_bus_voltage_lower(),
            self.limit_bus_voltage_upper(),
            self.limit_bus_current(),
            self.limit_velocity(),
            self.limit_motor_current(),
            self.limit_output_voltage_pwm(),
            self.error_reserved(),
            self.error_motor_over_speed(),
            self.error_desaturation_fault(),
            self.error15v_rail_under_voltage(),
            self.error_config_read(),
            self.error_watchdog_caused_last_reset(),
            self.error_bad_motor_position_hall_seq(),
            self.error_dc_bus_over_voltage(),
            self.error_software_over_current(),
            self.error_hardware_over_current(),
            self.active_motor(),
            self.tx_error_count(),
            self.rx_error_count(),
            );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Status {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let limit_reserved = u.int_in_range(0..=511)?;
        let limit_ipm_or_motor_temp = u.int_in_range(0..=1)? == 1;
        let limit_bus_voltage_lower = u.int_in_range(0..=1)? == 1;
        let limit_bus_voltage_upper = u.int_in_range(0..=1)? == 1;
        let limit_bus_current = u.int_in_range(0..=1)? == 1;
        let limit_velocity = u.int_in_range(0..=1)? == 1;
        let limit_motor_current = u.int_in_range(0..=1)? == 1;
        let limit_output_voltage_pwm = u.int_in_range(0..=1)? == 1;
        let error_reserved = u.int_in_range(0..=0)?;
        let error_motor_over_speed = u.int_in_range(0..=1)? == 1;
        let error_desaturation_fault = u.int_in_range(0..=1)? == 1;
        let error15v_rail_under_voltage = u.int_in_range(0..=1)? == 1;
        let error_config_read = u.int_in_range(0..=1)? == 1;
        let error_watchdog_caused_last_reset = u.int_in_range(0..=1)? == 1;
        let error_bad_motor_position_hall_seq = u.int_in_range(0..=1)? == 1;
        let error_dc_bus_over_voltage = u.int_in_range(0..=1)? == 1;
        let error_software_over_current = u.int_in_range(0..=1)? == 1;
        let error_hardware_over_current = u.int_in_range(0..=1)? == 1;
        let active_motor = u.int_in_range(0..=0)?;
        let tx_error_count = u.int_in_range(0..=0)?;
        let rx_error_count = u.int_in_range(0..=0)?;
        Status::new(
            limit_reserved,
            limit_ipm_or_motor_temp,
            limit_bus_voltage_lower,
            limit_bus_voltage_upper,
            limit_bus_current,
            limit_velocity,
            limit_motor_current,
            limit_output_voltage_pwm,
            error_reserved,
            error_motor_over_speed,
            error_desaturation_fault,
            error15v_rail_under_voltage,
            error_config_read,
            error_watchdog_caused_last_reset,
            error_bad_motor_position_hall_seq,
            error_dc_bus_over_voltage,
            error_software_over_current,
            error_hardware_over_current,
            active_motor,
            tx_error_count,
            rx_error_count,
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BusMeasurement
///
/// - Standard ID: 130 (0x82)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct BusMeasurement {
    raw: [u8; 8],
}

impl BusMeasurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x82) });

    pub const BUS_VOLTAGE_MIN: i32 = 0_i32;
    pub const BUS_VOLTAGE_MAX: i32 = 0_i32;
    pub const BUS_CURRENT_MIN: i32 = 0_i32;
    pub const BUS_CURRENT_MAX: i32 = 0_i32;

    /// Construct new BusMeasurement from values
    pub fn new(bus_voltage: i32, bus_current: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_bus_voltage(bus_voltage)?;
        res.set_bus_current(bus_current)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// BusVoltage
    ///
    /// DC bus voltage at the controller.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bus_voltage(&self) -> i32 {
        self.bus_voltage_raw()
    }

    /// Get raw value of BusVoltage
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn bus_voltage_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of BusVoltage
    #[inline(always)]
    pub fn set_bus_voltage(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: BusMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// BusCurrent
    ///
    /// Current drawn from the DC bus by the controller.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bus_current(&self) -> i32 {
        self.bus_current_raw()
    }

    /// Get raw value of BusCurrent
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn bus_current_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of BusCurrent
    #[inline(always)]
    pub fn set_bus_current(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: BusMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for BusMeasurement {
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

impl embedded_can::Frame for BusMeasurement {
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
impl core::fmt::Debug for BusMeasurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BusMeasurement")
                .field("bus_voltage", &self.bus_voltage())
                .field("bus_current", &self.bus_current())
                .finish()
        } else {
            f.debug_tuple("BusMeasurement").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for BusMeasurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BusMeasurement {{ BusVoltage={:?} BusCurrent={:?} }}",
            self.bus_voltage(),
            self.bus_current(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BusMeasurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let bus_voltage = u.int_in_range(0..=0)?;
        let bus_current = u.int_in_range(0..=0)?;
        BusMeasurement::new(bus_voltage, bus_current).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// VelocityMeasurement
///
/// - Standard ID: 131 (0x83)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct VelocityMeasurement {
    raw: [u8; 8],
}

impl VelocityMeasurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x83) });

    pub const MOTOR_VELOCITY_MIN: i32 = 0_i32;
    pub const MOTOR_VELOCITY_MAX: i32 = 0_i32;
    pub const VEHICLE_VELOCITY_MIN: i32 = 0_i32;
    pub const VEHICLE_VELOCITY_MAX: i32 = 0_i32;

    /// Construct new VelocityMeasurement from values
    pub fn new(motor_velocity: i32, vehicle_velocity: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_motor_velocity(motor_velocity)?;
        res.set_vehicle_velocity(vehicle_velocity)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// MotorVelocity
    ///
    /// Motor angular frequency in revolutions per minute.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "rpm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn motor_velocity(&self) -> i32 {
        self.motor_velocity_raw()
    }

    /// Get raw value of MotorVelocity
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn motor_velocity_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of MotorVelocity
    #[inline(always)]
    pub fn set_motor_velocity(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: VelocityMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// VehicleVelocity
    ///
    /// Vehicle velocity in metres / second.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn vehicle_velocity(&self) -> i32 {
        self.vehicle_velocity_raw()
    }

    /// Get raw value of VehicleVelocity
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn vehicle_velocity_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of VehicleVelocity
    #[inline(always)]
    pub fn set_vehicle_velocity(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: VelocityMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for VelocityMeasurement {
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

impl embedded_can::Frame for VelocityMeasurement {
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
impl core::fmt::Debug for VelocityMeasurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("VelocityMeasurement")
                .field("motor_velocity", &self.motor_velocity())
                .field("vehicle_velocity", &self.vehicle_velocity())
                .finish()
        } else {
            f.debug_tuple("VelocityMeasurement")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for VelocityMeasurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VelocityMeasurement {{ MotorVelocity={:?} VehicleVelocity={:?} }}",
            self.motor_velocity(),
            self.vehicle_velocity(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for VelocityMeasurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let motor_velocity = u.int_in_range(0..=0)?;
        let vehicle_velocity = u.int_in_range(0..=0)?;
        VelocityMeasurement::new(motor_velocity, vehicle_velocity)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// PhaseCurrentMeasurement
///
/// - Standard ID: 132 (0x84)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct PhaseCurrentMeasurement {
    raw: [u8; 8],
}

impl PhaseCurrentMeasurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x84) });

    pub const PHASE_CURRENT_B_MIN: i32 = 0_i32;
    pub const PHASE_CURRENT_B_MAX: i32 = 0_i32;
    pub const PHASE_CURRENT_C_MIN: i32 = 0_i32;
    pub const PHASE_CURRENT_C_MAX: i32 = 0_i32;

    /// Construct new PhaseCurrentMeasurement from values
    pub fn new(phase_current_b: i32, phase_current_c: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_phase_current_b(phase_current_b)?;
        res.set_phase_current_c(phase_current_c)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// PhaseCurrentB
    ///
    /// RMS current in motor Phase B.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "A_rms"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn phase_current_b(&self) -> i32 {
        self.phase_current_b_raw()
    }

    /// Get raw value of PhaseCurrentB
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn phase_current_b_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of PhaseCurrentB
    #[inline(always)]
    pub fn set_phase_current_b(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: PhaseCurrentMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// PhaseCurrentC
    ///
    /// RMS current in motor Phase C.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "A_rms"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn phase_current_c(&self) -> i32 {
        self.phase_current_c_raw()
    }

    /// Get raw value of PhaseCurrentC
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn phase_current_c_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of PhaseCurrentC
    #[inline(always)]
    pub fn set_phase_current_c(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: PhaseCurrentMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for PhaseCurrentMeasurement {
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

impl embedded_can::Frame for PhaseCurrentMeasurement {
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
impl core::fmt::Debug for PhaseCurrentMeasurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("PhaseCurrentMeasurement")
                .field("phase_current_b", &self.phase_current_b())
                .field("phase_current_c", &self.phase_current_c())
                .finish()
        } else {
            f.debug_tuple("PhaseCurrentMeasurement")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for PhaseCurrentMeasurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PhaseCurrentMeasurement {{ PhaseCurrentB={:?} PhaseCurrentC={:?} }}",
            self.phase_current_b(),
            self.phase_current_c(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for PhaseCurrentMeasurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let phase_current_b = u.int_in_range(0..=0)?;
        let phase_current_c = u.int_in_range(0..=0)?;
        PhaseCurrentMeasurement::new(phase_current_b, phase_current_c)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// MotorVoltageVectorMeasurement
///
/// - Standard ID: 133 (0x85)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct MotorVoltageVectorMeasurement {
    raw: [u8; 8],
}

impl MotorVoltageVectorMeasurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x85) });

    pub const VQ_MIN: i32 = 0_i32;
    pub const VQ_MAX: i32 = 0_i32;
    pub const VD_MIN: i32 = 0_i32;
    pub const VD_MAX: i32 = 0_i32;

    /// Construct new MotorVoltageVectorMeasurement from values
    pub fn new(vq: i32, vd: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_vq(vq)?;
        res.set_vd(vd)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Vq
    ///
    /// Imaginary component of the applied non-rotating voltage vector to the motor.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn vq(&self) -> i32 {
        self.vq_raw()
    }

    /// Get raw value of Vq
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn vq_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Vq
    #[inline(always)]
    pub fn set_vq(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: MotorVoltageVectorMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// Vd
    ///
    /// Real component of the applied non-rotating voltage vector to the motor.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn vd(&self) -> i32 {
        self.vd_raw()
    }

    /// Get raw value of Vd
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn vd_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Vd
    #[inline(always)]
    pub fn set_vd(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: MotorVoltageVectorMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for MotorVoltageVectorMeasurement {
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

impl embedded_can::Frame for MotorVoltageVectorMeasurement {
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
impl core::fmt::Debug for MotorVoltageVectorMeasurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("MotorVoltageVectorMeasurement")
                .field("vq", &self.vq())
                .field("vd", &self.vd())
                .finish()
        } else {
            f.debug_tuple("MotorVoltageVectorMeasurement")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for MotorVoltageVectorMeasurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MotorVoltageVectorMeasurement {{ Vq={:?} Vd={:?} }}",
            self.vq(),
            self.vd(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for MotorVoltageVectorMeasurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let vq = u.int_in_range(0..=0)?;
        let vd = u.int_in_range(0..=0)?;
        MotorVoltageVectorMeasurement::new(vq, vd).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// MotorCurrentVectorMeasurement
///
/// - Standard ID: 134 (0x86)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct MotorCurrentVectorMeasurement {
    raw: [u8; 8],
}

impl MotorCurrentVectorMeasurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x86) });

    pub const IQ_MIN: i32 = 0_i32;
    pub const IQ_MAX: i32 = 0_i32;
    pub const ID_MIN: i32 = 0_i32;
    pub const ID_MAX: i32 = 0_i32;

    /// Construct new MotorCurrentVectorMeasurement from values
    pub fn new(iq: i32, id: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_iq(iq)?;
        res.set_id(id)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Iq
    ///
    /// Imaginary component of the applied non-rotating current vector to the motor. This current produces torque in the motor and should be in phase with the back-EMF of the motor
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn iq(&self) -> i32 {
        self.iq_raw()
    }

    /// Get raw value of Iq
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn iq_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Iq
    #[inline(always)]
    pub fn set_iq(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: MotorCurrentVectorMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// Id
    ///
    /// Real component of the applied non-rotating current vector to the motor. This vector represents the field current of the motor.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.id_raw()
    }

    /// Get raw value of Id
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn id_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Id
    #[inline(always)]
    pub fn set_id(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: MotorCurrentVectorMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for MotorCurrentVectorMeasurement {
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

impl embedded_can::Frame for MotorCurrentVectorMeasurement {
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
impl core::fmt::Debug for MotorCurrentVectorMeasurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("MotorCurrentVectorMeasurement")
                .field("iq", &self.iq())
                .field("id", &self.id())
                .finish()
        } else {
            f.debug_tuple("MotorCurrentVectorMeasurement")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for MotorCurrentVectorMeasurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MotorCurrentVectorMeasurement {{ Iq={:?} Id={:?} }}",
            self.iq(),
            self.id(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for MotorCurrentVectorMeasurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let iq = u.int_in_range(0..=0)?;
        let id = u.int_in_range(0..=0)?;
        MotorCurrentVectorMeasurement::new(iq, id).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BackEMFMeasurementPrediction
///
/// - Standard ID: 135 (0x87)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct BackEmfMeasurementPrediction {
    raw: [u8; 8],
}

impl BackEmfMeasurementPrediction {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x87) });

    pub const BEM_FQ_MIN: i32 = 0_i32;
    pub const BEM_FQ_MAX: i32 = 0_i32;
    pub const BEM_FD_MIN: i32 = 0_i32;
    pub const BEM_FD_MAX: i32 = 0_i32;

    /// Construct new BackEMFMeasurementPrediction from values
    pub fn new(bem_fq: i32, bem_fd: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_bem_fq(bem_fq)?;
        res.set_bem_fd(bem_fd)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// BEMFq
    ///
    /// The peak of the phase to neutral motor voltage.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bem_fq(&self) -> i32 {
        self.bem_fq_raw()
    }

    /// Get raw value of BEMFq
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn bem_fq_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of BEMFq
    #[inline(always)]
    pub fn set_bem_fq(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: BackEmfMeasurementPrediction::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// BEMFd
    ///
    /// By definition this value is always 0V.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bem_fd(&self) -> i32 {
        self.bem_fd_raw()
    }

    /// Get raw value of BEMFd
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn bem_fd_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of BEMFd
    #[inline(always)]
    pub fn set_bem_fd(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: BackEmfMeasurementPrediction::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for BackEmfMeasurementPrediction {
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

impl embedded_can::Frame for BackEmfMeasurementPrediction {
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
impl core::fmt::Debug for BackEmfMeasurementPrediction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BackEmfMeasurementPrediction")
                .field("bem_fq", &self.bem_fq())
                .field("bem_fd", &self.bem_fd())
                .finish()
        } else {
            f.debug_tuple("BackEmfMeasurementPrediction")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for BackEmfMeasurementPrediction {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BackEmfMeasurementPrediction {{ BEMFq={:?} BEMFd={:?} }}",
            self.bem_fq(),
            self.bem_fd(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BackEmfMeasurementPrediction {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let bem_fq = u.int_in_range(0..=0)?;
        let bem_fd = u.int_in_range(0..=0)?;
        BackEmfMeasurementPrediction::new(bem_fq, bem_fd)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// VoltageRail15VMeasurement
///
/// - Standard ID: 136 (0x88)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct VoltageRail15VMeasurement {
    raw: [u8; 8],
}

impl VoltageRail15VMeasurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x88) });

    pub const SUPPLY15_V_MIN: i32 = 0_i32;
    pub const SUPPLY15_V_MAX: i32 = 0_i32;
    pub const RESERVED_SUPPLY15_V_MIN: i32 = 0_i32;
    pub const RESERVED_SUPPLY15_V_MAX: i32 = 0_i32;

    /// Construct new VoltageRail15VMeasurement from values
    pub fn new(supply15_v: i32, reserved_supply15_v: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_supply15_v(supply15_v)?;
        res.set_reserved_supply15_v(reserved_supply15_v)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Supply15V
    ///
    /// Actual voltage level of the 15V power rail.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn supply15_v(&self) -> i32 {
        self.supply15_v_raw()
    }

    /// Get raw value of Supply15V
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn supply15_v_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Supply15V
    #[inline(always)]
    pub fn set_supply15_v(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: VoltageRail15VMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }

    /// ReservedSupply15V
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved_supply15_v(&self) -> i32 {
        self.reserved_supply15_v_raw()
    }

    /// Get raw value of ReservedSupply15V
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn reserved_supply15_v_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of ReservedSupply15V
    #[inline(always)]
    pub fn set_reserved_supply15_v(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: VoltageRail15VMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for VoltageRail15VMeasurement {
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

impl embedded_can::Frame for VoltageRail15VMeasurement {
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
impl core::fmt::Debug for VoltageRail15VMeasurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("VoltageRail15VMeasurement")
                .field("supply15_v", &self.supply15_v())
                .field("reserved_supply15_v", &self.reserved_supply15_v())
                .finish()
        } else {
            f.debug_tuple("VoltageRail15VMeasurement")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for VoltageRail15VMeasurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VoltageRail15VMeasurement {{ Supply15V={:?} ReservedSupply15V={:?} }}",
            self.supply15_v(),
            self.reserved_supply15_v(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for VoltageRail15VMeasurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let supply15_v = u.int_in_range(0..=0)?;
        let reserved_supply15_v = u.int_in_range(0..=0)?;
        VoltageRail15VMeasurement::new(supply15_v, reserved_supply15_v)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// VoltageRail3V31V9Measurement
///
/// - Standard ID: 137 (0x89)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct VoltageRail3V31v9Measurement {
    raw: [u8; 8],
}

impl VoltageRail3V31v9Measurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x89) });

    pub const SUPPLY1_V9_MIN: i32 = 0_i32;
    pub const SUPPLY1_V9_MAX: i32 = 0_i32;
    pub const SUPPLY3_V3_MIN: i32 = 0_i32;
    pub const SUPPLY3_V3_MAX: i32 = 0_i32;

    /// Construct new VoltageRail3V31V9Measurement from values
    pub fn new(supply1_v9: i32, supply3_v3: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_supply1_v9(supply1_v9)?;
        res.set_supply3_v3(supply3_v3)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Supply1V9
    ///
    /// Actual voltage level of the 1.9V DSP power rail.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn supply1_v9(&self) -> i32 {
        self.supply1_v9_raw()
    }

    /// Get raw value of Supply1V9
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn supply1_v9_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Supply1V9
    #[inline(always)]
    pub fn set_supply1_v9(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: VoltageRail3V31v9Measurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// Supply3V3
    ///
    /// Actual voltage level of the 3.3V power rail.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn supply3_v3(&self) -> i32 {
        self.supply3_v3_raw()
    }

    /// Get raw value of Supply3V3
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn supply3_v3_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Supply3V3
    #[inline(always)]
    pub fn set_supply3_v3(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: VoltageRail3V31v9Measurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for VoltageRail3V31v9Measurement {
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

impl embedded_can::Frame for VoltageRail3V31v9Measurement {
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
impl core::fmt::Debug for VoltageRail3V31v9Measurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("VoltageRail3V31v9Measurement")
                .field("supply1_v9", &self.supply1_v9())
                .field("supply3_v3", &self.supply3_v3())
                .finish()
        } else {
            f.debug_tuple("VoltageRail3V31v9Measurement")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for VoltageRail3V31v9Measurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VoltageRail3V31v9Measurement {{ Supply1V9={:?} Supply3V3={:?} }}",
            self.supply1_v9(),
            self.supply3_v3(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for VoltageRail3V31v9Measurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let supply1_v9 = u.int_in_range(0..=0)?;
        let supply3_v3 = u.int_in_range(0..=0)?;
        VoltageRail3V31v9Measurement::new(supply1_v9, supply3_v3)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Reserved0A
///
/// - Standard ID: 138 (0x8a)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct Reserved0A {
    raw: [u8; 8],
}

impl Reserved0A {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x8a) });

    pub const RESERVED0_A0_MIN: i32 = 0_i32;
    pub const RESERVED0_A0_MAX: i32 = 0_i32;
    pub const RESERVED0_A1_MIN: i32 = 0_i32;
    pub const RESERVED0_A1_MAX: i32 = 0_i32;

    /// Construct new Reserved0A from values
    pub fn new(reserved0_a0: i32, reserved0_a1: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_reserved0_a0(reserved0_a0)?;
        res.set_reserved0_a1(reserved0_a1)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Reserved0A0
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved0_a0(&self) -> i32 {
        self.reserved0_a0_raw()
    }

    /// Get raw value of Reserved0A0
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn reserved0_a0_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Reserved0A0
    #[inline(always)]
    pub fn set_reserved0_a0(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Reserved0A::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// Reserved0A1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved0_a1(&self) -> i32 {
        self.reserved0_a1_raw()
    }

    /// Get raw value of Reserved0A1
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn reserved0_a1_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Reserved0A1
    #[inline(always)]
    pub fn set_reserved0_a1(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Reserved0A::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Reserved0A {
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

impl embedded_can::Frame for Reserved0A {
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
impl core::fmt::Debug for Reserved0A {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Reserved0A")
                .field("reserved0_a0", &self.reserved0_a0())
                .field("reserved0_a1", &self.reserved0_a1())
                .finish()
        } else {
            f.debug_tuple("Reserved0A").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Reserved0A {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reserved0A {{ Reserved0A0={:?} Reserved0A1={:?} }}",
            self.reserved0_a0(),
            self.reserved0_a1(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Reserved0A {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let reserved0_a0 = u.int_in_range(0..=0)?;
        let reserved0_a1 = u.int_in_range(0..=0)?;
        Reserved0A::new(reserved0_a0, reserved0_a1).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// HeatsinkMotorTempMeasurement
///
/// - Standard ID: 139 (0x8b)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct HeatsinkMotorTempMeasurement {
    raw: [u8; 8],
}

impl HeatsinkMotorTempMeasurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x8b) });

    pub const MOTOR_TEMP_MIN: i32 = 0_i32;
    pub const MOTOR_TEMP_MAX: i32 = 0_i32;
    pub const HEATSINK_TEMP_MIN: i32 = 0_i32;
    pub const HEATSINK_TEMP_MAX: i32 = 0_i32;

    /// Construct new HeatsinkMotorTempMeasurement from values
    pub fn new(motor_temp: i32, heatsink_temp: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_motor_temp(motor_temp)?;
        res.set_heatsink_temp(heatsink_temp)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// MotorTemp
    ///
    /// Internal temperature of the motor
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn motor_temp(&self) -> i32 {
        self.motor_temp_raw()
    }

    /// Get raw value of MotorTemp
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn motor_temp_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of MotorTemp
    #[inline(always)]
    pub fn set_motor_temp(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: HeatsinkMotorTempMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// HeatsinkTemp
    ///
    /// Internal temperature of Heat-sink (case).
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn heatsink_temp(&self) -> i32 {
        self.heatsink_temp_raw()
    }

    /// Get raw value of HeatsinkTemp
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn heatsink_temp_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of HeatsinkTemp
    #[inline(always)]
    pub fn set_heatsink_temp(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: HeatsinkMotorTempMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for HeatsinkMotorTempMeasurement {
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

impl embedded_can::Frame for HeatsinkMotorTempMeasurement {
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
impl core::fmt::Debug for HeatsinkMotorTempMeasurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("HeatsinkMotorTempMeasurement")
                .field("motor_temp", &self.motor_temp())
                .field("heatsink_temp", &self.heatsink_temp())
                .finish()
        } else {
            f.debug_tuple("HeatsinkMotorTempMeasurement")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for HeatsinkMotorTempMeasurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HeatsinkMotorTempMeasurement {{ MotorTemp={:?} HeatsinkTemp={:?} }}",
            self.motor_temp(),
            self.heatsink_temp(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for HeatsinkMotorTempMeasurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let motor_temp = u.int_in_range(0..=0)?;
        let heatsink_temp = u.int_in_range(0..=0)?;
        HeatsinkMotorTempMeasurement::new(motor_temp, heatsink_temp)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// DspBoardTempMeasurement
///
/// - Standard ID: 140 (0x8c)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct DspBoardTempMeasurement {
    raw: [u8; 8],
}

impl DspBoardTempMeasurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x8c) });

    pub const DSP_BOARD_TEMP_MIN: i32 = 0_i32;
    pub const DSP_BOARD_TEMP_MAX: i32 = 0_i32;
    pub const RESERVED_DSP_BOARD_TEMP_MIN: i32 = 0_i32;
    pub const RESERVED_DSP_BOARD_TEMP_MAX: i32 = 0_i32;

    /// Construct new DspBoardTempMeasurement from values
    pub fn new(dsp_board_temp: i32, reserved_dsp_board_temp: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_dsp_board_temp(dsp_board_temp)?;
        res.set_reserved_dsp_board_temp(reserved_dsp_board_temp)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// DspBoardTemp
    ///
    /// Temperature of the DSP board.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn dsp_board_temp(&self) -> i32 {
        self.dsp_board_temp_raw()
    }

    /// Get raw value of DspBoardTemp
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn dsp_board_temp_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of DspBoardTemp
    #[inline(always)]
    pub fn set_dsp_board_temp(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: DspBoardTempMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// ReservedDspBoardTemp
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved_dsp_board_temp(&self) -> i32 {
        self.reserved_dsp_board_temp_raw()
    }

    /// Get raw value of ReservedDspBoardTemp
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn reserved_dsp_board_temp_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of ReservedDspBoardTemp
    #[inline(always)]
    pub fn set_reserved_dsp_board_temp(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: DspBoardTempMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for DspBoardTempMeasurement {
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

impl embedded_can::Frame for DspBoardTempMeasurement {
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
impl core::fmt::Debug for DspBoardTempMeasurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("DspBoardTempMeasurement")
                .field("dsp_board_temp", &self.dsp_board_temp())
                .field("reserved_dsp_board_temp", &self.reserved_dsp_board_temp())
                .finish()
        } else {
            f.debug_tuple("DspBoardTempMeasurement")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for DspBoardTempMeasurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DspBoardTempMeasurement {{ DspBoardTemp={:?} ReservedDspBoardTemp={:?} }}",
            self.dsp_board_temp(),
            self.reserved_dsp_board_temp(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for DspBoardTempMeasurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let dsp_board_temp = u.int_in_range(0..=0)?;
        let reserved_dsp_board_temp = u.int_in_range(0..=0)?;
        DspBoardTempMeasurement::new(dsp_board_temp, reserved_dsp_board_temp)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Reserved0D
///
/// - Standard ID: 141 (0x8d)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct Reserved0D {
    raw: [u8; 8],
}

impl Reserved0D {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x8d) });

    pub const RESERVED0_D0_MIN: i32 = 0_i32;
    pub const RESERVED0_D0_MAX: i32 = 0_i32;
    pub const RESERVED0_D1_MIN: i32 = 0_i32;
    pub const RESERVED0_D1_MAX: i32 = 0_i32;

    /// Construct new Reserved0D from values
    pub fn new(reserved0_d0: i32, reserved0_d1: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_reserved0_d0(reserved0_d0)?;
        res.set_reserved0_d1(reserved0_d1)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Reserved0D0
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved0_d0(&self) -> i32 {
        self.reserved0_d0_raw()
    }

    /// Get raw value of Reserved0D0
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn reserved0_d0_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Reserved0D0
    #[inline(always)]
    pub fn set_reserved0_d0(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Reserved0D::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// Reserved0D1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved0_d1(&self) -> i32 {
        self.reserved0_d1_raw()
    }

    /// Get raw value of Reserved0D1
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn reserved0_d1_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Reserved0D1
    #[inline(always)]
    pub fn set_reserved0_d1(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Reserved0D::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Reserved0D {
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

impl embedded_can::Frame for Reserved0D {
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
impl core::fmt::Debug for Reserved0D {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Reserved0D")
                .field("reserved0_d0", &self.reserved0_d0())
                .field("reserved0_d1", &self.reserved0_d1())
                .finish()
        } else {
            f.debug_tuple("Reserved0D").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Reserved0D {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reserved0D {{ Reserved0D0={:?} Reserved0D1={:?} }}",
            self.reserved0_d0(),
            self.reserved0_d1(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Reserved0D {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let reserved0_d0 = u.int_in_range(0..=0)?;
        let reserved0_d1 = u.int_in_range(0..=0)?;
        Reserved0D::new(reserved0_d0, reserved0_d1).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// OdometerBusAhMeasurement
///
/// - Standard ID: 142 (0x8e)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct OdometerBusAhMeasurement {
    raw: [u8; 8],
}

impl OdometerBusAhMeasurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x8e) });

    pub const ODOMETER_MIN: i32 = 0_i32;
    pub const ODOMETER_MAX: i32 = 0_i32;
    pub const DC_BUS_AH_MIN: i32 = 0_i32;
    pub const DC_BUS_AH_MAX: i32 = 0_i32;

    /// Construct new OdometerBusAhMeasurement from values
    pub fn new(odometer: i32, dc_bus_ah: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_odometer(odometer)?;
        res.set_dc_bus_ah(dc_bus_ah)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Odometer
    ///
    /// Distance the vehicle has travelled since reset.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn odometer(&self) -> i32 {
        self.odometer_raw()
    }

    /// Get raw value of Odometer
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn odometer_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Odometer
    #[inline(always)]
    pub fn set_odometer(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: OdometerBusAhMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// DCBusAh
    ///
    /// Charge flow into the controller DC bus from the time of reset.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Ah"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn dc_bus_ah(&self) -> i32 {
        self.dc_bus_ah_raw()
    }

    /// Get raw value of DCBusAh
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn dc_bus_ah_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of DCBusAh
    #[inline(always)]
    pub fn set_dc_bus_ah(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: OdometerBusAhMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for OdometerBusAhMeasurement {
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

impl embedded_can::Frame for OdometerBusAhMeasurement {
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
impl core::fmt::Debug for OdometerBusAhMeasurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("OdometerBusAhMeasurement")
                .field("odometer", &self.odometer())
                .field("dc_bus_ah", &self.dc_bus_ah())
                .finish()
        } else {
            f.debug_tuple("OdometerBusAhMeasurement")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for OdometerBusAhMeasurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OdometerBusAhMeasurement {{ Odometer={:?} DCBusAh={:?} }}",
            self.odometer(),
            self.dc_bus_ah(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for OdometerBusAhMeasurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let odometer = u.int_in_range(0..=0)?;
        let dc_bus_ah = u.int_in_range(0..=0)?;
        OdometerBusAhMeasurement::new(odometer, dc_bus_ah)
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// SlipSpeedMeasurement
///
/// - Standard ID: 151 (0x97)
/// - Size: 8 bytes
/// - Transmitter: WaveSculptor22
#[derive(Clone, Copy)]
pub struct SlipSpeedMeasurement {
    raw: [u8; 8],
}

impl SlipSpeedMeasurement {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x97) });

    pub const SLIP_SPEED_MIN: i32 = 0_i32;
    pub const SLIP_SPEED_MAX: i32 = 0_i32;
    pub const RESERVED_SLIP_SPEED_MIN: i32 = 0_i32;
    pub const RESERVED_SLIP_SPEED_MAX: i32 = 0_i32;

    /// Construct new SlipSpeedMeasurement from values
    pub fn new(slip_speed: i32, reserved_slip_speed: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_slip_speed(slip_speed)?;
        res.set_reserved_slip_speed(reserved_slip_speed)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// SlipSpeed
    ///
    /// Slip speed when driving an induction motor.
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Hz"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn slip_speed(&self) -> i32 {
        self.slip_speed_raw()
    }

    /// Get raw value of SlipSpeed
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn slip_speed_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of SlipSpeed
    #[inline(always)]
    pub fn set_slip_speed(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: SlipSpeedMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }

    /// ReservedSlipSpeed
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved_slip_speed(&self) -> i32 {
        self.reserved_slip_speed_raw()
    }

    /// Get raw value of ReservedSlipSpeed
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn reserved_slip_speed_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();

        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of ReservedSlipSpeed
    #[inline(always)]
    pub fn set_reserved_slip_speed(&mut self, value: i32) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: SlipSpeedMeasurement::MESSAGE_ID,
        })?;
        let value = (value / factor) as i32;

        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for SlipSpeedMeasurement {
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

impl embedded_can::Frame for SlipSpeedMeasurement {
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
impl core::fmt::Debug for SlipSpeedMeasurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("SlipSpeedMeasurement")
                .field("slip_speed", &self.slip_speed())
                .field("reserved_slip_speed", &self.reserved_slip_speed())
                .finish()
        } else {
            f.debug_tuple("SlipSpeedMeasurement")
                .field(&self.raw)
                .finish()
        }
    }
}

impl defmt::Format for SlipSpeedMeasurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SlipSpeedMeasurement {{ SlipSpeed={:?} ReservedSlipSpeed={:?} }}",
            self.slip_speed(),
            self.reserved_slip_speed(),
        );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for SlipSpeedMeasurement {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let slip_speed = u.int_in_range(0..=0)?;
        let reserved_slip_speed = u.int_in_range(0..=0)?;
        SlipSpeedMeasurement::new(slip_speed, reserved_slip_speed)
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
