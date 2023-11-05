pub mod amazon;
pub mod reddit;
pub mod youtube;

pub const HANDLERS: [&dyn Handler; 3] = [&amazon::AmazonHandler {}, &reddit::RedditHandler {}, &youtube::YouTubeHandler {}];

pub trait Handler {
  fn get_domain(&self) -> &'static str;
  fn handle(&self, path: &[&str]) -> Option<String>;
}
