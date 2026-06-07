// src/dashboard.rs  — solar car telemetry dashboard
// Reads from Redis TimeSeries; runs as a separate binary alongside solar_rust.
//
// Run with: cargo run --bin dashboard
// The CAN reader (cargo run --bin solar_rust) must be running and writing to Redis.

use iced::widget::{canvas, column, container, row, text};
use iced::{Color, Element, Font, Length, Size, Subscription, Task, Theme};
use iced::widget::canvas::{Cache, Frame, Geometry, Path, Stroke};
use iced::time;
use std::time::Duration;

// ─── Redis fetch ──────────────────────────────────────────────────────────────
//
// Design notes:
//   - iced manages its own async executor; Task::perform() is the bridge.
//   - We do NOT use #[tokio::main] here; iced::application().run() owns the runtime.
//   - fetch_telemetry is a plain async fn returning anyhow::Result<Telemetry>.
//     Task::perform maps the error to String so it fits in Message::TelemetryLoaded.
//   - If Redis is unavailable or a key hasn't been written yet, we return Err
//     and the update() arm keeps showing the last good snapshot.
//   - ts_get_f64 is a helper that returns f64 on success and 0.0 on any Redis
//     error (missing key, type mismatch, etc.).  This prevents a single missing
//     key from aborting the entire fetch during startup.

async fn ts_get_f64(
    con: &mut redis::aio::MultiplexedConnection,
    key: &str,
) -> f64 {
    // TS.GET returns (timestamp_ms: i64, value: f64) as a two-element bulk reply.
    // If the key doesn't exist yet the command returns a Redis error — we treat
    // that as 0.0 so the dashboard stays alive during startup.
    let result: redis::RedisResult<(i64, f64)> = redis::cmd("TS.GET")
        .arg(key)
        .query_async(con)
        .await;
    match result {
        Ok((_ts, v)) => v,
        Err(_) => 0.0,
    }
}

async fn fetch_telemetry() -> anyhow::Result<Telemetry> {
    // Open a fresh multiplexed connection each tick.
    // For production you'd pass a connection pool/manager, but at 100 ms polling
    // the overhead is acceptable and keeps the code simple.
    let client = redis::Client::open("redis://127.0.0.1")?;
    let mut con = client.get_multiplexed_async_connection().await?;

    // ── Kelly keys ────────────────────────────────────────────────────────────
    let battery_voltage   = ts_get_f64(&mut con, "Kelly:battery_voltage").await;
    let throttle_raw      = ts_get_f64(&mut con, "Kelly:throttle_signal").await;
    let controller_temp   = ts_get_f64(&mut con, "Kelly:controller_temperature").await;
    let motor_temp        = ts_get_f64(&mut con, "Kelly:motor_temperature").await;
    let over_voltage      = ts_get_f64(&mut con, "Kelly:over_voltage").await;
    let low_voltage       = ts_get_f64(&mut con, "Kelly:low_voltage").await;
    let stall             = ts_get_f64(&mut con, "Kelly:stall").await;
    let motor_over_temp   = ts_get_f64(&mut con, "Kelly:motor_over_temperature").await;
    let command_status    = ts_get_f64(&mut con, "Kelly:command_status").await;
    let brake             = ts_get_f64(&mut con, "Kelly:brake_switch").await;
    let rpm               = ts_get_f64(&mut con, "Kelly:rpm").await;       // add this key when available
    let speed             = ts_get_f64(&mut con, "Kelly:speed").await;     // add this key when available

    // ── MPPT keys ─────────────────────────────────────────────────────────────
    let solar_v           = ts_get_f64(&mut con, "mppt:input_voltage").await;
    let solar_i           = ts_get_f64(&mut con, "mppt:input_current").await;
    let motor_v           = ts_get_f64(&mut con, "mppt:output_voltage").await;
    let motor_i           = ts_get_f64(&mut con, "mppt:output_current").await;
    let mppt_fault        = ts_get_f64(&mut con, "mppt:error_mosfet_overheat").await;
    // mppt_limiting and mppt_aux_fault: add keys here as they're wired up

    let direction = match command_status as u8 {
        0 => MotorDirection::Reverse,
        1 => MotorDirection::Forward,
        _ => MotorDirection::Neutral,
    };

    Ok(Telemetry {
        rpm:                  rpm as f32,
        speed:                speed as f32,
        battery_voltage:      battery_voltage as f32,
        rpm_peak:             rpm as f32, // peak tracked in update(), not here
        fault_over_voltage:   over_voltage > 0.5,
        fault_low_voltage:    low_voltage > 0.5,
        fault_stall:          stall > 0.5,
        fault_motor_over_temp: motor_over_temp > 0.5,
        throttle:             (throttle_raw / 255.0) as f32, // normalize 0–255 → 0–1
        motor_temp:           motor_temp as f32,
        controller_temp:      controller_temp as f32,
        motor_direction:      direction,
        brake:                brake > 0.5,
        solar_power_in:       (solar_v * solar_i) as f32,
        motor_power_out:      (motor_v * motor_i) as f32,
        mppt_fault:           mppt_fault > 0.5,
        mppt_limiting:        false, // wire up when key available
        mppt_aux_fault:       false, // wire up when key available
        tick:                 0.0,   // unused in live mode
    })
}

