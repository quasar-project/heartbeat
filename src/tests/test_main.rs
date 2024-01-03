#[cfg(test)]
mod tests {
  use std::time::Duration;
  use crate::HeartbeatWorker;

  #[test]
  fn test_main()
  {
    println!("➡️ starting {}...", env!("CARGO_PKG_NAME"));
    simple_logger::init_with_level(log::Level::Info)
      .expect("❌ failed to initialize logger");
    let result = HeartbeatWorker::new(
      4550,
      true,
      None,
      Duration::from_millis(1000)
    ).expect("❌ failed to create worker")
      .run(vec![
        String::from("notepad.exe"),
        String::from("explorer.exe"),
        String::from("calc.exe"),
        String::from("taskmgr.exe"),
      ])
      .expect("❌ failed to run worker");
    println!("☑️ {} finished running!", env!("CARGO_PKG_NAME"));
  }
}