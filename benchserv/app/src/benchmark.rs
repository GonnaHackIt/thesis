use abi_stable::{sabi_trait::TD_Opaque, std_types::Tuple2};

use futures::stream::Stream;
use interface::{ConnectionPlugin_Ref, ConnectionTimerBox, load_root_module_from_file};
use std::{
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{Poll, ready},
    time::Instant,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

#[derive(Debug)]
pub enum Message {
    Data(u64),
    Pause,
    Resume,
    Stop,
    ConnectionEnded,
}

#[derive(Debug)]
pub struct ConnectionManager {}

impl ConnectionManager {
    fn spawn_connection(
        plugin: &ConnectionPlugin_Ref,
        ip: SocketAddr,
        tx: UnboundedSender<Message>,
    ) {
        let connection_timer = ConnectionTimer::new(ip, tx);
        let connection_timer = ConnectionTimerBox::from_value(connection_timer, TD_Opaque);

        let task = plugin.run_connection()(connection_timer);
        tokio::spawn(task);
    }
}

// constant mode
impl ConnectionManager {
    pub async fn run_test_increase(
        max_connections: u64,
        ip: SocketAddr,
        plugins_path: String,
        plugin_name: String,
    ) -> Arc<(Wrapper<(u64, u64)>, UnboundedSender<Message>)> {
        let plugin = load_root_module_from_file(std::path::Path::new(&format!(
            "{plugins_path}/{plugin_name}"
        )))
        .expect("This plugin is not compatible");

        let (tx_wrapper, rx_wrapper) = unbounded_channel();
        let (tx_connections, mut rx_connections) = unbounded_channel();

        let mut paused = false;

        let mut current_connections_cap = 1;
        let mut current_connections = 1;

        let mut requests = 0;
        const REQUESTS_THRESHOLD: u64 = 10;

        //starting point
        let ip = ip;
        let tx = tx_connections.clone();
        ConnectionManager::spawn_connection(&plugin, ip, tx.clone());

        tokio::spawn(async move {
            while let Some(message) = rx_connections.recv().await {
                use Message::*;

                match message {
                    Data(latency) if current_connections_cap <= max_connections => {
                        requests += 1;
                        tx_wrapper.send((latency, current_connections_cap)).unwrap();
                    }
                    ConnectionEnded => {
                        current_connections -= 1;
                    }
                    Pause => paused = true,
                    Resume => paused = false,
                    Stop => break,
                    _ => {}
                }
                if current_connections_cap > max_connections {
                    continue;
                }

                if requests >= REQUESTS_THRESHOLD {
                    current_connections_cap += 1;
                    requests = 0;
                }

                if current_connections_cap > max_connections {
                    tx_wrapper
                        .send((u64::MAX, current_connections_cap))
                        .unwrap();
                    break;
                }

                if !paused && current_connections < current_connections_cap {
                    for _ in 0..(current_connections_cap - current_connections) {
                        let ip = ip;
                        let tx = tx_connections.clone();
                        ConnectionManager::spawn_connection(&plugin, ip, tx);
                        current_connections += 1;
                    }
                }
            }
        });

        Arc::new((Wrapper(rx_wrapper), tx))
    }
}

// incresing mode
impl ConnectionManager {
    pub async fn run_test_constant(
        connections_number: u64,
        ip: SocketAddr,
        plugins_path: String,
        plugin_name: String,
    ) -> Arc<(Wrapper<u64>, UnboundedSender<Message>)> {
        let plugin = load_root_module_from_file(std::path::Path::new(&format!(
            "{plugins_path}/{plugin_name}"
        )))
        .expect("This plugin is not compatible");

        let (tx_wrapper, rx_wrapper) = unbounded_channel();
        let (tx_connections, mut rx_connections) = unbounded_channel();

        // for return purposed
        let tx = tx_connections.clone();

        let mut paused = false;
        let mut current_connections = connections_number;

        for _ in 0..connections_number {
            let ip = ip;
            let tx = tx_connections.clone();
            ConnectionManager::spawn_connection(&plugin, ip, tx);
        }

        tokio::spawn(async move {
            while let Some(message) = rx_connections.recv().await {
                use Message::*;

                match message {
                    Data(latency) => {
                        tx_wrapper.send(latency).unwrap();
                    }
                    ConnectionEnded => {
                        current_connections -= 1;
                    }
                    Pause => paused = true,
                    Resume => paused = false,
                    Stop => break,
                }

                if !paused && current_connections < connections_number {
                    for _ in 0..(connections_number - current_connections) {
                        let ip = ip;
                        let tx = tx_connections.clone();
                        ConnectionManager::spawn_connection(&plugin, ip, tx);
                        current_connections += 1;
                    }
                }
            }
        });

        Arc::new((Wrapper(rx_wrapper), tx))
    }
}

#[derive(Debug)]
pub struct Wrapper<T>(UnboundedReceiver<T>);

impl<T> Stream for Wrapper<T> {
    type Item = T;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let result = ready!(self.0.poll_recv(cx));
        let Some(latency) = result else {
            return Poll::Ready(None);
        };

        return Poll::Ready(Some(latency));
    }
}

struct ConnectionTimer {
    ip: SocketAddr,
    start: Instant,
    tx: UnboundedSender<Message>,
}

impl ConnectionTimer {
    fn new(ip: SocketAddr, tx: UnboundedSender<Message>) -> Self {
        ConnectionTimer {
            ip,
            start: Instant::now(),
            tx,
        }
    }
}

impl Drop for ConnectionTimer {
    fn drop(&mut self) {
        let _ = self.tx.send(Message::ConnectionEnded);
    }
}

impl interface::ConnectionTimer for ConnectionTimer {
    fn start(&mut self) {
        self.start = Instant::now();
    }
    fn stop(&self) {
        let latency = self.start.elapsed().as_millis();
        let _ = self.tx.send(Message::Data(latency as u64));
    }
    fn ip_v4(&self) -> abi_stable::std_types::Tuple2<[u8; 4], u16> where {
        let SocketAddr::V4(addr) = self.ip else {
            panic!("its not ipv4")
        };

        let port = addr.port();
        let ip = addr.ip().octets();

        Tuple2::from_tuple((ip, port))
    }
    fn ip_v6(&self) -> Tuple2<[u8; 16], u16> where {
        let SocketAddr::V6(addr) = self.ip else {
            panic!("its not ipv4")
        };

        let port = addr.port();
        let ip = addr.ip().octets();

        Tuple2::from_tuple((ip, port))
    }
}
