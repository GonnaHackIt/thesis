use benchmark::ConnectionManager;
use containers::ConnectionChart;
use iced::{
    Element, Length, Task,
    widget::{column, container, row},
};
use plotters::prelude::*;
use plotters_iced::ChartBuilder;
use rfd::{AsyncFileDialog, FileHandle};
use std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration};
use tokio::sync::mpsc::UnboundedSender;

mod benchmark;
mod containers;

fn main() -> iced::Result {
    iced::application("Benchserv", Main::update, Main::view)
        .subscription(|_state| iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick))
        .run_with(|| {
            let mut state = Main::default();
            let plugins_path = std::env::var("PLUGINS_PATH").unwrap_or(String::from("plugins"));
            let plugin_names = load_plugins_names(&plugins_path);

            state.plugin_container.selected = plugin_names[0].clone();
            state.plugin_container.all = plugin_names;
            state.plugins_path = plugins_path;

            (state, Task::none())
        })
}

fn load_plugins_names(path: &str) -> Vec<String> {
    let dir = std::fs::read_dir(path).expect("Specified path for plugins doesn't exist");

    let mut names = vec![];
    for entry in dir {
        let Ok(entry) = entry else { continue };
        let Ok(file_type) = entry.file_type() else {
            continue;
        };

        if !file_type.is_file() {
            continue;
        }

        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };

        if !name.ends_with(".dll") && !name.ends_with(".so") {
            continue;
        }

        let name = name.split(".").next().unwrap();

        names.push(name.to_owned());
    }

    names
}

#[derive(Clone, Debug)]
enum Message {
    // ip address text inputs
    IpChanged(String),
    PortChanged(String),

    // plugin choosing
    PluginChange(String),

    // choosing mode
    ConstantModeChanged(bool),
    IncreaseModeChanged(bool),

    // choosing connections number
    ConnectionsChangedSlider(f64),
    ConnectionsChangedInput(String),

    // buttons
    RunTest,
    Pause,
    Resume,
    Stop,
    Save,

    FileDialog(Option<FileHandle>, ConnectionChart),

    // test data
    ConstantTestInitData(Arc<(benchmark::Wrapper<u64>, UnboundedSender<benchmark::Message>)>),
    IncreaseTestInitData(
        Arc<(
            benchmark::Wrapper<(u64, u64)>,
            UnboundedSender<benchmark::Message>,
        )>,
    ),

    // chart
    Tick,
    ConstantNewData(u64),
    IncreaseNewData((u64, u64)),
    NewChartData(u64),
}

#[derive(Default)]
struct Main {
    // inputs
    ip_container: containers::Ip,
    port_container: containers::Port,
    connections_container: containers::Connections,
    buttons_container: containers::Buttons,
    mode_container: containers::Mode,
    plugin_container: containers::PluginsSelect,

    // test conditions
    test_running: bool,
    paused: bool,
    cant_run: bool,

    // benchmark
    sender: Option<UnboundedSender<benchmark::Message>>,
    plugins_path: String,

    // chart data
    data_points_constant: Vec<u64>,
    data_points_increase: Vec<(u64, u64)>,

    chart: ConnectionChart,
}

