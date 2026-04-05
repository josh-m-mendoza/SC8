use iced::widget::{canvas, column, container, row, text};
use iced::{
    Color, Element, Font, Length, Size, Subscription, Task, Theme,
};
use iced::widget::canvas::{Cache, Frame, Geometry, Path, Stroke};
use iced::time;
use std::time::Duration;

// ─── Colors ───────────────────────────────────────────────────────────────────

const BG_ROOT: Color      = Color { r: 0.04,  g: 0.045, b: 0.055, a: 1.0 };
const BG_PANEL: Color     = Color { r: 0.075, g: 0.08,  b: 0.10,  a: 1.0 };
const COLOR_BORDER: Color = Color { r: 0.14,  g: 0.16,  b: 0.21,  a: 1.0 };
const COLOR_DIM: Color    = Color { r: 0.35,  g: 0.38,  b: 0.44,  a: 1.0 };
const COLOR_TEXT: Color   = Color { r: 0.88,  g: 0.91,  b: 0.96,  a: 1.0 };
const CYAN: Color         = Color { r: 0.0,   g: 0.85,  b: 0.95,  a: 1.0 };
const GREEN: Color        = Color { r: 0.18,  g: 0.95,  b: 0.48,  a: 1.0 };
const AMBER: Color        = Color { r: 1.0,   g: 0.72,  b: 0.0,   a: 1.0 };
const RED: Color          = Color { r: 1.0,   g: 0.22,  b: 0.22,  a: 1.0 };

// ─── Mock Telemetry ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum MotorDirection { Forward, Neutral, Reverse }

#[derive(Debug, Clone)]
struct Telemetry {
    rpm: f32,
    motor_current: f32,
    battery_voltage: f32,
    throttle: f32,
    brake: bool,
    motor_temp: f32,
    controller_temp: f32,
    motor_direction: MotorDirection,
    rpm_peak: f32,
    fault_over_voltage: bool,
    fault_low_voltage: bool,
    fault_stall: bool,
    fault_motor_over_temp: bool,
    tick: f32,
}

impl Default for Telemetry {
    fn default() -> Self {
        Self {
            rpm: 2340.0,
            motor_current: 47.3,
            battery_voltage: 98.6,
            throttle: 0.62,
            brake: false,
            motor_temp: 54.0,
            controller_temp: 41.0,
            motor_direction: MotorDirection::Forward,
            rpm_peak: 2340.0,
            fault_over_voltage: false,
            fault_low_voltage: false,
            fault_stall: false,
            fault_motor_over_temp: false,
            tick: 0.0,
        }
    }
}

impl Telemetry {
    fn advance(&mut self) {
        self.tick += 0.05;
        let t = self.tick;
        self.rpm              = (2340.0 + (t * 0.7).sin() * 420.0 + (t * 2.1).cos() * 80.0).max(0.0);
        self.motor_current    = (47.3  + (t * 1.1).sin() * 12.0).max(0.0);
        self.battery_voltage  = 98.6  + (t * 0.3).cos() * 1.8;
        self.throttle         = (0.62 + (t * 0.6).sin() * 0.25).clamp(0.0, 1.0);
        self.brake            = (t * 0.4).sin() > 0.85;
        self.motor_temp       = 54.0  + (t * 0.15).sin() * 8.0;
        self.controller_temp  = 41.0  + (t * 0.18).cos() * 5.0;

        // Peak hold — only goes up, never down
        if self.rpm > self.rpm_peak { self.rpm_peak = self.rpm; }

        // Cycle direction for demo: Forward → Neutral → Reverse
        let dir_cycle = (t * 0.04) as u32 % 12;
        self.motor_direction = match dir_cycle {
            0..=1  => MotorDirection::Neutral,
            2..=7  => MotorDirection::Forward,
            8..=9  => MotorDirection::Neutral,
            _      => MotorDirection::Reverse,
        };

        let cycle = (t * 0.08) as u32 % 20;
        self.fault_over_voltage    = cycle == 3;
        self.fault_low_voltage     = false;
        self.fault_stall           = cycle == 11;
        self.fault_motor_over_temp = self.motor_temp > 60.0;
    }

    fn any_fault(&self) -> bool {
        self.fault_over_voltage || self.fault_low_voltage
            || self.fault_stall || self.fault_motor_over_temp
    }
}

