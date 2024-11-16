use std::{collections::HashMap, process::{Command, Output}, ptr, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

use discord_rich_presence::{activity::{self, Activity, ActivityType, Timestamps}, DiscordIpc, DiscordIpcClient};

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

  let mut string_to_field = [
    ("tag title ", &mut title),
    ("tag artist ", &mut artist),
    ("tag album ", &mut album),
    ("duration ", &mut duration_sec),
    ("position ", &mut position_sec),
  ];
  
  for line in status_str.split('\n') {
    for (line_start, field) in &mut string_to_field {
      if line.starts_with(*line_start) {
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

fn unix_epoch_secs_from_now(secs: &i64) -> i64 {
  i64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()).unwrap() + secs
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Connecting to Discord...");

  let mut client = DiscordIpcClient::new("1307075307299405844")?;

  match client.connect() {
    Ok(()) => println!("Successfully connected to Discord."),
    Err(e) => println!("Error occurred while connecting to Discord: {}", &e),
  }

  loop {
    let cmus_remote_result = Command::new("cmus-remote")
      .args(["-C", "status"])
      .output();

    match cmus_remote_result {
      Ok(Output { stdout, .. }) => {
        let CmusStatus { title, artist, album, duration_sec, position_sec } =
          parse_status_str(&String::from_utf8_lossy(&stdout));

        client.set_activity(Activity::new()
          .activity_type(ActivityType::Listening)
          .details(&title)
          .state(&artist)
          .timestamps(Timestamps::new()
            .start(unix_epoch_secs_from_now(&-position_sec.try_into()?))
            .end(unix_epoch_secs_from_now(&(duration_sec - position_sec).try_into()?)))
        )?;
      },

      Err(_) => println!("An error occurred while launching cmus-remote."),
    }

    thread::sleep(Duration::from_millis(1000));
  }

  // client.close()?;
  // Ok(())
}