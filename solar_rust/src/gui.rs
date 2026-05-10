// src/gui.rs
// Solar Car Telemetry Dashboard — iced 0.13

use iced::{
    widget::{column, container, row, text, Space},
    Alignment, Color, Element, Font, Length, Padding, Size, Task, Theme,
};

// ─────────────────────────────────────────────────────────────────────────────
//  Data model
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct Telemetry {
    // Message 1
    pub rpm: u16,
    pub motor_current: f32,
    pub battery_voltage: f32,
    pub id_error: bool,
    pub over_voltage: bool,
    pub low_voltage: bool,
    pub stall: bool,
    pub internal_volts_fault: bool,
    pub over_temperature: bool,
    pub throttle_error: bool,
    pub internal_reset: bool,
    pub hall_throttle_open: bool,
    pub angle_sensor_error: bool,
    pub motor_over_temperature: bool,
    pub hall_galvanometer_error: bool,

    // Message 2
    pub throttle_signal: f32,
    pub controller_temperature: i16,
    pub motor_temperature: i16,
    pub command_status: String,
    pub feedback_status: String,
    pub brake_switch: bool,
    pub backward_switch: bool,
    pub forward_switch: bool,
    pub foot_switch: bool,
    pub boost_switch: bool,
}

impl Telemetry {
    pub fn active_faults(&self) -> Vec<&'static str> {
        let mut f = Vec::new();
        if self.id_error                { f.push("ID ERR") }
        if self.over_voltage            { f.push("OVERVOLT") }
        if self.low_voltage             { f.push("LOWVOLT") }
        if self.stall                   { f.push("STALL") }
        if self.internal_volts_fault    { f.push("INT VOLT") }
        if self.over_temperature        { f.push("CTRL TEMP") }
        if self.throttle_error          { f.push("THROTTLE") }
        if self.internal_reset          { f.push("INT RESET") }
        if self.hall_throttle_open      { f.push("HALL OPEN") }
        if self.angle_sensor_error      { f.push("ANGLE SNS") }
        if self.motor_over_temperature  { f.push("MTR TEMP") }
        if self.hall_galvanometer_error { f.push("HALL GALV") }
        f
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Palette
// ─────────────────────────────────────────────────────────────────────────────

const BG:        Color = Color { r: 0.05, g: 0.06, b: 0.09, a: 1.0 };
const PANEL:     Color = Color { r: 0.08, g: 0.10, b: 0.14, a: 1.0 };
const SOLAR:     Color = Color { r: 1.00, g: 0.80, b: 0.00, a: 1.0 };
const TEAL:      Color = Color { r: 0.00, g: 0.88, b: 0.82, a: 1.0 };
const RED:       Color = Color { r: 1.00, g: 0.18, b: 0.22, a: 1.0 };
const GREEN:     Color = Color { r: 0.18, g: 0.95, b: 0.42, a: 1.0 };
const MUTED:     Color = Color { r: 0.45, g: 0.50, b: 0.58, a: 1.0 };

fn dim(c: Color) -> Color { Color { a: 0.40, ..c } }
fn border(c: Color) -> Color { Color { a: 0.25, ..c } }

// ─────────────────────────────────────────────────────────────────────────────
//  Application
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    TelemetryUpdated(Telemetry),
}

pub struct Dashboard {
    telemetry: Telemetry,
}

impl Dashboard {
    pub fn new() -> (Self, Task<Message>) {
        let demo = Telemetry {
            rpm: 3450,
            motor_current: 47.3,
            battery_voltage: 118.6,
            throttle_signal: 0.72,
            controller_temperature: 38,
            motor_temperature: 54,
            command_status: "RUN".into(),
            feedback_status: "OK".into(),
            forward_switch: true,
            foot_switch: true,
            ..Default::default()
        };
        (Self { telemetry: demo }, Task::none())
    }

    pub fn title(&self) -> String {
        "Solar Car Dashboard".into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TelemetryUpdated(t) => self.telemetry = t,
        }
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        let t = &self.telemetry;

        // ── Fault banner ─────────────────────────────────────────────────────
        let faults = t.active_faults();
        let (fault_text, fault_fg, fault_bg) = if faults.is_empty() {
            ("● ALL SYSTEMS NOMINAL".to_string(), GREEN, Color { r: 0.04, g: 0.18, b: 0.08, a: 1.0 })
        } else {
            (format!("⚠  FAULT: {}", faults.join("  ·  ")), Color::WHITE, RED)
        };

        // ── Primary metrics ───────────────────────────────────────────────────
        let throttle_pct = format!("{:.0}%", t.throttle_signal * 100.0);
        let primary = row![
            big_metric("RPM",     &t.rpm.to_string(),                      SOLAR),
            big_metric("VOLTAGE", &format!("{:.1} V", t.battery_voltage),  TEAL),
            big_metric("CURRENT", &format!("{:.1} A", t.motor_current),    SOLAR),
            big_metric("THROTTLE",&throttle_pct,                            TEAL),
        ]
        .spacing(10)
        .padding(Padding::from([0u16, 14]));