// ─── App State ────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
struct Dashboard {
    data: Telemetry,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Dashboard {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Tick => self.data.advance(),
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let d = &self.data;

        // ── Performance gauges (top 75% of left+center) ───────────────────────
        let gauges = row![
            gauge_panel("RPM",           d.rpm,           0.0,  5000.0, &format!("{:.0}",   d.rpm),           level_color(d.rpm / 5000.0), Some(d.rpm_peak / 5000.0)),
            gauge_panel("MOTOR CURRENT", d.motor_current, 0.0,  120.0,  &format!("{:.1} A", d.motor_current), level_color(d.motor_current / 120.0), None),
            soc_panel(d.battery_voltage),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::FillPortion(6));

        // ── Status strip (bottom 25% of left+center) ─────────────────────────
        let status = row![
            status_panel("THROTTLE",   &format!("{:.0}%",  d.throttle * 100.0), d.throttle,               CYAN),
            direction_panel(&d.motor_direction),
            status_panel("MOTOR TEMP", &format!("{:.1}°C", d.motor_temp),        d.motor_temp / 100.0,     level_color(d.motor_temp / 100.0)),
            status_panel("CTRL TEMP",  &format!("{:.1}°C", d.controller_temp),   d.controller_temp / 80.0, level_color(d.controller_temp / 80.0)),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::FillPortion(2));

        let main_col = column![gauges, status]
            .spacing(8)
            .width(Length::FillPortion(3))
            .height(Length::Fill);

        // ── Fault panel (right, full height) ─────────────────────────────────
        let any = d.any_fault();
        let fault_col = column![
            fault_header(any),
            fault_row("OVER VOLTAGE",    d.fault_over_voltage),
            fault_row("LOW VOLTAGE",     d.fault_low_voltage),
            fault_row("STALL ERROR",     d.fault_stall),
            fault_row("MOTOR OVER TEMP", d.fault_motor_over_temp),
        ]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

        let fault_box = container(fault_col)
            .style(move |_theme: &Theme| container::Style {
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
            .style(|_theme: &Theme| container::Style {
                background: Some(iced::Background::Color(BG_ROOT)),
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(100)).map(|_| Message::Tick)
    }
}

// ─── Color helpers ────────────────────────────────────────────────────────────

fn level_color(frac: f32) -> Color {
    if frac >= 0.85 { RED } else if frac >= 0.65 { AMBER } else { CYAN }
}

// ─── Gauge Canvas ─────────────────────────────────────────────────────────────

struct GaugeCanvas {
    label:     String,
    value:     f32,
    min:       f32,
    max:       f32,
    display:   String,
    color:     Color,
    peak_frac: Option<f32>, // 0.0–1.0, draws a second hold needle if Some
}

impl<Message> canvas::Program<Message> for GaugeCanvas {
    type State = ();

    fn draw(
        &self, _state: &(), renderer: &iced::Renderer, _theme: &Theme,
        bounds: iced::Rectangle, _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let cx = bounds.width / 2.0;
            let cy = bounds.height * 0.50;
            let r  = (bounds.width.min(bounds.height) * 0.38).min(cy - 12.0);
            let start: f32 = std::f32::consts::PI * (5.0 / 4.0); // 225°
            let sweep: f32 = std::f32::consts::PI * 1.5;           // 270° arc
            let frac = ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0);

            // Background track
            draw_arc(frame, cx, cy, r, start, start + sweep,
                Color { r: 0.18, g: 0.20, b: 0.26, a: 1.0 }, 7.0, 120);

            // Value arc
            if frac > 0.005 {
                draw_arc(frame, cx, cy, r, start, start + sweep * frac,
                    self.color, 7.0, 120);
            }

            // Tick marks
            for i in 0..=10 {
                let t = i as f32 / 10.0;
                let a = start + sweep * t;
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

            // Needle
            let na = start + sweep * frac;
            frame.stroke(
                &Path::line(
                    iced::Point::new(cx, cy),
                    iced::Point::new(cx + na.cos() * (r - 4.0), cy + na.sin() * (r - 4.0)),
                ),
                Stroke::default().with_color(Color::WHITE).with_width(2.0),
            );

            // Peak hold needle (thin, amber, stays at max)
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

            // Hub dot
            frame.fill(
                &Path::circle(iced::Point::new(cx, cy), 5.0),
                canvas::Fill {
                    style: canvas::Style::Solid(self.color),
                    ..canvas::Fill::default()
                },
            );

            // Value readout
            frame.fill_text(canvas::Text {
                content: self.display.clone(),
                position: iced::Point::new(cx, cy + r * 0.42),
                color: COLOR_TEXT,
                size: iced::Pixels(20.0),
                font: Font::MONOSPACE,
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Center.into(),
                line_height: iced::widget::text::LineHeight::Relative(1.3),
                shaping: iced::widget::text::Shaping::Basic,
                ..canvas::Text::default()
            });

            // Label
            frame.fill_text(canvas::Text {
                content: self.label.clone(),
                position: iced::Point::new(cx, bounds.height - 5.0),
                color: COLOR_DIM,
                size: iced::Pixels(10.0),
                font: Font::MONOSPACE,
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Bottom.into(),
                line_height: iced::widget::text::LineHeight::Relative(1.0),
                shaping: iced::widget::text::Shaping::Basic,
                ..canvas::Text::default()
            });
        });
        vec![geom]
    }
}

fn draw_arc(frame: &mut Frame, cx: f32, cy: f32, r: f32, start: f32, end: f32, color: Color, width: f32, steps: usize) {
    let mut b = canvas::path::Builder::new();
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let a = start + (end - start) * t;
        let p = iced::Point::new(cx + a.cos() * r, cy + a.sin() * r);
        if i == 0 { b.move_to(p); } else { b.line_to(p); }
    }
    frame.stroke(&b.build(), Stroke::default().with_color(color).with_width(width));
}