// ─── Colors ───────────────────────────────────────────────────────────────────

const BG_ROOT:       Color = Color { r: 0.04,  g: 0.045, b: 0.055, a: 1.0 };
const BG_PANEL:      Color = Color { r: 0.075, g: 0.08,  b: 0.10,  a: 1.0 };
const COLOR_BORDER:  Color = Color { r: 0.14,  g: 0.16,  b: 0.21,  a: 1.0 };
const COLOR_DIM:     Color = Color { r: 0.35,  g: 0.38,  b: 0.44,  a: 1.0 };
const COLOR_TEXT:    Color = Color { r: 0.88,  g: 0.91,  b: 0.96,  a: 1.0 };
const CYAN:          Color = Color { r: 0.0,   g: 0.85,  b: 0.95,  a: 1.0 };
const GREEN:         Color = Color { r: 0.18,  g: 0.95,  b: 0.48,  a: 1.0 };
const AMBER:         Color = Color { r: 1.0,   g: 0.72,  b: 0.0,   a: 1.0 };
const RED:           Color = Color { r: 1.0,   g: 0.22,  b: 0.22,  a: 1.0 };

const PACK_V_MIN: f32 = 84.0;
const PACK_V_MAX: f32 = 102.0;
const SOC_SEGMENTS: usize = 10;

// ─── Telemetry ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum MotorDirection { Forward, Neutral, Reverse }

#[derive(Debug, Clone)]
struct Telemetry {
    rpm:                  f32,
    speed:                f32,
    battery_voltage:      f32,
    rpm_peak:             f32,          // maintained in update(), not fetched
    fault_over_voltage:   bool,
    fault_low_voltage:    bool,
    fault_stall:          bool,
    fault_motor_over_temp: bool,
    throttle:             f32,
    motor_temp:           f32,
    controller_temp:      f32,
    motor_direction:      MotorDirection,
    brake:                bool,
    solar_power_in:       f32,
    motor_power_out:      f32,
    mppt_fault:           bool,
    mppt_limiting:        bool,
    mppt_aux_fault:       bool,
    tick:                 f32,          // unused in live mode, kept for compat
}

impl Default for Telemetry {
    fn default() -> Self {
        Self {
            rpm: 0.0,
            speed: 0.0,
            battery_voltage: 0.0,
            rpm_peak: 0.0,
            fault_over_voltage: false,
            fault_low_voltage: false,
            fault_stall: false,
            fault_motor_over_temp: false,
            throttle: 0.0,
            motor_temp: 0.0,
            controller_temp: 0.0,
            motor_direction: MotorDirection::Neutral,
            brake: false,
            solar_power_in: 0.0,
            motor_power_out: 0.0,
            mppt_fault: false,
            mppt_limiting: false,
            mppt_aux_fault: false,
            tick: 0.0,
        }
    }
}

