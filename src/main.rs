#![windows_subsystem = "windows"]

use std::{thread::sleep, time::Duration};

use clipboard_win::{formats, get_clipboard, set_clipboard};

fn main() {
  loop {
    sleep(Duration::from_secs(1));

    let clipboard: String = get_clipboard(formats::Unicode).expect("Failed to get clipboard contents");

    if clipboard.is_empty() {
      continue;
    }

    let url_parts = clipboard.split("/").collect::<Vec<&str>>();

    if url_parts.len() < 3 {
      continue;
    }

    let domain = url_parts[2].replace("www.", "");
    let path = &url_parts[3..];

    let new_clipboard = match domain.as_str() {
      "amazon.com" => {
        if url_parts[3] == "dp" {
          let product_id = url_parts[4].split("?").collect::<Vec<&str>>()[0];

          format!("https://www.amazon.com/dp/{}", product_id)
        } else {
          let product_id_index = url_parts.iter().position(|&r| r == "dp").unwrap() + 1;
          let product_id = url_parts[product_id_index].to_string().split("?").collect::<Vec<&str>>()[0].to_string();

          format!("https://www.amazon.com/dp/{}", product_id)
        }
      }
      "youtube.com" => {
        if url_parts.len() != 4 || !url_parts[3].starts_with("watch?v=") {
          continue;
        }

        let video_id = url_parts[3].replace("watch?v=", "");

        format!("https://youtu.be/{}", video_id)
      }
      "reddit.com" => {
        if path.len() <= 4 || path[0] != "r" || path[2] != "comments" || (path.len() == 6 && path[4] == "comment") {
          continue;
        }

        if path.len() >= 6 && path[4] == "comment" {
          format!("https://www.reddit.com/r/{}/comments/{}/comment/{}", path[1], path[3], path[5])
        } else {
          format!("https://www.reddit.com/r/{}/comments/{}", path[1], path[3])
        }
      }
      _ => "".to_string(),
    };

    if new_clipboard.is_empty() {
      continue;
    }

    set_clipboard(formats::Unicode, &new_clipboard).expect("Failed to set clipboard contents");
    println!("Copied {} to clipboard", new_clipboard);
  }
}