fn gauge_panel<'a>(
    label: &str, value: f32, min: f32, max: f32, display: &str, color: Color,
    peak_frac: Option<f32>,
) -> Element<'a, Message> {
    container(
        canvas(GaugeCanvas {
            label: label.to_string(), value, min, max,
            display: display.to_string(), color, peak_frac,
        })
        .width(Length::Fill)
        .height(Length::Fill)
    )
    .style(|_theme: &Theme| container::Style {
        background: Some(iced::Background::Color(BG_PANEL)),
        border: iced::Border { color: COLOR_BORDER, width: 1.0, radius: 4.0.into() },
        ..Default::default()
    })
    .width(Length::FillPortion(1))
    .height(Length::Fill)
    .padding(10)
    .into()
}

// ─── Status Canvas ────────────────────────────────────────────────────────────

struct StatusCanvas { label: String, val_str: String, fill: f32, color: Color }

impl<Message> canvas::Program<Message> for StatusCanvas {
    type State = ();
    fn draw(
        &self, _state: &(), renderer: &iced::Renderer, _theme: &Theme,
        bounds: iced::Rectangle, _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width;
            let h = bounds.height;
            let bar_h = 5.0;
            let bar_y = h - bar_h - 2.0;

            frame.fill_rectangle(
                iced::Point::new(0.0, bar_y), iced::Size::new(w, bar_h),
                Color { r: 0.14, g: 0.16, b: 0.21, a: 1.0 },
            );
            if self.fill > 0.01 {
                frame.fill_rectangle(
                    iced::Point::new(0.0, bar_y), iced::Size::new(w * self.fill, bar_h),
                    self.color,
                );
            }
            frame.fill_text(canvas::Text {
                content: self.label.clone(),
                position: iced::Point::new(0.0, 0.0),
                color: COLOR_DIM, size: iced::Pixels(10.0), font: Font::MONOSPACE,
                align_x: iced::alignment::Horizontal::Left.into(),
                align_y: iced::alignment::Vertical::Top.into(),
                line_height: iced::widget::text::LineHeight::Relative(1.0),
                shaping: iced::widget::text::Shaping::Basic,
                ..canvas::Text::default()
            });
            frame.fill_text(canvas::Text {
                content: self.val_str.clone(),
                position: iced::Point::new(w / 2.0, (h - bar_h - 6.0) / 2.0),
                color: self.color, size: iced::Pixels(17.0), font: Font::MONOSPACE,
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Center.into(),
                line_height: iced::widget::text::LineHeight::Relative(1.0),
                shaping: iced::widget::text::Shaping::Basic,
                ..canvas::Text::default()
            });
        });
        vec![geom]
    }
}

fn status_panel<'a>(label: &str, val: &str, fill: f32, color: Color) -> Element<'a, Message> {
    container(
        canvas(StatusCanvas {
            label: label.to_string(), val_str: val.to_string(),
            fill: fill.clamp(0.0, 1.0), color,
        })
        .width(Length::Fill)
        .height(Length::Fill)
    )
    .style(|_theme: &Theme| container::Style {
        background: Some(iced::Background::Color(BG_PANEL)),
        border: iced::Border { color: COLOR_BORDER, width: 1.0, radius: 4.0.into() },
        ..Default::default()
    })
    .width(Length::FillPortion(1))
    .height(Length::Fill)
    .padding([6, 10])
    .into()
}