impl Telemetry {
    fn net_power(&self) -> f32 { self.solar_power_in - self.motor_power_out }
    fn any_kelly_fault(&self) -> bool {
        self.fault_over_voltage || self.fault_low_voltage
            || self.fault_stall || self.fault_motor_over_temp
    }
    fn any_mppt_fault(&self) -> bool { self.mppt_fault || self.mppt_aux_fault }
    fn any_fault(&self) -> bool { self.any_kelly_fault() || self.any_mppt_fault() }
}

// ─── App ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
struct Dashboard { data: Telemetry }

#[derive(Debug, Clone)]
enum Message {
    Tick,
    TelemetryLoaded(Result<Telemetry, String>),
}

impl Dashboard {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Tick => {
                // Kick off an async Redis fetch. iced awaits it on its internal
                // executor and delivers the result as Message::TelemetryLoaded.
                Task::perform(
                    fetch_telemetry(),
                    |result| Message::TelemetryLoaded(result.map_err(|e| e.to_string())),
                )
            }
            Message::TelemetryLoaded(Ok(mut fresh)) => {
                // Peak-hold: carry the running maximum forward across ticks.
                // This is permanent logic (not mock scaffolding) — it lives here
                // in update() because it's derived state that spans multiple reads.
                fresh.rpm_peak = self.data.rpm_peak.max(fresh.rpm);
                self.data = fresh;
                Task::none()
            }
            Message::TelemetryLoaded(Err(e)) => {
                // Redis unavailable or key missing: log and keep last good frame.
                // The dashboard stays alive and shows stale data rather than crashing.
                eprintln!("telemetry fetch error: {e}");
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        // 100 ms poll rate — fast enough for smooth needle movement,
        // slow enough not to saturate the Pi's Redis connection.
        time::every(Duration::from_millis(100)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_,Message> {
        let d = &self.data;

        let gauges = row![
            gauge_panel("RPM",   d.rpm,   0.0, 5000.0,
                &format!("{:.0}", d.rpm),       level_color(d.rpm / 5000.0),
                Some(d.rpm_peak / 5000.0)),
            gauge_panel("SPEED", d.speed, 0.0,  120.0,
                &format!("{:.0} mph", d.speed), level_color(d.speed / 120.0),
                None),
            soc_panel(d.battery_voltage),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::FillPortion(5));

        let net = d.net_power();
        let power_strip = row![
            power_panel("SOLAR IN",  d.solar_power_in,  GREEN, false),
            power_panel("MOTOR OUT", d.motor_power_out, CYAN,  false),
            power_panel("NET POWER", net.abs(),
                if net >= 0.0 { GREEN } else { RED }, net < 0.0),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::FillPortion(2));

        let status = row![
            status_panel("THROTTLE",   &format!("{:.0}%",  d.throttle * 100.0),
                d.throttle,                CYAN),
            direction_panel(&d.motor_direction),
            status_panel("MOTOR TEMP", &format!("{:.1}°C", d.motor_temp),
                d.motor_temp / 100.0,      level_color(d.motor_temp / 100.0)),
            status_panel("CTRL TEMP",  &format!("{:.1}°C", d.controller_temp),
                d.controller_temp / 80.0,  level_color(d.controller_temp / 80.0)),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::FillPortion(2));

        let main_col = column![gauges, power_strip, status]
            .spacing(8)
            .width(Length::FillPortion(4))
            .height(Length::Fill);

        let any = d.any_fault();

        let fault_col = column![
            fault_header(any),
            fault_section_label("KELLY"),
            fault_row("Over Voltage", d.fault_over_voltage),
            fault_row("Low Voltage",  d.fault_low_voltage),
            fault_row("Stall",        d.fault_stall),
            fault_row("Motor Temp",   d.fault_motor_over_temp),
            fault_section_label("MPPT"),
            fault_row("Fault",        d.mppt_fault),
            fault_row("Limiting",     d.mppt_limiting),
            fault_row("Aux Supply",   d.mppt_aux_fault),
        ]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

        let fault_box = container(fault_col)
            .style(move |_: &Theme| container::Style {
                background: Some(iced::Background::Color(
                    if any { Color { r: 0.12, g: 0.03, b: 0.03, a: 1.0 } } else { BG_PANEL }
                )),
                border: iced::Border {
                    color: if any { RED } else { COLOR_BORDER },
                    width: if any { 1.5 } else { 1.0 },
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .width(Length::FillPortion(1))
            .height(Length::Fill);

        let root = row![main_col, fault_box]
            .spacing(8)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill);

        container(root)
            .style(|_: &Theme| container::Style {
                background: Some(iced::Background::Color(BG_ROOT)),
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// ─── Color helpers ────────────────────────────────────────────────────────────

fn level_color(frac: f32) -> Color {
    if frac >= 0.85 { RED } else if frac >= 0.65 { AMBER } else { CYAN }
}

// ─── Shared canvas helpers ────────────────────────────────────────────────────

fn draw_arc(frame: &mut Frame, cx: f32, cy: f32, r: f32,
            start: f32, end: f32, color: Color, width: f32, steps: usize) {
    let mut b = canvas::path::Builder::new();
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let a = start + (end - start) * t;
        let p = iced::Point::new(cx + a.cos() * r, cy + a.sin() * r);
        if i == 0 { b.move_to(p); } else { b.line_to(p); }
    }
    frame.stroke(&b.build(), Stroke::default().with_color(color).with_width(width));
}

fn canvas_text(content: String, x: f32, y: f32, size: f32, color: Color,
               ax: iced::alignment::Horizontal, ay: iced::alignment::Vertical) -> canvas::Text {
    canvas::Text {
        content,
        position: iced::Point::new(x, y),
        color,
        size: iced::Pixels(size),
        font: Font::MONOSPACE,
        align_x: ax.into(),
        align_y: ay.into(),
        ..canvas::Text::default()
    }
}

fn panel_container<'a>(c: impl canvas::Program<Message> + 'a,
                        portion: u16, h: Length) -> Element<'a, Message> {
    container(
        canvas(c).width(Length::Fill).height(Length::Fill)
    )
    .style(|_: &Theme| container::Style {
        background: Some(iced::Background::Color(BG_PANEL)),
        border: iced::Border { color: COLOR_BORDER, width: 1.0, radius: 4.0.into() },
        ..Default::default()
    })
    .width(Length::FillPortion(portion))
    .height(h)
    .padding(8)
    .into()
}

// ─── Gauge Canvas ─────────────────────────────────────────────────────────────

struct GaugeCanvas {
    label: String, value: f32, min: f32, max: f32,
    display: String, color: Color, peak_frac: Option<f32>,
}

impl<Message> canvas::Program<Message> for GaugeCanvas {
    type State = ();
    fn draw(&self, _: &(), renderer: &iced::Renderer, _: &Theme,
            bounds: iced::Rectangle, _: iced::mouse::Cursor) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let cx = bounds.width / 2.0;
            let cy = bounds.height * 0.50;
            let r  = (bounds.width.min(bounds.height) * 0.38).min(cy - 12.0);
            let start: f32 = std::f32::consts::PI * (5.0 / 4.0);
            let sweep: f32 = std::f32::consts::PI * 1.5;
            let frac = ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0);

            draw_arc(frame, cx, cy, r, start, start + sweep,
                Color { r: 0.18, g: 0.20, b: 0.26, a: 1.0 }, 7.0, 120);
            if frac > 0.005 {
                draw_arc(frame, cx, cy, r, start, start + sweep * frac, self.color, 7.0, 120);
            }
            for i in 0..=10 {
                let a = start + sweep * (i as f32 / 10.0);
                let major = i % 5 == 0;
                let ir = if major { r - 13.0 } else { r - 7.0 };
                frame.stroke(
                    &Path::line(
                        iced::Point::new(cx + a.cos() * ir,        cy + a.sin() * ir),
                        iced::Point::new(cx + a.cos() * (r + 1.0), cy + a.sin() * (r + 1.0)),
                    ),
                    Stroke::default()
                        .with_color(Color { r: 0.4, g: 0.42, b: 0.5, a: 0.6 })
                        .with_width(if major { 2.0 } else { 1.0 }),
                );
            }
            let na = start + sweep * frac;
            frame.stroke(
                &Path::line(
                    iced::Point::new(cx, cy),
                    iced::Point::new(cx + na.cos() * (r - 4.0), cy + na.sin() * (r - 4.0)),
                ),
                Stroke::default().with_color(Color::WHITE).with_width(2.0),
            );
            if let Some(pf) = self.peak_frac {
                let pa = start + sweep * pf.clamp(0.0, 1.0);
                frame.stroke(
                    &Path::line(
                        iced::Point::new(cx + pa.cos() * (r - 14.0), cy + pa.sin() * (r - 14.0)),
                        iced::Point::new(cx + pa.cos() * (r + 1.0),  cy + pa.sin() * (r + 1.0)),
                    ),
                    Stroke::default().with_color(AMBER).with_width(2.5),
                );
            }
            frame.fill(&Path::circle(iced::Point::new(cx, cy), 5.0),
                canvas::Fill { style: canvas::Style::Solid(self.color), ..canvas::Fill::default() });
            frame.fill_text(canvas_text(self.display.clone(), cx, cy + r * 0.42, 20.0,
                COLOR_TEXT, iced::alignment::Horizontal::Center, iced::alignment::Vertical::Center));
            frame.fill_text(canvas_text(self.label.clone(), cx, bounds.height - 5.0, 10.0,
                COLOR_DIM, iced::alignment::Horizontal::Center, iced::alignment::Vertical::Bottom));
        });
        vec![geom]
    }
}

fn gauge_panel<'a>(label: &str, value: f32, min: f32, max: f32,
                   display: &str, color: Color, peak_frac: Option<f32>) -> Element<'a, Message> {
    panel_container(GaugeCanvas {
        label: label.to_string(), value, min, max,
        display: display.to_string(), color, peak_frac,
    }, 1, Length::Fill)
}

