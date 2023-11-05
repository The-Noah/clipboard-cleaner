use super::Handler;

pub struct AmazonHandler;

impl Handler for AmazonHandler {
  fn get_domain(&self) -> &'static str {
    "amazon.com"
  }

  fn handle(&self, path: &[&str]) -> Option<String> {
    Some(if path[0] == "dp" {
      let product_id = path[1].split("?").collect::<Vec<&str>>()[0];

      format!("https://www.amazon.com/dp/{}", product_id)
    } else {
      let product_id_index = path.iter().position(|&r| r == "dp").unwrap() + 1;
      let product_id = path[product_id_index].to_string().split("?").collect::<Vec<&str>>()[0].to_string();

      format!("https://www.amazon.com/dp/{}", product_id)
    })
  }
}
