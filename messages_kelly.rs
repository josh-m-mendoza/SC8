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

//! Message definitions from file `"kelly_dbc.dbc"`
//!
//! - Version: `Version("HIPBNYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY/4/%%%/4/'%**4YYY///")`

#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};
use bitvec::prelude::*;
use core::ops::BitOr;
use embedded_can::{ExtendedId, Id, StandardId};

/// All messages
#[derive(Clone, Debug, defmt::Format)]
pub enum Messages {
    /// Message1
    Message1(Message1),
    /// Message2
    Message2(Message2),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        let res = match id {
            Message1::MESSAGE_ID => Messages::Message1(Message1::try_from(payload)?),
            Message2::MESSAGE_ID => Messages::Message2(Message2::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Message1
///
/// - Standard ID: 7685 (0x1e05)
/// - Size: 8 bytes
/// - Transmitter: SinusodialWaveControllerKLS
///
/// Message 1 broadcast (OUT IN ID 0x0CF11E05), period 50 ms.
#[derive(Clone, Copy)]
pub struct Message1 {
    raw: [u8; 8],
}

impl Message1 {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x1e05) });

    pub const SPEED_RPM_MIN: u16 = 0_u16;
    pub const SPEED_RPM_MAX: u16 = 6000_u16;
    pub const MOTOR_CURRENT_MIN: f32 = 0_f32;
    pub const MOTOR_CURRENT_MAX: f32 = 400_f32;
    pub const BATTERY_VOLTAGE_MIN: f32 = 0_f32;
    pub const BATTERY_VOLTAGE_MAX: f32 = 180_f32;

    /// Construct new Message1 from values
    pub fn new(
        speed_rpm: u16,
        motor_current: f32,
        battery_voltage: f32,
        id_error: bool,
        over_voltage: bool,
        low_voltage: bool,
        reserved51: bool,
        stall: bool,
        internal_volts_fault: bool,
        over_temperature: bool,
        throttle_error: bool,
        reserved56: bool,
        internal_reset: bool,
        hall_throttle_open: bool,
        angle_sensor_error: bool,
        reserved60: bool,
        reserved61: bool,
        motor_over_temperature: bool,
        hall_galvanometer_error: bool,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_speed_rpm(speed_rpm)?;
        res.set_motor_current(motor_current)?;
        res.set_battery_voltage(battery_voltage)?;
        res.set_id_error(id_error)?;
        res.set_over_voltage(over_voltage)?;
        res.set_low_voltage(low_voltage)?;
        res.set_reserved51(reserved51)?;
        res.set_stall(stall)?;
        res.set_internal_volts_fault(internal_volts_fault)?;
        res.set_over_temperature(over_temperature)?;
        res.set_throttle_error(throttle_error)?;
        res.set_reserved56(reserved56)?;
        res.set_internal_reset(internal_reset)?;
        res.set_hall_throttle_open(hall_throttle_open)?;
        res.set_angle_sensor_error(angle_sensor_error)?;
        res.set_reserved60(reserved60)?;
        res.set_reserved61(reserved61)?;
        res.set_motor_over_temperature(motor_over_temperature)?;
        res.set_hall_galvanometer_error(hall_galvanometer_error)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// SpeedRPM
    ///
    /// Actual speed (RPM) = (MSB*256 + LSB), 1 rpm/bit; range 0–6000 rpm.
    ///
    /// - Min: 0
    /// - Max: 6000
    /// - Unit: "rpm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn speed_rpm(&self) -> u16 {
        self.speed_rpm_raw()
    }

