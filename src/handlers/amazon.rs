use super::Handler;

pub struct AmazonHandler;

impl Handler for AmazonHandler {
  fn get_domain(&self) -> &'static str {
    "amazon.com"
  }

  fn handle(&self, path: &[&str]) -> Option<String> {
    if path.len() == 0 || path[0] != "dp" && path.iter().position(|&r| r == "dp").is_none() {
      return None;
    }

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_homepage() {
    let handler = AmazonHandler {};
    let url = "https://www.amazon.com/";
    let path = &url.split("/").collect::<Vec<&str>>()[3..];

    let result = handler.handle(&path);
    assert_eq!(result, None);
  }

  #[test]
  fn test_product() {
    let handler = AmazonHandler {};
    let url = "https://www.amazon.com/dp/B0C658RJL9?ref_=ast_sto_dp";
    let path = &url.split("/").collect::<Vec<&str>>()[3..];

    let result = handler.handle(&path);
    assert_eq!(result, Some("https://www.amazon.com/dp/B0C658RJL9".to_string()));
  }

  #[test]
  fn test_product_with_name() {
    let handler = AmazonHandler {};
    let url = "https://www.amazon.com/Taco-Cat-Goat-Cheese-Pizza/dp/B0C658RJL9?ref_=ast_sto_dp";
    let path = &url.split("/").collect::<Vec<&str>>()[3..];

    let result = handler.handle(&path);
    assert_eq!(result, Some("https://www.amazon.com/dp/B0C658RJL9".to_string()));
  }
}
