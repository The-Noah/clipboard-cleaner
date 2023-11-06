use super::Handler;

pub struct RedditHandler;

impl Handler for RedditHandler {
  fn get_domain(&self) -> &'static str {
    "reddit.com"
  }

  fn handle(&self, path: &[&str]) -> Option<String> {
    if path.len() <= 4 || path[0] != "r" || path[2] != "comments" || (path.len() == 6 && path[4] == "comment") {
      return None;
    }

    Some(if path.len() >= 6 && path[4] == "comment" {
      format!("https://www.reddit.com/r/{}/comments/{}/comment/{}", path[1], path[3], path[5])
    } else {
      format!("https://www.reddit.com/r/{}/comments/{}", path[1], path[3])
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_homepage() {
    let handler = RedditHandler {};
    let url = "https://www.reddit.com/";
    let path = &url.split("/").collect::<Vec<&str>>()[3..];

    let result = handler.handle(&path);
    assert_eq!(result, None);
  }

  #[test]
  fn test_subreddit() {
    let handler = RedditHandler {};
    let url = "https://www.reddit.com/r/ProgrammerHumor/";
    let path = &url.split("/").collect::<Vec<&str>>()[3..];

    let result = handler.handle(&path);
    assert_eq!(result, None);
  }

  #[test]
  fn test_post() {
    let handler = RedditHandler {};
    let url = "https://www.reddit.com/r/ProgrammerHumor/comments/17oip80/thisismyworstdream/?utm_source=share&utm_medium=web2x&context=3";
    let path = &url.split("/").collect::<Vec<&str>>()[3..];

    let result = handler.handle(&path);
    assert_eq!(result, Some("https://www.reddit.com/r/ProgrammerHumor/comments/17oip80".to_string()));
  }

  #[test]
  fn test_comment() {
    let handler = RedditHandler {};
    let url = "https://www.reddit.com/r/ProgrammerHumor/comments/17oip80/comment/k7yph09/?utm_source=share&utm_medium=web2x&context=3";
    let path = &url.split("/").collect::<Vec<&str>>()[3..];

    let result = handler.handle(&path);
    assert_eq!(result, Some("https://www.reddit.com/r/ProgrammerHumor/comments/17oip80/comment/k7yph09".to_string()));
  }
}
