use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use anyhow::{ensure, Error};
use log::info;

pub struct HeartbeatWorker
{
  socket: UdpSocket,
  interval: Duration
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

  pub fn run(&self, process_list: Vec<String>) -> Result<(), Error>
  {
    info!("starting heartbeat on {} processes", process_list.len());
    loop
    {
      self.socket.send(b"hello!")?;
      thread::sleep(self.interval);
      print!(".");
    }
    info!("stopping heartbeat");
    Ok(())
  }
}