// ─── SoC Canvas ───────────────────────────────────────────────────────────────

struct SocCanvas { voltage: f32, soc: f32 }

impl<Message> canvas::Program<Message> for SocCanvas {
    type State = ();
    fn draw(&self, _: &(), renderer: &iced::Renderer, _: &Theme,
            bounds: iced::Rectangle, _: iced::mouse::Cursor) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width; let h = bounds.height; let cx = w / 2.0;
            let body_w = w * 0.42; let body_h = h * 0.58;
            let body_x = cx - body_w / 2.0; let body_y = h * 0.14;
            let nub_w = body_w * 0.3;
            frame.fill_rectangle(
                iced::Point::new(cx - nub_w / 2.0, body_y - 6.0),
                iced::Size::new(nub_w, 6.0),
                Color { r: 0.25, g: 0.27, b: 0.32, a: 1.0 },
            );
            frame.stroke(
                &Path::rectangle(iced::Point::new(body_x, body_y), iced::Size::new(body_w, body_h)),
                Stroke::default().with_color(COLOR_BORDER).with_width(1.5),
            );
            let seg_color = if self.soc > 0.5 { GREEN } else if self.soc > 0.2 { AMBER } else { RED };
            let seg_margin = 3.0;
            let seg_h = (body_h - seg_margin * (SOC_SEGMENTS as f32 + 1.0)) / SOC_SEGMENTS as f32;
            let filled = (self.soc * SOC_SEGMENTS as f32).round() as usize;
            for i in 0..SOC_SEGMENTS {
                let seg_y = body_y + body_h - seg_margin
                    - (i as f32 + 1.0) * (seg_h + seg_margin) + seg_margin;
                frame.fill_rectangle(
                    iced::Point::new(body_x + seg_margin, seg_y),
                    iced::Size::new(body_w - seg_margin * 2.0, seg_h),
                    if i < filled { seg_color } else { Color { r: 0.14, g: 0.16, b: 0.21, a: 1.0 } },
                );
            }
            frame.fill_text(canvas_text(format!("{:.0}%", self.soc * 100.0),
                cx, body_y + body_h + 10.0, 18.0, seg_color,
                iced::alignment::Horizontal::Center, iced::alignment::Vertical::Top));
            frame.fill_text(canvas_text(format!("{:.1}V", self.voltage),
                cx, body_y + body_h + 28.0, 10.0, COLOR_DIM,
                iced::alignment::Horizontal::Center, iced::alignment::Vertical::Top));
            frame.fill_text(canvas_text("BATTERY SOC".to_string(),
                cx, h - 5.0, 10.0, COLOR_DIM,
                iced::alignment::Horizontal::Center, iced::alignment::Vertical::Bottom));
        });
        vec![geom]
    }
}

