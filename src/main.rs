#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unused)]

use std::{
  path::Path,
  rc::Rc,
  sync::{Arc, Mutex},
  thread::{self, sleep},
  time::Duration,
};

use clipboard_win::{formats, get_clipboard, set_clipboard};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
  menu::{Menu, MenuEvent, MenuItem},
  TrayIconBuilder, TrayIconEvent,
};

mod handlers;

fn main() {
  let event_loop = EventLoopBuilder::new().build();

  let tray_menu = Menu::new();
  let quit = MenuItem::new("Quit", true, None);
  tray_menu.append_items(&[&quit]).unwrap();

  let mut tray_icon = Some(
    TrayIconBuilder::new()
      .with_menu(Box::new(tray_menu))
      .with_title("Clipboard Cleaner")
      .with_tooltip("Clipboard Cleaner")
      .with_icon(load_icon(&Path::new(env!("CARGO_MANIFEST_DIR")).join("icon.png")))
      .build()
      .unwrap(),
  );

  let menu_channel = MenuEvent::receiver();
  let tray_channel = TrayIconEvent::receiver();

  let worker_running = Arc::new(Mutex::new(true));
  let worker_running_clone = worker_running.clone();

  let worker_thread = thread::spawn(move || loop {
    if !*worker_running_clone.lock().unwrap() {
      break;
    }

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
  });

  event_loop.run(move |_event, _, control_flow| {
    *control_flow = ControlFlow::Poll;

    if let Ok(event) = menu_channel.try_recv() {
      if event.id == quit.id() {
        *worker_running.lock().unwrap() = false;

        tray_icon.take();
        *control_flow = ControlFlow::Exit;
      }

      println!("{event:?}");
    }

    if let Ok(event) = tray_channel.try_recv() {
      println!("{event:?}");
    }
  });

  worker_thread.join().unwrap();
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
  let (icon_rgba, icon_width, icon_height) = {
    let image = image::open(path).expect("Failed to open icon path").into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };

  tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
