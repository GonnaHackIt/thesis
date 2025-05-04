use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn};
use async_ffi::async_ffi;
use interface::{ConnectionPlugin, ConnectionPlugin_Ref, ConnectionTimerBox};

#[export_root_module]
pub fn get_library() -> ConnectionPlugin_Ref {
    ConnectionPlugin { run_connection }.leak_into_prefix()
}

#[async_ffi]
#[sabi_extern_fn]
pub async fn run_connection(mut connection_timer: ConnectionTimerBox) {}
