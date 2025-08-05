// mayaqua.rs - Direct translation of mayaqua.go

use thiserror::Error;

#[derive(Error, Debug)]
#[error("ERR_SERVER_IS_NOT_VPN")]
pub struct ErrServerIsNotVpn;

pub const ERR_SERVER_IS_NOT_VPN: ErrServerIsNotVpn = ErrServerIsNotVpn;