// ─── Battery SoC Panel ────────────────────────────────────────────────────────
// Estimates SoC from voltage using a simple linear map over the usable range.
// Replace the min/max here to match your actual pack's voltage window.

const PACK_V_MIN: f32 = 84.0;  // 0% — adjust to your pack
const PACK_V_MAX: f32 = 102.0; // 100% — adjust to your pack
const SOC_SEGMENTS: usize = 10;

struct SocCanvas { voltage: f32, soc: f32 }

impl<Message> canvas::Program<Message> for SocCanvas {
    type State = ();
    fn draw(
        &self, _state: &(), renderer: &iced::Renderer, _theme: &Theme,
        bounds: iced::Rectangle, _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width;
            let h = bounds.height;
            let cx = w / 2.0;

            // Battery body dimensions
            let body_w = w * 0.42;
            let body_h = h * 0.62;
            let body_x = cx - body_w / 2.0;
            let body_y = h * 0.18;

            // Battery terminal nub at top
            let nub_w = body_w * 0.3;
            let nub_h = 6.0;
            frame.fill_rectangle(
                iced::Point::new(cx - nub_w / 2.0, body_y - nub_h),
                iced::Size::new(nub_w, nub_h),
                Color { r: 0.25, g: 0.27, b: 0.32, a: 1.0 },
            );

            // Battery outline
            frame.stroke(
                &Path::rectangle(
                    iced::Point::new(body_x, body_y),
                    iced::Size::new(body_w, body_h),
                ),
                Stroke::default().with_color(COLOR_BORDER).with_width(1.5),
            );

            // Filled segments from bottom up
            let seg_margin = 3.0;
            let seg_h = (body_h - seg_margin * (SOC_SEGMENTS as f32 + 1.0)) / SOC_SEGMENTS as f32;
            let filled = (self.soc * SOC_SEGMENTS as f32).round() as usize;

            let seg_color = if self.soc > 0.5 { GREEN }
                else if self.soc > 0.2 { AMBER }
                else { RED };

            for i in 0..SOC_SEGMENTS {
                let seg_y = body_y + body_h - seg_margin - (i as f32 + 1.0) * (seg_h + seg_margin) + seg_margin;
                let active = i < filled;
                frame.fill_rectangle(
                    iced::Point::new(body_x + seg_margin, seg_y),
                    iced::Size::new(body_w - seg_margin * 2.0, seg_h),
                    if active { seg_color } else { Color { r: 0.14, g: 0.16, b: 0.21, a: 1.0 } },
                );
            }

            // SoC % label
            frame.fill_text(canvas::Text {
                content: format!("{:.0}%", self.soc * 100.0),
                position: iced::Point::new(cx, body_y + body_h + 10.0),
                color: seg_color,
                size: iced::Pixels(18.0),
                font: Font::MONOSPACE,
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Top.into(),
                ..canvas::Text::default()
            });

            // Voltage label
            frame.fill_text(canvas::Text {
                content: format!("{:.1}V", self.voltage),
                position: iced::Point::new(cx, body_y + body_h + 28.0),
                color: COLOR_DIM,
                size: iced::Pixels(10.0),
                font: Font::MONOSPACE,
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Top.into(),
                ..canvas::Text::default()
            });

            // Label
            frame.fill_text(canvas::Text {
                content: "BATTERY SOC".to_string(),
                position: iced::Point::new(cx, h - 5.0),
                color: COLOR_DIM,
                size: iced::Pixels(10.0),
                font: Font::MONOSPACE,
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Bottom.into(),
                ..canvas::Text::default()
            });
        });
        vec![geom]
    }
}

fn soc_panel<'a>(voltage: f32) -> Element<'a, Message> {
    let soc = ((voltage - PACK_V_MIN) / (PACK_V_MAX - PACK_V_MIN)).clamp(0.0, 1.0);
    container(
        canvas(SocCanvas { voltage, soc })
            .width(Length::Fill)
            .height(Length::Fill)
    )
    .style(|_theme: &Theme| container::Style {
        background: Some(iced::Background::Color(BG_PANEL)),
        border: iced::Border { color: COLOR_BORDER, width: 1.0, radius: 4.0.into() },
        ..Default::default()
    })
    .width(Length::FillPortion(1))
    .height(Length::Fill)
    .padding(10)
    .into()
}

