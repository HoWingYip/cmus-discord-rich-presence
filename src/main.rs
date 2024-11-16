use std::{collections::HashMap, process::Command, sync::{LazyLock, Mutex}, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

use discord_rich_presence::{activity::{Activity, ActivityType, Assets, Timestamps}, DiscordIpc, DiscordIpcClient};

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

async fn get_album_art_url(
  reqwest_client: &reqwest::Client, album: &String, artist: &String,
) -> Result<String, Box<dyn std::error::Error>> {
  if album.is_empty() || artist.is_empty() {
    return Err("album and artist must not be empty".into());
  }

  static ALBUM_ART_CACHE: LazyLock<Mutex<HashMap<(String, String), String>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

  let first_artist = match artist.split_once(",") {
    Some((first_artist, _)) => first_artist,
    None => artist,
  };

  let cache_key = (first_artist.to_owned(), album.to_owned());
  match ALBUM_ART_CACHE.lock()?.get(&cache_key) {
    Some(url) => return Ok(url.to_owned()),
    None => {
      println!("Cache miss for key {:?}", cache_key);
    },
  }

  let response = reqwest_client.post("https://itunes.apple.com/search")
    .form(&[
      ("term", format!("{} {}", first_artist, album)),
      ("entity", "album".to_owned()),
    ])
    .send()
    .await?
    .json::<serde_json::Value>()
    .await?;

  let url = "https://a5.mzstatic.com/us/r1000/0/".to_string() +
    &response
      .get("results").ok_or("'results' key missing from response")?
      .as_array().ok_or("'results' field could not be converted to Vec")?
      .iter()
      .find(|v| -> bool {
        match v.get("collectionName") {
          Some(found_album) => found_album == album,
          None => false,
        }
      }).ok_or("No album found with matching name")?
      .get("artworkUrl100").ok_or("'artworkUrl100' key missing from 'results' object")?
      .to_string()
      .splitn(2, "/image/thumb")
      .collect::<Vec<&str>>()[1]
      .rsplitn(2, "/")
      .collect::<Vec<&str>>()[1]
      .to_owned();
  
  ALBUM_ART_CACHE.lock()?.insert(cache_key, url.clone());

  Ok(url)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Connecting to Discord...");

  let mut discord_ipc_client = DiscordIpcClient::new("1307075307299405844")?;
  discord_ipc_client.connect()?;

  println!("Successfully connected to Discord.");

  let reqwest_client = reqwest::Client::new();

  loop {
    let cmus_remote_result = Command::new("cmus-remote")
      .args(["-C", "status"])
      .output()?;

    match parse_status_str(&String::from_utf8_lossy(&cmus_remote_result.stdout)) {
      Ok(CmusStatus { playing, title, artist, album, duration_sec, position_sec }) => {
        if playing {
          discord_ipc_client.set_activity(Activity::new()
            .activity_type(ActivityType::Listening)
            .details(&title)
            .state(&artist)
            .timestamps(Timestamps::new()
              .start(unix_epoch_secs_from_now(&-position_sec.try_into()?))
              .end(unix_epoch_secs_from_now(&(duration_sec - position_sec).try_into()?)))
            .assets(Assets::new()
              .large_image(match &get_album_art_url(&reqwest_client, &album, &artist).await {
                Ok(album_art_url) => album_art_url,
                Err(_) => "",
              }))
          )?;
        } else {
          discord_ipc_client.clear_activity()?;
        }
      },

      Err(e) => {
        println!("An error occurred while parsing cmus-remote output: {:?}", e);
        println!("Clearing Rich Presence status.");
        discord_ipc_client.clear_activity()?;
      }
    }

    thread::sleep(Duration::from_millis(1000));
  }

  // client.close()?;
  // Ok(())
}