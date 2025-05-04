use std::{net::SocketAddr, path::Path};

use async_ffi::FfiFuture;

use abi_stable::{
    StableAbi, declare_root_module_statics,
    library::{LibraryError, RootModule},
    package_version_strings, sabi_trait,
    sabi_types::VersionStrings,
    std_types::{RBox, RVec, Tuple2},
};

#[sabi_trait]
pub trait ConnectionTimer: Send + Sync {
    /// start counting the time
    fn start(&mut self);
    /// stop counting the time and send latency to app
    fn stop(&self);
    fn ip_v4(&self) -> Tuple2<[u8; 4], u16>;
    fn ip_v6(&self) -> Tuple2<[u8; 16], u16>;
}

pub type ConnectionTimerBox = ConnectionTimer_TO<'static, RBox<()>>;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = ConnectionPlugin_Ref)))]
#[sabi(missing_field(panic))]
pub struct ConnectionPlugin {
    #[sabi(last_prefix_field)]
    /// this function is directly spawned onto a tokio runtime and should use the
    /// ConnectionTimerBox methods to measure the latency and show it on chart
    pub run_connection: extern "C" fn(ConnectionTimerBox) -> FfiFuture<()>,
}

impl RootModule for ConnectionPlugin_Ref {
    declare_root_module_statics! { ConnectionPlugin_Ref }

    const BASE_NAME: &'static str = "connections";
    const NAME: &'static str = "connections plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub fn load_root_module_from_file(path: &Path) -> Result<ConnectionPlugin_Ref, LibraryError> {
    ConnectionPlugin_Ref::load_from_file(path)
}