fn soc_panel<'a>(voltage: f32) -> Element<'a, Message> {
    let soc = ((voltage - PACK_V_MIN) / (PACK_V_MAX - PACK_V_MIN)).clamp(0.0, 1.0);
    panel_container(SocCanvas { voltage, soc }, 1, Length::Fill)
}

// ─── Power Strip Canvas ───────────────────────────────────────────────────────

struct PowerCanvas { label: String, watts: f32, color: Color, negative: bool }

impl<Message> canvas::Program<Message> for PowerCanvas {
    type State = ();
    fn draw(&self, _: &(), renderer: &iced::Renderer, _: &Theme,
            bounds: iced::Rectangle, _: iced::mouse::Cursor) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width; let h = bounds.height;
            let cx = w / 2.0; let cy = h / 2.0;
            let prefix = if self.negative { "−" } else { "" };
            frame.fill_text(canvas_text(
                format!("{}{:.0} W", prefix, self.watts), cx, cy - 6.0, 22.0, self.color,
                iced::alignment::Horizontal::Center, iced::alignment::Vertical::Center));
            frame.fill_text(canvas_text(
                self.label.clone(), 0.0, 0.0, 10.0, COLOR_DIM,
                iced::alignment::Horizontal::Left, iced::alignment::Vertical::Top));
        });
        vec![geom]
    }
}

