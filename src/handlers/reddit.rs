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