// ─── Motor Direction Panel ────────────────────────────────────────────────────

struct DirectionCanvas { direction: MotorDirection }

impl<Message> canvas::Program<Message> for DirectionCanvas {
    type State = ();
    fn draw(
        &self, _state: &(), renderer: &iced::Renderer, _theme: &Theme,
        bounds: iced::Rectangle, _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width;
            let h = bounds.height;
            let cx = w / 2.0;

            let (label, color) = match self.direction {
                MotorDirection::Forward => ("▶ FWD",  CYAN),
                MotorDirection::Neutral => ("■ NEUT", AMBER),
                MotorDirection::Reverse => ("◀ REV",  RED),
            };

            // Colored background pill
            let pill_w = w * 0.80;
            let pill_h = h * 0.44;
            let pill_x = cx - pill_w / 2.0;
            let pill_y = (h - pill_h) / 2.0 - 6.0;

            frame.fill_rectangle(
                iced::Point::new(pill_x, pill_y),
                iced::Size::new(pill_w, pill_h),
                Color { r: color.r * 0.15, g: color.g * 0.15, b: color.b * 0.15, a: 1.0 },
            );
            frame.stroke(
                &Path::rectangle(
                    iced::Point::new(pill_x, pill_y),
                    iced::Size::new(pill_w, pill_h),
                ),
                Stroke::default().with_color(color).with_width(1.5),
            );

            frame.fill_text(canvas::Text {
                content: label.to_string(),
                position: iced::Point::new(cx, pill_y + pill_h / 2.0),
                color,
                size: iced::Pixels(15.0),
                font: Font::MONOSPACE,
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Center.into(),
                ..canvas::Text::default()
            });

            // Label
            frame.fill_text(canvas::Text {
                content: "DIRECTION".to_string(),
                position: iced::Point::new(0.0, 0.0),
                color: COLOR_DIM,
                size: iced::Pixels(10.0),
                font: Font::MONOSPACE,
                align_x: iced::alignment::Horizontal::Left.into(),
                align_y: iced::alignment::Vertical::Top.into(),
                ..canvas::Text::default()
            });
        });
        vec![geom]
    }
}

fn direction_panel<'a>(direction: &MotorDirection) -> Element<'a, Message> {
    container(
        canvas(DirectionCanvas { direction: direction.clone() })
            .width(Length::Fill)
            .height(Length::Fill)
    )
    .style(|_theme: &Theme| container::Style {
        background: Some(iced::Background::Color(BG_PANEL)),
        border: iced::Border { color: COLOR_BORDER, width: 1.0, radius: 4.0.into() },
        ..Default::default()
    })
    .width(Length::FillPortion(1))
    .height(Length::Fill)
    .padding([6, 10])
    .into()
}

// ─── Fault Panel ──────────────────────────────────────────────────────────────

fn fault_header<'a>(any: bool) -> Element<'a, Message> {
    container(
        text(if any { "⚡ CRITICAL FAULTS" } else { "CRITICAL FAULTS" })
            .font(Font::MONOSPACE)
            .size(12)
            .color(if any { RED } else { COLOR_DIM })
    )
    .padding(iced::Padding { top: 10.0, right: 12.0, bottom: 6.0, left: 12.0 })
    .width(Length::Fill)
    .into()
}

fn fault_row<'a>(label: &'a str, active: bool) -> Element<'a, Message> {
    let dot = if active { "● FAULT" } else { "○  OK" };
    let col = if active { RED } else { COLOR_DIM };

    container(
        row![
            text(label)
                .font(Font::MONOSPACE)
                .size(11)
                .color(if active { RED } else { COLOR_DIM })
                .width(Length::Fill),
            text(dot)
                .font(Font::MONOSPACE)
                .size(11)
                .color(col),
        ]
        .spacing(4)
    )
    .style(move |_theme: &Theme| container::Style {
        background: Some(iced::Background::Color(
            if active { Color { r: 0.35, g: 0.0, b: 0.0, a: 0.18 } } else { Color::TRANSPARENT }
        )),
        ..Default::default()
    })
    .padding([9, 12])
    .width(Length::Fill)
    .into()
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn dashboard_theme(_state: &Dashboard) -> Theme {
    Theme::Dark
}

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