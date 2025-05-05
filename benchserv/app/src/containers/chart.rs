use super::*;
use crate::Message;
use iced::widget::{button, column, container, row, text};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use std::{collections::VecDeque, default::Default};

#[derive(Clone, Debug)]
pub struct ConnectionChart {
    pub index: u64,
    pub data_points: VecDeque<(u64, u64)>,
    pub max_connections: Option<u64>,
}

impl Default for ConnectionChart {
    fn default() -> Self {
        ConnectionChart {
            index: 1,
            data_points: VecDeque::new(),
            max_connections: None,
        }
    }
}

impl ConnectionChart {
    const X_AXIS: u64 = 60;

    pub fn clear(&mut self) {
        self.index = 1;
        self.data_points.clear();
    }

    pub fn update(&mut self, message: Message) {
        use Message::*;

        match message {
            NewChartData(data) => {
                if self.max_connections.is_none() && self.index > ConnectionChart::X_AXIS {
                    self.data_points.pop_front();
                }

                self.data_points.push_back((self.index, data));
                self.index += 1;
            }
            _ => {}
        }
    }
    pub fn view(&self) -> Element<Message> {
        column![
            container(
                row![
                    container(text!("Server Latency Test")).padding(Padding::default().top(4)),
                    container(text!("-")).padding(Padding::default().top(4)),
                    button("Save Chart").on_press(Message::Save)
                ]
                .spacing(10)
            )
            .width(Length::Fill)
            .align_x(Alignment::Center),
            ChartWidget::new(self),
        ]
        .into()
    }
}

impl ConnectionChart {
    fn x_range(&self) -> std::ops::Range<u64> {
        if let Some(val) = self.max_connections {
            return 1..val;
        }

        let front = self.data_points.front().unwrap_or(&(1, 0));
        let back = self
            .data_points
            .back()
            .unwrap_or(&(ConnectionChart::X_AXIS, 0));

        front.0..std::cmp::max(back.0, ConnectionChart::X_AXIS)
    }
    fn y_range(&self) -> std::ops::Range<u64> {
        let max = self
            .data_points
            .iter()
            .map(|(_idx, val)| *val)
            .max()
            .unwrap_or(50u64);

        // make some room on the top of chart
        let max = (max as f64 * 1.3) as u64;

        0..max
    }
}

impl Chart<Message> for ConnectionChart {
    type State = ();

    // method to build chart on the canvas
    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        use plotters::prelude::*;

        const PLOT_LINE_COLOR: RGBColor = RGBColor(0, 175, 255);

        let x_range = self.x_range();
        let y_range = self.y_range();

        let mut chart = chart
            .x_label_area_size(40)
            .y_label_area_size(40)
            .margin(20)
            .build_cartesian_2d(x_range, y_range)
            .expect("failed to build chart");

        chart
            .configure_mesh()
            .bold_line_style(WHITE.mix(0.1))
            .light_line_style(WHITE.mix(0.05))
            .axis_style(ShapeStyle::from(WHITE.mix(0.45)).stroke_width(1))
            .label_style(TextStyle::from(("sans-serif", 10).into_font()).color(&WHITE))
            .y_labels(10)
            .y_label_formatter(&|y: &u64| format!("{}ms", y))
            .x_labels(15)
            .x_label_formatter(&|x: &u64| {
                if self.max_connections.is_none() {
                    format!("{}s", x)
                } else {
                    format!("{x}")
                }
            })
            .draw()
            .expect("failed to draw chart mesh");

        chart
            .draw_series(
                AreaSeries::new(
                    self.data_points.iter().map(|x| (x.0, x.1)),
                    0,
                    PLOT_LINE_COLOR.mix(0.175),
                )
                .border_style(ShapeStyle::from(PLOT_LINE_COLOR).stroke_width(2)),
            )
            .expect("failed to draw chart data");
    }
}

impl ConnectionChart {
    // method to build chart in the png file
    pub fn build_chart_png<DB: DrawingBackend>(&self, mut chart: ChartBuilder<DB>) {
        use plotters::prelude::*;

        const PLOT_LINE_COLOR: RGBColor = RGBColor(0, 175, 255);

        let x_range = self.x_range();
        let y_range = self.y_range();

        let mut chart = chart
            .x_label_area_size(60)
            .y_label_area_size(100)
            .margin(20)
            .build_cartesian_2d(x_range, y_range)
            .expect("failed to build chart");

        chart
            .configure_mesh()
            .axis_desc_style(TextStyle::from(("sans-serif", 30).into_font()).color(&WHITE))
            .bold_line_style(WHITE.mix(0.30))
            .light_line_style(WHITE.mix(0.1))
            .axis_style(ShapeStyle::from(WHITE.mix(0.65)).stroke_width(1))
            .label_style(TextStyle::from(("sans-serif", 15).into_font()).color(&WHITE))
            .y_desc("Latency")
            .y_labels(10)
            .y_label_formatter(&|y: &u64| format!("{}ms", y))
            .x_desc(if self.max_connections.is_some() {
                "Connection Number"
            } else {
                "Time"
            })
            .x_labels(15)
            .x_label_formatter(&|x: &u64| {
                if self.max_connections.is_none() {
                    format!("{}s", x)
                } else {
                    format!("{x}")
                }
            })
            .draw()
            .expect("failed to draw chart mesh");

        chart
            .draw_series(
                AreaSeries::new(
                    self.data_points.iter().map(|x| (x.0, x.1)),
                    0,
                    PLOT_LINE_COLOR.mix(0.175),
                )
                .border_style(ShapeStyle::from(PLOT_LINE_COLOR).stroke_width(2)),
            )
            .expect("failed to draw chart data");
    }
}
