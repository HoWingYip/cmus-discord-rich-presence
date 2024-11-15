use std::{collections::HashMap, process::{Command, Output}, ptr};

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

#[derive(Debug)]
struct CmusStatus {
  title: String,
  artist: String,
  album: String,
  duration_sec: u32,
  position_sec: u32,
}

fn parse_status_str(status_str: &str) -> CmusStatus {
  let mut title: String = Default::default();
  let mut artist: String = Default::default();
  let mut album: String = Default::default();
  let mut duration_sec: String = Default::default();
  let mut position_sec: String = Default::default();

  let mut string_to_field = HashMap::from([
    ("tag title ", &mut title),
    ("tag artist ", &mut artist),
    ("tag album ", &mut album),
    ("duration ", &mut duration_sec),
    ("position ", &mut position_sec),
  ]);
  
  for line in status_str.split('\n') {
    for (line_start, field) in &mut string_to_field {
      if line.starts_with(line_start) {
        field.push_str(&line[line_start.len()..]);
      }
    }
  }

  CmusStatus {
    title, artist, album,
    duration_sec: duration_sec.parse().unwrap(),
    position_sec: position_sec.parse().unwrap(),
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // let mut client = DiscordIpcClient::new("1307075307299405844")?;
  // client.connect()?;

  let cmus_status = Command::new("cmus-remote")
    .args(["-C", "status"])
    .output();

  match cmus_status {
    Ok(Output { stdout, .. }) => {
      println!("{:?}", parse_status_str(&String::from_utf8_lossy(stdout.as_ref())));
    },
    Err(_) => println!("An error occurred."),
  }

  // client.set_activity(activity::Activity::new()
  //   .state("foo")
  //   .details("bar")
  // )?;
  // client.close()?;

  Ok(())
}