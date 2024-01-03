use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use anyhow::{
  ensure,
  Error
};
use log::info;
use crate::HeartbeatPacket;
use crate::packet::{
  HEARTBEAT_PACKET_END_MARKER,
  HEARTBEAT_PACKET_START_MARKER
};

pub struct HeartbeatWorker
{
  socket: UdpSocket,
  interval: Duration
}

#[derive(Debug, Default)]
pub enum HeartbeatPacketMode
{
  JsonString,
  #[default] BincodePacket
}

impl HeartbeatWorker
{
  pub fn new(port: u16, broadcast: bool, address: Option<&str>, interval: Duration)
    -> Result<Self, Error>
  {
    ensure!(interval > Duration::from_millis(0), "interval must be greater than 0");

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(broadcast)?;
    socket.connect(format!("{}:{}",
                           address.unwrap_or_else(|| "255.255.255.255"),
                           port
    ))?;

    info!("heartbeat initialized on address {}:{}", address.unwrap_or_else(|| "0.0.0.0"), port);
    info!("broadcast mode: {}", if broadcast { "enabled" } else { "disabled" });
    info!("interval: {}ms", interval.as_millis());
    Ok(Self {
      socket,
      interval
    })
  }

  #[allow(unreachable_code)]
  pub fn run(&self, process_list: Vec<String>, mode: HeartbeatPacketMode) -> Result<(), Error>
  {
    info!("starting heartbeat on {} processes", process_list.len());
    loop
    {
      let packet = HeartbeatPacket::new(
        self.socket.local_addr()?,
        &process_list
      );
      match mode {
        HeartbeatPacketMode::JsonString => {
          let buf = packet?.serialize_to_json()?;
          self.socket.send(buf.as_bytes())?;
        },
        HeartbeatPacketMode::BincodePacket => {
          let buf = packet?.serialize_to_binary()?;
          let buf_with_markers = [
            &[HEARTBEAT_PACKET_START_MARKER],
            buf.as_slice(),
            &[HEARTBEAT_PACKET_END_MARKER]
          ].concat();
          self.socket.send(&buf_with_markers)?;
        }
      };
      thread::sleep(self.interval);
    }
    info!("stopping heartbeat");
    Ok(())
  }
}