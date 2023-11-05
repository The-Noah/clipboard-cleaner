#![windows_subsystem = "windows"]

use std::{thread::sleep, time::Duration};

use clipboard_win::{formats, get_clipboard, set_clipboard};

mod handlers;

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

    let mut new_clipboard = None;

    for handler in handlers::HANDLERS {
      if handler.get_domain() == domain {
        println!("Running handler for {}", domain);

        new_clipboard = handler.handle(path);
        break;
      }
    }

    if new_clipboard.is_none() {
      continue;
    }

    let new_clipboard = new_clipboard.unwrap();

    if new_clipboard == clipboard {
      continue;
    }

    set_clipboard(formats::Unicode, &new_clipboard).expect("Failed to set clipboard contents");
    println!("Copied {} to clipboard", new_clipboard);
  }
}