        // ── Thermal / status ──────────────────────────────────────────────────
        let ctrl_col = if t.controller_temperature > 70 { RED } else { TEAL };
        let mtr_col  = if t.motor_temperature > 80      { RED } else { TEAL };

        let thermals = row![
            med_metric("CTRL TEMP",  &format!("{}°C", t.controller_temperature), ctrl_col),
            med_metric("MTR TEMP",   &format!("{}°C", t.motor_temperature),      mtr_col),
            med_metric("CMD",        &t.command_status,                            SOLAR),
            med_metric("FEEDBACK",   &t.feedback_status,                           SOLAR),
        ]
        .spacing(10)
        .padding(Padding::from([0u16, 14]));

        // ── Switches ──────────────────────────────────────────────────────────
        let switches = row![
            switch_pill("BRAKE",   t.brake_switch,    RED),
            switch_pill("FWD",     t.forward_switch,  GREEN),
            switch_pill("REVERSE", t.backward_switch, SOLAR),
            switch_pill("FOOT SW", t.foot_switch,     TEAL),
            switch_pill("BOOST",   t.boost_switch,    SOLAR),
        ]
        .spacing(8)
        .padding(Padding::from([0u16, 14]));

        // ── Full layout ───────────────────────────────────────────────────────
        let content = column![
            // Header
            container(
                row![
                    text("☀  SOLAR CAR TELEMETRY")
                        .size(20)
                        .color(SOLAR)
                        .font(Font::MONOSPACE),
                    Space::with_width(Length::Fill),
                    text("DRIVER HUD")
                        .size(12)
                        .color(MUTED)
                        .font(Font::MONOSPACE),
                ]
                .align_y(Alignment::Center)
                .padding(Padding::from([10u16, 18]))
            )
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(PANEL)),
                ..Default::default()
            }),

            // Fault bar
            container(
                text(fault_text)
                    .size(14)
                    .color(fault_fg)
                    .font(Font::MONOSPACE)
            )
            .width(Length::Fill)
            .padding(Padding::from([7u16, 18]))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(fault_bg)),
                border: iced::Border {
                    color: fault_fg,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }),

            Space::with_height(8),
            section_label("POWERTRAIN"),
            primary,
            Space::with_height(8),
            section_label("THERMAL & STATUS"),
            thermals,
            Space::with_height(8),
            section_label("SWITCHES"),
            switches,
        ]
        .spacing(4)
        .width(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(BG)),
                ..Default::default()
            })
            .into()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Widget helpers
// ─────────────────────────────────────────────────────────────────────────────

fn section_label(label: &str) -> Element<Message> {
    container(
        text(label)
            .size(10)
            .color(MUTED)
            .font(Font::MONOSPACE),
    )
    .padding(Padding::from([0u16, 16]))
    .into()
}

fn big_metric<'a>(label: &'a str, value: &'a str, accent: Color) -> Element<'a, Message> {
    container(
        column![
            text(label).size(10).color(dim(accent)).font(Font::MONOSPACE),
            text(value).size(38).color(accent).font(Font::MONOSPACE),
        ]
        .spacing(2),
    )
    .padding(Padding::from([12u16, 16]))
    .width(Length::FillPortion(1))
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(PANEL)),
        border: iced::Border {
            color: border(accent),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .into()
}

fn med_metric<'a>(label: &'a str, value: &'a str, accent: Color) -> Element<'a, Message> {
    container(
        column![
            text(label).size(9).color(dim(accent)).font(Font::MONOSPACE),
            text(value).size(24).color(accent).font(Font::MONOSPACE),
        ]
        .spacing(2),
    )
    .padding(Padding::from([10u16, 14]))
    .width(Length::FillPortion(1))
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(PANEL)),
        border: iced::Border {
            color: border(accent),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .into()
}

fn switch_pill(label: &str, active: bool, accent: Color) -> Element<Message> {
    let (fg, bg_col, border_col) = if active {
        (
            accent,
            Color { r: accent.r * 0.12, g: accent.g * 0.12, b: accent.b * 0.12, a: 1.0 },
            accent,
        )
    } else {
        (MUTED, BG, Color { a: 0.20, ..MUTED })
    };

    let dot = if active { "● " } else { "○ " };
    container(
        text(format!("{}{}", dot, label))
            .size(12)
            .color(fg)
            .font(Font::MONOSPACE),
    )
    .padding(Padding::from([7u16, 14]))
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(bg_col)),
        border: iced::Border {
            color: border_col,
            width: 1.0,
            radius: 20.0.into(),
        },
        ..Default::default()
    })
    .into()
}

// ─────────────────────────────────────────────────────────────────────────────
//  Entry point
// ─────────────────────────────────────────────────────────────────────────────

pub fn main() -> iced::Result {
    iced::application("Solar Car Dashboard", Dashboard::update, Dashboard::view)
        .window(iced::window::Settings {
            size: Size::new(800.0, 480.0),
            ..Default::default()
        })
        .theme(|_| Theme::Dark)
        .run_with(Dashboard::new)
}