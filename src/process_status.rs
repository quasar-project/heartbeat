use std::sync::Mutex;
use anyhow::Error;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProcessInfo
{
  pub name: String,
  pub is_running: bool
}

impl ProcessInfo
{
  pub fn from_name(name: &str) -> Result<Self, Error>
  {
    get_process_info(name)
  }
}

fn get_process_info(name: &str) -> Result<ProcessInfo, Error>
{
  let mut s = SYSTEM.lock().unwrap();
  s.refresh_processes();
  let is_running = s.processes_by_name(name).count() > 0;
  Ok(ProcessInfo {
    name: name.to_string(),
    is_running
  })
}

lazy_static!
{
  static ref SYSTEM: Mutex<sysinfo::System> = Mutex::new(sysinfo::System::new());
}