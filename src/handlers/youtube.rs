use super::Handler;

pub struct YouTubeHandler;

impl Handler for YouTubeHandler {
  fn get_domain(&self) -> &'static str {
    "youtube.com"
  }

  fn handle(&self, path: &[&str]) -> Option<String> {
    if path.len() != 1 || !path[0].starts_with("watch?v=") {
      return None;
    }

    let video_id = path[0].replace("watch?v=", "");

    Some(format!("https://youtu.be/{}", video_id))
  }
}
