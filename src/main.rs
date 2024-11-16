use std::{process::Command, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

use discord_rich_presence::{activity::{Activity, ActivityType, Timestamps}, DiscordIpc, DiscordIpcClient};

#[derive(Debug)]
struct CmusStatus {
  playing: bool,
  title: String,
  artist: String,
  album: String,
  duration_sec: u32,
  position_sec: u32,
}

fn parse_status_str(status_str: &str) -> Result<CmusStatus, Box<dyn std::error::Error>> {
  let mut status: String = Default::default();
  let mut title: String = Default::default();
  let mut artist: String = Default::default();
  let mut album: String = Default::default();
  let mut duration_sec: String = Default::default();
  let mut position_sec: String = Default::default();

  let mut string_to_field = [
    ("status ", &mut status),
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

  Ok(CmusStatus {
    playing: status == "playing",
    title, artist, album,
    duration_sec: duration_sec.parse()?,
    position_sec: position_sec.parse()?,
  })
}

fn unix_epoch_secs_from_now(secs: &i64) -> i64 {
  i64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()).unwrap() + secs
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Connecting to Discord...");

  let mut client = DiscordIpcClient::new("1307075307299405844")?;
  client.connect()?;

  println!("Successfully connected to Discord.");

  loop {
    let cmus_remote_result = Command::new("cmus-remote")
      .args(["-C", "status"])
      .output()?;

    match parse_status_str(&String::from_utf8_lossy(&cmus_remote_result.stdout)) {
      Ok(CmusStatus { playing, title, artist, album, duration_sec, position_sec }) => {
        // TODO: add album art using iTunes API
        // https://github.com/bendodson/itunes-artwork-finder/blob/master/api.php
        // Leave album art blank if album is empty.

        if playing {
          client.set_activity(Activity::new()
            .activity_type(ActivityType::Listening)
            .details(&title)
            .state(&artist)
            .timestamps(Timestamps::new()
              .start(unix_epoch_secs_from_now(&-position_sec.try_into()?))
              .end(unix_epoch_secs_from_now(&(duration_sec - position_sec).try_into()?)))
          )?;
        } else {
          client.clear_activity()?;
        }
      },

      Err(e) => {
        println!("An error occurred while parsing cmus-remote output: {:?}", e);
        println!("Clearing Rich Presence status.");
        client.clear_activity()?;
      }
    }

    thread::sleep(Duration::from_millis(1000));
  }

  // client.close()?;
  // Ok(())
}