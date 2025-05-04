use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn};
use async_ffi::async_ffi;
use async_std::{net::TcpStream, prelude::*};
use interface::{ConnectionPlugin, ConnectionPlugin_Ref, ConnectionTimerBox};
use std::{net::SocketAddr, time::Duration};

#[export_root_module]
pub fn get_library() -> ConnectionPlugin_Ref {
    ConnectionPlugin { run_connection }.leak_into_prefix()
}

#[async_ffi]
#[sabi_extern_fn]
pub async fn run_connection(mut connection_timer: ConnectionTimerBox) {
    let (ip, port) = connection_timer.ip_v4().into_tuple();
    let ip = SocketAddr::new(ip.into(), port);

    // start counting time
    connection_timer.start();

    let mut socket = None;
    for _ in 0.. {
        match TcpStream::connect(ip).await {
            Ok(stream) => {
                socket = Some(stream);
                break;
            }
            Err(err) => async_std::task::sleep(Duration::from_millis(10)).await,
        }
    }

    let mut socket = socket.unwrap();
    let buffer = vec![1; 1024];

    // header
    socket
        .write_all(&buffer.len().to_le_bytes())
        .await
        .expect("server crashed");

    // content
    socket.write_all(&buffer).await.expect("server crashed");

    // response
    let mut header = [0u8; 8];
    socket.read_exact(&mut header).await.expect("crash");

    let len = usize::from_le_bytes(header);
    let mut buffer = Vec::with_capacity(len);
    socket.read_exact(&mut buffer).await.expect("crash");

    // stop counting time - send to chart
    connection_timer.stop();
}