fn power_panel<'a>(label: &str, watts: f32, color: Color, negative: bool) -> Element<'a, Message> {
    panel_container(PowerCanvas { label: label.to_string(), watts, color, negative }, 1, Length::Fill)
}

// ─── Status Canvas ────────────────────────────────────────────────────────────

struct StatusCanvas { label: String, val_str: String, fill: f32, color: Color }

impl<Message> canvas::Program<Message> for StatusCanvas {
    type State = ();
    fn draw(&self, _: &(), renderer: &iced::Renderer, _: &Theme,
            bounds: iced::Rectangle, _: iced::mouse::Cursor) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width; let h = bounds.height;
            let bar_h = 5.0; let bar_y = h - bar_h - 2.0;
            frame.fill_rectangle(iced::Point::new(0.0, bar_y), iced::Size::new(w, bar_h),
                Color { r: 0.14, g: 0.16, b: 0.21, a: 1.0 });
            if self.fill > 0.01 {
                frame.fill_rectangle(iced::Point::new(0.0, bar_y),
                    iced::Size::new(w * self.fill, bar_h), self.color);
            }
            frame.fill_text(canvas_text(self.label.clone(), 0.0, 0.0, 10.0, COLOR_DIM,
                iced::alignment::Horizontal::Left, iced::alignment::Vertical::Top));
            frame.fill_text(canvas_text(self.val_str.clone(), w / 2.0,
                (h - bar_h - 6.0) / 2.0, 17.0, self.color,
                iced::alignment::Horizontal::Center, iced::alignment::Vertical::Center));
        });
        vec![geom]
    }
}

fn status_panel<'a>(label: &str, val: &str, fill: f32, color: Color) -> Element<'a, Message> {
    panel_container(StatusCanvas {
        label: label.to_string(), val_str: val.to_string(),
        fill: fill.clamp(0.0, 1.0), color,
    }, 1, Length::Fill)
}

// ─── Direction Canvas ─────────────────────────────────────────────────────────

