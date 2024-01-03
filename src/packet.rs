use std::collections::HashMap;
use std::net::{
  Ipv4Addr,
  SocketAddr
};
use std::str::FromStr;
use std::time::SystemTime;
use anyhow::Error;
use bincode::{
  Decode,
  Encode
};
use serde_derive::{
  Deserialize,
  Serialize
};
use crate::process_status::ProcessInfo;

pub const HEARTBEAT_PACKET_START_MARKER: u8 = 0xBE;
pub const HEARTBEAT_PACKET_END_MARKER: u8 = 0xEF;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct HeartbeatPacket
{
  sender_ip: u32,
  sender_port: u16,
  unix_time: u64,
  process_status: HashMap<String, bool>
}

impl HeartbeatPacket
{
  pub fn new(socket_addr: SocketAddr, process_list: &Vec<String>) -> Result<Self, Error>
  {
    let unix_time = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?;
    let sender_ip = u32::from_ne_bytes(Ipv4Addr::from_str(socket_addr.ip().to_string().as_str())?.octets());
    let sender_port = socket_addr.port();
    Ok(Self {
      sender_ip,
      sender_port,
      unix_time: unix_time.as_secs(),
      process_status: Self::fetch_process_status(&process_list)?
    })
  }

  pub fn serialize_to_json(&self) -> Result<String, Error>
  {
    Ok(serde_json::to_string(self)?)
  }

  pub fn serialize_to_binary(&self) -> Result<Vec<u8>, Error>
  {
    Ok(bincode::encode_to_vec(self, bincode::config::standard())?)
  }

  fn fetch_process_status(process_list: &Vec<String>) -> Result<HashMap<String, bool>, Error>
  {
    let process_status: HashMap<_, _> = process_list
      .iter()
      .map(|process| {
        let proc_info = ProcessInfo::from_name(process)
          .unwrap_or_default();
        (proc_info.name, proc_info.is_running)
      }).collect();
    Ok(process_status)
  }
}