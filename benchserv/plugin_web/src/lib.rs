use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn};
use async_compat::Compat;
use async_ffi::async_ffi;
use interface::{ConnectionPlugin, ConnectionPlugin_Ref, ConnectionTimerBox};
use reqwest::Client;
use std::{net::SocketAddr, sync::LazyLock};

#[export_root_module]
pub fn get_library() -> ConnectionPlugin_Ref {
    ConnectionPlugin { run_connection }.leak_into_prefix()
}

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| Client::new());

#[async_ffi]
#[sabi_extern_fn]
pub async fn run_connection(mut connection_timer: ConnectionTimerBox) {
    let (ip, port) = connection_timer.ip_v4().into_tuple();
    let ip = SocketAddr::new(ip.into(), port).to_string();

    // start counting time
    connection_timer.start();

    // compat is used because dynamic libraries don't see tokio runtime
    Compat::new(async {
        let request = CLIENT.get(format!("http://{ip}")).build().unwrap();
        let response = CLIENT.execute(request).await.unwrap().text().await.unwrap();

        //println!("{response}");
    })
    .await;

    connection_timer.stop();
}
