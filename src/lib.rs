// mayaqua/src/lib.rs - Main mayaqua library

pub mod pack_types;
pub mod pack_reader;
pub mod pack_writer;
pub mod encrypt;
pub mod memory;
pub mod network;
pub mod sock;
pub mod http;

// Re-export commonly used types and functions
pub use pack_types::*;
pub use pack_reader::{read_pack, read_element, read_value};
#[allow(unused_imports)]
pub use pack_writer::*;
pub use encrypt::*;
pub use sock::AsyncSock;
pub use http::{http_client_send, HTTP_VPN_TARGET, HTTP_VPN_TARGET2, HTTP_PACK_RAND_SIZE_MAX};

// Helper functions for socket operations
pub async fn sock_send_all(sock: &mut AsyncSock, data: &[u8]) -> Result<(), anyhow::Error> {
    sock.send_all(data).await.map_err(|e| anyhow::anyhow!("Send failed: {}", e))
}

pub async fn sock_recv_exact(sock: &mut AsyncSock, buf: &mut [u8], blocking: bool) -> Result<usize, anyhow::Error> {
    sock.recv_exact(buf, blocking).await.map_err(|e| anyhow::anyhow!("Receive failed: {}", e))
}