impl Main {
    fn update(&mut self, message: Message) -> Task<Message> {
        macro_rules! send_msg {
            ($msg:expr) => {
                self.sender.as_mut().unwrap().send($msg).unwrap();
            };
        }

        use Message::*;

        match message {
            IpChanged(_) if !self.test_running => self.ip_container.update(message),
            PortChanged(_) if !self.test_running => self.port_container.update(message),
            ConnectionsChangedSlider(val) if !self.test_running => {
                if self.mode_container.increase {
                    self.chart.max_connections = Some(val as u64);
                }

                self.connections_container.update(message)
            }
            ConnectionsChangedInput(ref content) if !self.test_running => {
                if self.mode_container.increase {
                    if let Ok(val) = content.parse::<u64>() {
                        self.chart.max_connections = Some(val as u64);
                    }
                }

                self.connections_container.update(message)
            }
            PluginChange(_) if !self.test_running => self.plugin_container.update(message),
            ConstantModeChanged(_) if !self.test_running => {
                self.chart.max_connections = None;
                self.mode_container.update(message)
            }
            IncreaseModeChanged(_) if !self.test_running => {
                self.chart.max_connections = Some(self.connections_container.value as u64);
                self.mode_container.update(message)
            }
            RunTest if !self.test_running => {
                let containers = [
                    &self.ip_container.0,
                    &self.port_container.0,
                    &self.connections_container.text_state,
                ];

                let all_correct = containers.iter().map(|c| c.incorrect).all(|k| k == false);
                let all_not_empty = containers.iter().map(|c| &c.content).all(|s| s.len() > 0);
                let run_mode_chosen = self.mode_container.chosen();

                if !all_correct
                    || !all_not_empty
                    || !run_mode_chosen
                    || self.connections_container.value == 0.0
                {
                    self.cant_run = true;
                    return Task::none();
                }

                self.cant_run = false;
                self.test_running = true;
                self.chart.clear();

                let plugin_name = self.plugin_container.selected.clone();
                if self.mode_container.constant {
                    Task::perform(
                        ConnectionManager::run_test_constant(
                            self.connections_container.value as u64,
                            SocketAddr::from_str(&format!(
                                "{}:{}",
                                self.ip_container.content, self.port_container.content
                            ))
                            .unwrap(),
                            self.plugins_path.clone(),
                            plugin_name,
                        ),
                        Message::ConstantTestInitData,
                    )
                } else {
                    Task::perform(
                        ConnectionManager::run_test_increase(
                            self.connections_container.value as u64,
                            SocketAddr::from_str(&format!(
                                "{}:{}",
                                self.ip_container.content, self.port_container.content,
                            ))
                            .unwrap(),
                            self.plugins_path.clone(),
                            plugin_name,
                        ),
                        Message::IncreaseTestInitData,
                    )
                }
            }
            ConstantTestInitData(data) => {
                let (wrapper, sender) = Arc::into_inner(data).unwrap();
                self.sender = Some(sender);

                Task::run(wrapper, Message::ConstantNewData)
            }
            IncreaseTestInitData(data) => {
                let (wrapper, sender) = Arc::into_inner(data).unwrap();
                self.sender = Some(sender);

                Task::run(wrapper, Message::IncreaseNewData)
            }
            ConstantNewData(value) => {
                self.data_points_constant.push(value);

                Task::none()
            }
            IncreaseNewData(value) => {
                if self.data_points_increase.first().unwrap_or(&(0, 1u64)).1 != value.1 {
                    let averaged = {
                        let n = self.data_points_increase.len() as f64;
                        let sum = self
                            .data_points_increase
                            .iter()
                            .map(|(x, _)| x * x)
                            .sum::<u64>() as f64;
                        (sum / n).sqrt()
                    };

                    self.data_points_increase.clear();
                    ConnectionChart::update(
                        &mut self.chart,
                        Message::NewChartData(averaged as u64),
                    );
                }

                if value.0 == u64::MAX {
                    self.sender = None;
                    self.paused = false;
                    self.test_running = false;

                    return Task::none();
                }

                self.data_points_increase.push(value);

                Task::none()
            }
            Pause => {
                self.paused = true;
                send_msg!(benchmark::Message::Pause);
                Task::none()
            }
            Resume => {
                self.paused = false;
                send_msg!(benchmark::Message::Resume);
                Task::none()
            }
            Stop => {
                send_msg!(benchmark::Message::Stop);

                self.sender = None;
                self.paused = false;
                self.test_running = false;

                Task::none()
            }
            Save => {
                let chart = self.chart.clone();
                let task = async {
                    let file = AsyncFileDialog::new()
                        .add_filter("image", &["png", "jpg"])
                        .save_file()
                        .await;

                    (file, chart)
                };

                Task::perform(task, |result| Message::FileDialog(result.0, result.1))
            }
            FileDialog(Some(file_handle), chart) => {
                std::thread::spawn(move || {
                    let path = file_handle.path();
                    let root = BitMapBackend::new(path, (1920, 1080)).into_drawing_area();
                    root.fill(&plotters::style::RGBColor(32, 34, 37))
                        .expect("Unable to create chart");

                    let chart_builder = ChartBuilder::on(&root);
                    chart.build_chart_png(chart_builder);

                    root.present().expect("Unable to create chart");
                });

                Task::none()
            }
            Tick if self.test_running && !self.paused && self.mode_container.constant => {
                let averaged = {
                    let n = self.data_points_constant.len() as f64;
                    let sum = self.data_points_constant.iter().map(|x| x * x).sum::<u64>() as f64;
                    (sum / n).sqrt()
                };

                self.data_points_constant.clear();

                ConnectionChart::update(&mut self.chart, Message::NewChartData(averaged as u64));

                Task::none()
            }
            _ => Task::none(),
        }
    }
    fn view(&self) -> Element<Message> {
        return container(column![
            row![
                self.ip_container.view(),
                self.port_container.view(),
                self.plugin_container.view(),
                self.mode_container.view(),
            ]
            .push_maybe(
                self.mode_container.chosen().then_some(
                    self.connections_container
                        .view(self.mode_container.constant)
                )
            )
            .push(
                self.buttons_container
                    .view(self.cant_run, self.test_running, self.paused)
            )
            .spacing(10),
            container(self.chart.view())
                .width(Length::Fill)
                .height(Length::Fill)
        ])
        .padding(5)
        .into();
    }
}