struct DirectionCanvas { direction: MotorDirection }

impl<Message> canvas::Program<Message> for DirectionCanvas {
    type State = ();
    fn draw(&self, _: &(), renderer: &iced::Renderer, _: &Theme,
            bounds: iced::Rectangle, _: iced::mouse::Cursor) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width; let h = bounds.height; let cx = w / 2.0;
            let (label, color) = match self.direction {
                MotorDirection::Forward => ("▶ FWD",  CYAN),
                MotorDirection::Neutral => ("■ NEUT", AMBER),
                MotorDirection::Reverse => ("◀ REV",  RED),
            };
            let pill_w = w * 0.82; let pill_h = h * 0.44;
            let pill_x = cx - pill_w / 2.0; let pill_y = (h - pill_h) / 2.0 - 4.0;
            frame.fill_rectangle(iced::Point::new(pill_x, pill_y),
                iced::Size::new(pill_w, pill_h),
                Color { r: color.r * 0.15, g: color.g * 0.15, b: color.b * 0.15, a: 1.0 });
            frame.stroke(
                &Path::rectangle(iced::Point::new(pill_x, pill_y), iced::Size::new(pill_w, pill_h)),
                Stroke::default().with_color(color).with_width(1.5));
            frame.fill_text(canvas_text(label.to_string(), cx, pill_y + pill_h / 2.0, 14.0, color,
                iced::alignment::Horizontal::Center, iced::alignment::Vertical::Center));
            frame.fill_text(canvas_text("DIRECTION".to_string(), 0.0, 0.0, 10.0, COLOR_DIM,
                iced::alignment::Horizontal::Left, iced::alignment::Vertical::Top));
        });
        vec![geom]
    }
}

fn direction_panel<'a>(direction: &MotorDirection) -> Element<'a, Message> {
    panel_container(DirectionCanvas { direction: direction.clone() }, 1, Length::Fill)
}

// ─── Fault Panel ──────────────────────────────────────────────────────────────

fn fault_header<'a>(any: bool) -> Element<'a, Message> {
    container(
        text(if any { "⚡ FAULTS" } else { "FAULTS" })
            .font(Font::MONOSPACE).size(11)
            .color(if any { RED } else { COLOR_DIM })
    )
    .padding(iced::Padding { top: 8.0, right: 8.0, bottom: 4.0, left: 8.0 })
    .width(Length::Fill).into()
}

fn fault_section_label<'a>(label: &'a str) -> Element<'a, Message> {
    container(text(label).font(Font::MONOSPACE).size(9).color(COLOR_DIM))
        .padding(iced::Padding { top: 6.0, right: 8.0, bottom: 2.0, left: 8.0 })
        .width(Length::Fill).into()
}

fn fault_row<'a>(label: &'a str, active: bool) -> Element<'a, Message> {
    let dot = if active { "●" } else { "○" };
    let col = if active { RED } else { COLOR_DIM };
    container(
        row![
            text(dot).font(Font::MONOSPACE).size(10).color(col),
            text(label).font(Font::MONOSPACE).size(10)
                .color(if active { RED } else { COLOR_DIM })
                .width(Length::Fill),
        ].spacing(5)
    )
    .style(move |_: &Theme| container::Style {
        background: Some(iced::Background::Color(
            if active { Color { r: 0.35, g: 0.0, b: 0.0, a: 0.18 } } else { Color::TRANSPARENT }
        )),
        ..Default::default()
    })
    .padding([5, 8])
    .width(Length::Fill)
    .into()
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn dashboard_theme(_: &Dashboard) -> Theme { Theme::Dark }

// Note: NO #[tokio::main] here. iced::application().run() manages the executor.
fn main() -> iced::Result {
    iced::application(Dashboard::default, Dashboard::update, Dashboard::view)
        .title("Solar Car · Dashboard")
        .subscription(Dashboard::subscription)
        .theme(dashboard_theme)
        .window(iced::window::Settings {
            size: Size::new(800.0, 480.0),
            resizable: false,
            ..Default::default()
        })
        .run()
}