    /// Get raw value of SpeedRPM
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn speed_rpm_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();

        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of SpeedRPM
    #[inline(always)]
    pub fn set_speed_rpm(&mut self, value: u16) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Message1::MESSAGE_ID,
        })?;
        let value = (value / factor) as u16;

        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }

    /// MotorCurrent
    ///
    /// Actual current = (MSB*256 + LSB) / 10, 0.1 A/bit; raw range 0–4000 maps to 0–400 A.
    ///
    /// - Min: 0
    /// - Max: 400
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn motor_current(&self) -> f32 {
        self.motor_current_raw()
    }

    /// Get raw value of MotorCurrent
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn motor_current_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();

        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of MotorCurrent
    #[inline(always)]
    pub fn set_motor_current(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;

        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }

    /// BatteryVoltage
    ///
    /// Actual voltage = (MSB*256 + LSB) / 10, 0.1 V/bit; raw range 0–1800 maps to 0–180 V.
    ///
    /// - Min: 0
    /// - Max: 180
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn battery_voltage(&self) -> f32 {
        self.battery_voltage_raw()
    }

    /// Get raw value of BatteryVoltage
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn battery_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();

        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of BatteryVoltage
    #[inline(always)]
    pub fn set_battery_voltage(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;

        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }

    /// IDError
    ///
    /// ERR0 Identification error: Identification Angle operation failed; retry identification per Kelly instructions.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn id_error(&self) -> bool {
        self.id_error_raw()
    }

    /// Get raw value of IDError
    ///
    /// - Start bit: 48
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn id_error_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[48..49].load_le::<u8>();

        signal == 1
    }

    /// Set value of IDError
    #[inline(always)]
    pub fn set_id_error(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[48..49].store_le(value);
        Ok(())
    }

    /// OverVoltage
    ///
    /// ERR1 Over voltage: battery voltage too high; check battery volts and configuration.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn over_voltage(&self) -> bool {
        self.over_voltage_raw()
    }

    /// Get raw value of OverVoltage
    ///
    /// - Start bit: 49
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn over_voltage_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[49..50].load_le::<u8>();

        signal == 1
    }

    /// Set value of OverVoltage
    #[inline(always)]
    pub fn set_over_voltage(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[49..50].store_le(value);
        Ok(())
    }

    /// LowVoltage
    ///
    /// ERR2 Low voltage: battery voltage too low / under-voltage setting; check/charge battery.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn low_voltage(&self) -> bool {
        self.low_voltage_raw()
    }

    /// Get raw value of LowVoltage
    ///
    /// - Start bit: 50
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn low_voltage_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[50..51].load_le::<u8>();

        signal == 1
    }

    /// Set value of LowVoltage
    #[inline(always)]
    pub fn set_low_voltage(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[50..51].store_le(value);
        Ok(())
    }

    /// Reserved51
    ///
    /// ERR3 reserved.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved51(&self) -> bool {
        self.reserved51_raw()
    }

    /// Get raw value of Reserved51
    ///
    /// - Start bit: 51
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn reserved51_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[51..52].load_le::<u8>();

        signal == 1
    }

    /// Set value of Reserved51
    #[inline(always)]
    pub fn set_reserved51(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[51..52].store_le(value);
        Ok(())
    }

    /// Stall
    ///
    /// ERR4 stall: no speed feedback after controller outputs command for 2 seconds; may relate to speed sensors or phase wires.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn stall(&self) -> bool {
        self.stall_raw()
    }

    /// Get raw value of Stall
    ///
    /// - Start bit: 52
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn stall_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[52..53].load_le::<u8>();

        signal == 1
    }

    /// Set value of Stall
    #[inline(always)]
    pub fn set_stall(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[52..53].store_le(value);
        Ok(())
    }

    /// InternalVoltsFault
    ///
    /// ERR5 Internal volts fault: check B+/PWR wiring; possible excessive load on +5V supply; controller may be damaged.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn internal_volts_fault(&self) -> bool {
        self.internal_volts_fault_raw()
    }

    /// Get raw value of InternalVoltsFault
    ///
    /// - Start bit: 53
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn internal_volts_fault_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[53..54].load_le::<u8>();

        signal == 1
    }

    /// Set value of InternalVoltsFault
    #[inline(always)]
    pub fn set_internal_volts_fault(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[53..54].store_le(value);
        Ok(())
    }

    /// OverTemperature
    ///
    /// ERR6 Over temperature: controller temp exceeded 100°C; stops, restarts below 80°C.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn over_temperature(&self) -> bool {
        self.over_temperature_raw()
    }

    /// Get raw value of OverTemperature
    ///
    /// - Start bit: 54
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn over_temperature_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[54..55].load_le::<u8>();

        signal == 1
    }

    /// Set value of OverTemperature
    #[inline(always)]
    pub fn set_over_temperature(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[54..55].store_le(value);
        Ok(())
    }

    /// ThrottleError
    ///
    /// ERR7 Throttle error at power-up: valid throttle above TPS Low / dead zone at power-on; fault clears when throttle released; set correct pedal type if hall pedal.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn throttle_error(&self) -> bool {
        self.throttle_error_raw()
    }

    /// Get raw value of ThrottleError
    ///
    /// - Start bit: 55
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn throttle_error_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[55..56].load_le::<u8>();

        signal == 1
    }

    /// Set value of ThrottleError
    #[inline(always)]
    pub fn set_throttle_error(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[55..56].store_le(value);
        Ok(())
    }

    /// Reserved56
    ///
    /// ERR8 Reserved.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved56(&self) -> bool {
        self.reserved56_raw()
    }

    /// Get raw value of Reserved56
    ///
    /// - Start bit: 56
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn reserved56_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[56..57].load_le::<u8>();

        signal == 1
    }

    /// Set value of Reserved56
    #[inline(always)]
    pub fn set_reserved56(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[56..57].store_le(value);
        Ok(())
    }

    /// InternalReset
    ///
    /// ERR9 Internal reset: may be transient (temporary over-current or momentary high/low battery voltage).
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn internal_reset(&self) -> bool {
        self.internal_reset_raw()
    }

    /// Get raw value of InternalReset
    ///
    /// - Start bit: 57
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn internal_reset_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[57..58].load_le::<u8>();

        signal == 1
    }

    /// Set value of InternalReset
    #[inline(always)]
    pub fn set_internal_reset(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[57..58].store_le(value);
        Ok(())
    }

    /// HallThrottleOpen
    ///
    /// ERR10 Hall throttle open/short-circuit: check throttle pedal wiring; restart clears after repair.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hall_throttle_open(&self) -> bool {
        self.hall_throttle_open_raw()
    }

    /// Get raw value of HallThrottleOpen
    ///
    /// - Start bit: 58
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hall_throttle_open_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[58..59].load_le::<u8>();

        signal == 1
    }

    /// Set value of HallThrottleOpen
    #[inline(always)]
    pub fn set_hall_throttle_open(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[58..59].store_le(value);
        Ok(())
    }

    /// AngleSensorError
    ///
    /// ERR11 Angle sensor error: speed sensor type error / incorrect wiring / sensor damaged or erratic feedback.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn angle_sensor_error(&self) -> bool {
        self.angle_sensor_error_raw()
    }

    /// Get raw value of AngleSensorError
    ///
    /// - Start bit: 59
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn angle_sensor_error_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[59..60].load_le::<u8>();

        signal == 1
    }

    /// Set value of AngleSensorError
    #[inline(always)]
    pub fn set_angle_sensor_error(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[59..60].store_le(value);
        Ok(())
    }

    /// Reserved60
    ///
    /// ERR12 Reserved.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved60(&self) -> bool {
        self.reserved60_raw()
    }

    /// Get raw value of Reserved60
    ///
    /// - Start bit: 60
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn reserved60_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[60..61].load_le::<u8>();

        signal == 1
    }

    /// Set value of Reserved60
    #[inline(always)]
    pub fn set_reserved60(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[60..61].store_le(value);
        Ok(())
    }

    /// Reserved61
    ///
    /// ERR13 Reserved.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reserved61(&self) -> bool {
        self.reserved61_raw()
    }

    /// Get raw value of Reserved61
    ///
    /// - Start bit: 61
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn reserved61_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[61..62].load_le::<u8>();

        signal == 1
    }

    /// Set value of Reserved61
    #[inline(always)]
    pub fn set_reserved61(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[61..62].store_le(value);
        Ok(())
    }

    /// MotorOverTemperature
    ///
    /// ERR14 Motor over-temperature: exceeds configured maximum; controller shuts down until motor cools; max temp configurable.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn motor_over_temperature(&self) -> bool {
        self.motor_over_temperature_raw()
    }

    /// Get raw value of MotorOverTemperature
    ///
    /// - Start bit: 62
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn motor_over_temperature_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[62..63].load_le::<u8>();

        signal == 1
    }

    /// Set value of MotorOverTemperature
    #[inline(always)]
    pub fn set_motor_over_temperature(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[62..63].store_le(value);
        Ok(())
    }

    /// HallGalvanometerError
    ///
    /// ERR15 Hall galvanometer sensor error: hall galvanometer device damaged; only valid for KLS-8080I.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hall_galvanometer_error(&self) -> bool {
        self.hall_galvanometer_error_raw()
    }

    /// Get raw value of HallGalvanometerError
    ///
    /// - Start bit: 63
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hall_galvanometer_error_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[63..64].load_le::<u8>();

        signal == 1
    }

    /// Set value of HallGalvanometerError
    #[inline(always)]
    pub fn set_hall_galvanometer_error(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[63..64].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Message1 {
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

impl embedded_can::Frame for Message1 {
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
impl core::fmt::Debug for Message1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Message1")
                .field("speed_rpm", &self.speed_rpm())
                .field("motor_current", &self.motor_current())
                .field("battery_voltage", &self.battery_voltage())
                .field("id_error", &self.id_error())
                .field("over_voltage", &self.over_voltage())
                .field("low_voltage", &self.low_voltage())
                .field("reserved51", &self.reserved51())
                .field("stall", &self.stall())
                .field("internal_volts_fault", &self.internal_volts_fault())
                .field("over_temperature", &self.over_temperature())
                .field("throttle_error", &self.throttle_error())
                .field("reserved56", &self.reserved56())
                .field("internal_reset", &self.internal_reset())
                .field("hall_throttle_open", &self.hall_throttle_open())
                .field("angle_sensor_error", &self.angle_sensor_error())
                .field("reserved60", &self.reserved60())
                .field("reserved61", &self.reserved61())
                .field("motor_over_temperature", &self.motor_over_temperature())
                .field("hall_galvanometer_error", &self.hall_galvanometer_error())
                .finish()
        } else {
            f.debug_tuple("Message1").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Message1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Message1 {{ SpeedRPM={:?} MotorCurrent={:?} BatteryVoltage={:?} IDError={:?} OverVoltage={:?} LowVoltage={:?} Reserved51={:?} Stall={:?} InternalVoltsFault={:?} OverTemperature={:?} ThrottleError={:?} Reserved56={:?} InternalReset={:?} HallThrottleOpen={:?} AngleSensorError={:?} Reserved60={:?} Reserved61={:?} MotorOverTemperature={:?} HallGalvanometerError={:?} }}",
            self.speed_rpm(),
            self.motor_current(),
            self.battery_voltage(),
            self.id_error(),
            self.over_voltage(),
            self.low_voltage(),
            self.reserved51(),
            self.stall(),
            self.internal_volts_fault(),
            self.over_temperature(),
            self.throttle_error(),
            self.reserved56(),
            self.internal_reset(),
            self.hall_throttle_open(),
            self.angle_sensor_error(),
            self.reserved60(),
            self.reserved61(),
            self.motor_over_temperature(),
            self.hall_galvanometer_error(),
            );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Message1 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let speed_rpm = u.int_in_range(0..=6000)?;
        let motor_current = u.float_in_range(0_f32..=400_f32)?;
        let battery_voltage = u.float_in_range(0_f32..=180_f32)?;
        let id_error = u.int_in_range(0..=1)? == 1;
        let over_voltage = u.int_in_range(0..=1)? == 1;
        let low_voltage = u.int_in_range(0..=1)? == 1;
        let reserved51 = u.int_in_range(0..=1)? == 1;
        let stall = u.int_in_range(0..=1)? == 1;
        let internal_volts_fault = u.int_in_range(0..=1)? == 1;
        let over_temperature = u.int_in_range(0..=1)? == 1;
        let throttle_error = u.int_in_range(0..=1)? == 1;
        let reserved56 = u.int_in_range(0..=1)? == 1;
        let internal_reset = u.int_in_range(0..=1)? == 1;
        let hall_throttle_open = u.int_in_range(0..=1)? == 1;
        let angle_sensor_error = u.int_in_range(0..=1)? == 1;
        let reserved60 = u.int_in_range(0..=1)? == 1;
        let reserved61 = u.int_in_range(0..=1)? == 1;
        let motor_over_temperature = u.int_in_range(0..=1)? == 1;
        let hall_galvanometer_error = u.int_in_range(0..=1)? == 1;
        Message1::new(
            speed_rpm,
            motor_current,
            battery_voltage,
            id_error,
            over_voltage,
            low_voltage,
            reserved51,
            stall,
            internal_volts_fault,
            over_temperature,
            throttle_error,
            reserved56,
            internal_reset,
            hall_throttle_open,
            angle_sensor_error,
            reserved60,
            reserved61,
            motor_over_temperature,
            hall_galvanometer_error,
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Message2
///
/// - Standard ID: 7941 (0x1f05)
/// - Size: 8 bytes
/// - Transmitter: SinusodialWaveControllerKLS
///
/// Message 2 broadcast (OUT IN ID 0x0CF11F05), period 50 ms.
#[derive(Clone, Copy)]
pub struct Message2 {
    raw: [u8; 8],
}

impl Message2 {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x1f05) });

    pub const THROTTLE_SIGNAL_MIN: f32 = 0_f32;
    pub const THROTTLE_SIGNAL_MAX: f32 = 5_f32;
    pub const CONTROLLER_TEMPERATURE_MIN: i16 = -40_i16;
    pub const CONTROLLER_TEMPERATURE_MAX: i16 = 215_i16;
    pub const MOTOR_TEMPERATURE_MIN: i16 = -30_i16;
    pub const MOTOR_TEMPERATURE_MAX: i16 = 225_i16;
    pub const COMMAND_STATUS_MIN: u8 = 0_u8;
    pub const COMMAND_STATUS_MAX: u8 = 3_u8;
    pub const FEEDBACK_STATUS_MIN: u8 = 0_u8;
    pub const FEEDBACK_STATUS_MAX: u8 = 3_u8;

    /// Construct new Message2 from values
    pub fn new(
        throttle_signal: f32,
        controller_temperature: i16,
        motor_temperature: i16,
        command_status: u8,
        feedback_status: u8,
        hall_a: bool,
        hall_b: bool,
        hall_c: bool,
        brake_switch: bool,
        backward_switch: bool,
        forward_switch: bool,
        foot_switch: bool,
        boost_switch: bool,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_throttle_signal(throttle_signal)?;
        res.set_controller_temperature(controller_temperature)?;
        res.set_motor_temperature(motor_temperature)?;
        res.set_command_status(command_status)?;
        res.set_feedback_status(feedback_status)?;
        res.set_hall_a(hall_a)?;
        res.set_hall_b(hall_b)?;
        res.set_hall_c(hall_c)?;
        res.set_brake_switch(brake_switch)?;
        res.set_backward_switch(backward_switch)?;
        res.set_forward_switch(forward_switch)?;
        res.set_foot_switch(foot_switch)?;
        res.set_boost_switch(boost_switch)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// ThrottleSignal
    ///
    /// Throttle signal: raw 0–255 maps to 0–5 V.
    ///
    /// - Min: 0
    /// - Max: 5
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn throttle_signal(&self) -> f32 {
        self.throttle_signal_raw()
    }

    /// Get raw value of ThrottleSignal
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 0.019607843
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn throttle_signal_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();

        let factor = 0.019607843_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of ThrottleSignal
    #[inline(always)]
    pub fn set_throttle_signal(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.019607843_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }

    /// ControllerTemperature
    ///
    /// Controller temperature: offset 40; actual = raw - 40, 1°C/bit.
    ///
    /// - Min: -40
    /// - Max: 215
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn controller_temperature(&self) -> i16 {
        self.controller_temperature_raw()
    }

    /// Get raw value of ControllerTemperature
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: -40
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn controller_temperature_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();

        let factor = 1;
        i16::from(signal).saturating_mul(factor).saturating_sub(40)
    }

    /// Set value of ControllerTemperature
    #[inline(always)]
    pub fn set_controller_temperature(&mut self, value: i16) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_add(40).ok_or(CanError::ParameterOutOfRange {
            message_id: Message2::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }

    /// MotorTemperature
    ///
    /// Motor temperature: offset 30; actual = raw - 30, 1°C/bit.
    ///
    /// - Min: -30
    /// - Max: 225
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn motor_temperature(&self) -> i16 {
        self.motor_temperature_raw()
    }

    /// Get raw value of MotorTemperature
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: -30
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn motor_temperature_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();

        let factor = 1;
        i16::from(signal).saturating_mul(factor).saturating_sub(30)
    }

    /// Set value of MotorTemperature
    #[inline(always)]
    pub fn set_motor_temperature(&mut self, value: i16) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_add(30).ok_or(CanError::ParameterOutOfRange {
            message_id: Message2::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }

    /// CommandStatus
    ///
    /// Status of command: 0=Neutral, 1=forward, 2=backward, 3=reserved.
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn command_status(&self) -> Message2CommandStatus {
        let signal = self.raw.view_bits::<Lsb0>()[32..34].load_le::<u8>();

        match signal {
            0 => Message2CommandStatus::Neutral,
            1 => Message2CommandStatus::Forward,
            2 => Message2CommandStatus::Backward,
            3 => Message2CommandStatus::Reserved,
            _ => Message2CommandStatus::_Other(self.command_status_raw()),
        }
    }

    /// Get raw value of CommandStatus
    ///
    /// - Start bit: 32
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn command_status_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[32..34].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of CommandStatus
    #[inline(always)]
    pub fn set_command_status(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Message2::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[32..34].store_le(value);
        Ok(())
    }

    /// FeedbackStatus
    ///
    /// Status of feedback: 0=stationary, 1=forward, 2=backward, 3=reserved.
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn feedback_status(&self) -> Message2FeedbackStatus {
        let signal = self.raw.view_bits::<Lsb0>()[34..36].load_le::<u8>();

        match signal {
            0 => Message2FeedbackStatus::Stationary,
            1 => Message2FeedbackStatus::Forward,
            2 => Message2FeedbackStatus::Backward,
            3 => Message2FeedbackStatus::Reserved,
            _ => Message2FeedbackStatus::_Other(self.feedback_status_raw()),
        }
    }

    /// Get raw value of FeedbackStatus
    ///
    /// - Start bit: 34
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn feedback_status_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[34..36].load_le::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of FeedbackStatus
    #[inline(always)]
    pub fn set_feedback_status(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Message2::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Lsb0>()[34..36].store_le(value);
        Ok(())
    }

    /// HallA
    ///
    /// Switch signals: Hall A.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hall_a(&self) -> bool {
        self.hall_a_raw()
    }

    /// Get raw value of HallA
    ///
    /// - Start bit: 40
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hall_a_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[40..41].load_le::<u8>();

        signal == 1
    }

    /// Set value of HallA
    #[inline(always)]
    pub fn set_hall_a(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[40..41].store_le(value);
        Ok(())
    }

    /// HallB
    ///
    /// Switch signals: Hall B.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hall_b(&self) -> bool {
        self.hall_b_raw()
    }

    /// Get raw value of HallB
    ///
    /// - Start bit: 41
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hall_b_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[41..42].load_le::<u8>();

        signal == 1
    }

    /// Set value of HallB
    #[inline(always)]
    pub fn set_hall_b(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[41..42].store_le(value);
        Ok(())
    }

    /// HallC
    ///
    /// Switch signals: Hall C.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hall_c(&self) -> bool {
        self.hall_c_raw()
    }

    /// Get raw value of HallC
    ///
    /// - Start bit: 42
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hall_c_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[42..43].load_le::<u8>();

        signal == 1
    }

    /// Set value of HallC
    #[inline(always)]
    pub fn set_hall_c(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[42..43].store_le(value);
        Ok(())
    }

    /// BrakeSwitch
    ///
    /// Switch signals: 12V brake switch.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_switch(&self) -> bool {
        self.brake_switch_raw()
    }

    /// Get raw value of BrakeSwitch
    ///
    /// - Start bit: 43
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_switch_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[43..44].load_le::<u8>();

        signal == 1
    }

    /// Set value of BrakeSwitch
    #[inline(always)]
    pub fn set_brake_switch(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[43..44].store_le(value);
        Ok(())
    }

    /// BackwardSwitch
    ///
    /// Switch signals: backward switch.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn backward_switch(&self) -> bool {
        self.backward_switch_raw()
    }

    /// Get raw value of BackwardSwitch
    ///
    /// - Start bit: 44
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn backward_switch_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[44..45].load_le::<u8>();

        signal == 1
    }

    /// Set value of BackwardSwitch
    #[inline(always)]
    pub fn set_backward_switch(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[44..45].store_le(value);
        Ok(())
    }

    /// ForwardSwitch
    ///
    /// Switch signals: forward switch.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn forward_switch(&self) -> bool {
        self.forward_switch_raw()
    }

    /// Get raw value of ForwardSwitch
    ///
    /// - Start bit: 45
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn forward_switch_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[45..46].load_le::<u8>();

        signal == 1
    }

    /// Set value of ForwardSwitch
    #[inline(always)]
    pub fn set_forward_switch(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[45..46].store_le(value);
        Ok(())
    }

    /// FootSwitch
    ///
    /// Switch signals: foot switch.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn foot_switch(&self) -> bool {
        self.foot_switch_raw()
    }

    /// Get raw value of FootSwitch
    ///
    /// - Start bit: 46
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn foot_switch_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[46..47].load_le::<u8>();

        signal == 1
    }

    /// Set value of FootSwitch
    #[inline(always)]
    pub fn set_foot_switch(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[46..47].store_le(value);
        Ok(())
    }

    /// BoostSwitch
    ///
    /// Switch signals: boost switch.
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn boost_switch(&self) -> bool {
        self.boost_switch_raw()
    }

    /// Get raw value of BoostSwitch
    ///
    /// - Start bit: 47
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn boost_switch_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[47..48].load_le::<u8>();

        signal == 1
    }

    /// Set value of BoostSwitch
    #[inline(always)]
    pub fn set_boost_switch(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[47..48].store_le(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Message2 {
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

impl embedded_can::Frame for Message2 {
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
impl core::fmt::Debug for Message2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Message2")
                .field("throttle_signal", &self.throttle_signal())
                .field("controller_temperature", &self.controller_temperature())
                .field("motor_temperature", &self.motor_temperature())
                .field("command_status", &self.command_status())
                .field("feedback_status", &self.feedback_status())
                .field("hall_a", &self.hall_a())
                .field("hall_b", &self.hall_b())
                .field("hall_c", &self.hall_c())
                .field("brake_switch", &self.brake_switch())
                .field("backward_switch", &self.backward_switch())
                .field("forward_switch", &self.forward_switch())
                .field("foot_switch", &self.foot_switch())
                .field("boost_switch", &self.boost_switch())
                .finish()
        } else {
            f.debug_tuple("Message2").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Message2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Message2 {{ ThrottleSignal={:?} ControllerTemperature={:?} MotorTemperature={:?} CommandStatus={:?} FeedbackStatus={:?} HallA={:?} HallB={:?} HallC={:?} BrakeSwitch={:?} BackwardSwitch={:?} ForwardSwitch={:?} FootSwitch={:?} BoostSwitch={:?} }}",
            self.throttle_signal(),
            self.controller_temperature(),
            self.motor_temperature(),
            self.command_status(),
            self.feedback_status(),
            self.hall_a(),
            self.hall_b(),
            self.hall_c(),
            self.brake_switch(),
            self.backward_switch(),
            self.forward_switch(),
            self.foot_switch(),
            self.boost_switch(),
            );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Message2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let throttle_signal = u.float_in_range(0_f32..=5_f32)?;
        let controller_temperature = u.int_in_range(-40..=215)?;
        let motor_temperature = u.int_in_range(-30..=225)?;
        let command_status = u.int_in_range(0..=3)?;
        let feedback_status = u.int_in_range(0..=3)?;
        let hall_a = u.int_in_range(0..=1)? == 1;
        let hall_b = u.int_in_range(0..=1)? == 1;
        let hall_c = u.int_in_range(0..=1)? == 1;
        let brake_switch = u.int_in_range(0..=1)? == 1;
        let backward_switch = u.int_in_range(0..=1)? == 1;
        let forward_switch = u.int_in_range(0..=1)? == 1;
        let foot_switch = u.int_in_range(0..=1)? == 1;
        let boost_switch = u.int_in_range(0..=1)? == 1;
        Message2::new(
            throttle_signal,
            controller_temperature,
            motor_temperature,
            command_status,
            feedback_status,
            hall_a,
            hall_b,
            hall_c,
            brake_switch,
            backward_switch,
            forward_switch,
            foot_switch,
            boost_switch,
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for CommandStatus
#[derive(Clone, Copy, PartialEq, Debug, defmt::Format)]
pub enum Message2CommandStatus {
    Neutral,
    Forward,
    Backward,
    Reserved,
    _Other(u8),
}

impl From<Message2CommandStatus> for u8 {
    fn from(val: Message2CommandStatus) -> u8 {
        match val {
            Message2CommandStatus::Neutral => 0,
            Message2CommandStatus::Forward => 1,
            Message2CommandStatus::Backward => 2,
            Message2CommandStatus::Reserved => 3,
            Message2CommandStatus::_Other(x) => x,
        }
    }
}

/// Defined values for FeedbackStatus
#[derive(Clone, Copy, PartialEq, Debug, defmt::Format)]
pub enum Message2FeedbackStatus {
    Stationary,
    Forward,
    Backward,
    Reserved,
    _Other(u8),
}

impl From<Message2FeedbackStatus> for u8 {
    fn from(val: Message2FeedbackStatus) -> u8 {
        match val {
            Message2FeedbackStatus::Stationary => 0,
            Message2FeedbackStatus::Forward => 1,
            Message2FeedbackStatus::Backward => 2,
            Message2FeedbackStatus::Reserved => 3,
            Message2FeedbackStatus::_Other(x) => x,
        }
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
