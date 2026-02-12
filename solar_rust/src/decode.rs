use embedded_can::Frame;
use socketcan::{CanFrame, CanId};
use crate::messages_kelly;





pub trait DecodeMessage{
    fn decodeMessage(&self) -> Vec<String>;
}


impl messages_kelly::Message1{

    pub fn decodeMessage(&self)->Vec<String>{
        let rpm: u16 = self.speed_rpm();
        let motor_current: f32 = self.motor_current();
        let battery_voltage: f32 = self.battery_voltage();

        let id_error: bool = self.id_error();
        let over_voltage_error: bool = self.over_voltage();
        let low_voltage_error: bool = self.low_voltage();
        let stall_error: bool = self.stall();
        let internal_volts_fault: bool = self.internal_volts_fault();
        let over_temp_error: bool = self.over_temperature();
        let throttle_error: bool = self.throttle_error();
        let internal_reset_error: bool = self.internal_reset();
        let hall_throttle_open_error: bool = self.hall_throttle_open();
        let angle_sensor_error: bool = self.angle_sensor_error();
        let motor_over_temp_error: bool = self.motor_over_temperature();
        let hall_galvanometer_error: bool = self.hall_galvanometer_error();
        vec![
            format!("RPM: {}", rpm),
            format!("Motor Current: {:.2} A", motor_current),
            format!("Battery Voltage: {:.2} V", battery_voltage),
            format!("ID Error: {}", id_error),
            format!("Over Voltage Error: {}", over_voltage_error),
            format!("Low Voltage Error: {}", low_voltage_error),
            format!("Stall Error: {}", stall_error),
            format!("Internal Volts Fault: {}", internal_volts_fault),
            format!("Over Temp Error: {}", over_temp_error),
            format!("Throttle Error: {}", throttle_error),
            format!("Internal Reset Error: {}", internal_reset_error),
            format!("Hall Throttle Open Error: {}", hall_throttle_open_error),
            format!("Angle Sensor Error: {}", angle_sensor_error),
            format!("Motor Over Temp Error: {}", motor_over_temp_error),
            format!("Hall Galvanometer Error: {}", hall_galvanometer_error),
        ]
    }

}

