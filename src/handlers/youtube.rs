use super::Handler;

pub struct YouTubeHandler;

impl Handler for YouTubeHandler {
  fn get_domain(&self) -> &'static str {
    "youtube.com"
  }

  fn handle(&self, path: &[&str]) -> Option<String> {
    if path.len() == 2 && path[0] == "shorts" {
      return Some(format!("https://www.youtube.com/shorts/{}", path[1].split("?").collect::<Vec<&str>>()[0]));
    }

    if path.len() != 1 || !path[0].starts_with("watch?v=") {
      return None;
    }

    let video_id = path[0].replace("watch?v=", "");

    Some(format!("https://youtu.be/{}", video_id))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_homepage() {
    let handler = YouTubeHandler {};
    let url = "https://www.youtube.com/";
    let path = &url.split("/").collect::<Vec<&str>>()[3..];

    let result = handler.handle(&path);
    assert_eq!(result, None);
  }

  #[test]
  fn test_video() {
    let handler = YouTubeHandler {};
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    let path = &url.split("/").collect::<Vec<&str>>()[3..];

    let result = handler.handle(&path);
    assert_eq!(result, Some("https://youtu.be/dQw4w9WgXcQ".to_string()));
  }

  #[test]
  fn test_short() {
    let handler = YouTubeHandler {};
    let url = "https://www.youtube.com/shorts/-ZDi3lBLHVw";
    let path = &url.split("/").collect::<Vec<&str>>()[3..];

    let result = handler.handle(&path);
    assert_eq!(result, Some("https://www.youtube.com/shorts/-ZDi3lBLHVw".to_string()));
  }
}
