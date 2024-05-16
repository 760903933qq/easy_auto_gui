#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use base64::{engine, Engine};
use image;
use once_cell::sync::Lazy;
use rdev::{listen, Event, EventType};
use rsautogui::screen;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
// 定义全局的 events
static EVENTS: Lazy<Arc<Mutex<Vec<Event>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
// 定义全局 截图
static PHOTO_VEC: Lazy<Arc<Mutex<Vec<screen::DynamicImage>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

const WIDTH: u16 = 300;
const HEIGHT: u16 = 300;
fn callback(window: tauri::Window, event: Event) {
    println!("My callback {:?}", event);
    match event.event_type {
        EventType::ButtonPress(button) => {
            println!("User pressed {:?}", button);
        }
        _ => (),
    }
    let events_clone = EVENTS.clone();
    let mut events_lock = events_clone.lock().unwrap();
    events_lock.push(event.clone());
    match event.event_type {
        EventType::ButtonPress(button) => {
            println!("User pressed {:?}", button);
            let last_move_event = &events_lock[events_lock.len() - 2];

            if let EventType::MouseMove { x, y } = last_move_event.event_type {
                let half_width = WIDTH / 2;
                let half_height = HEIGHT / 2;
                let screenshot_x = (x - half_width as f64) as u16;
                let screenshot_y = (y - half_height as f64) as u16;
                let ss = screen::screenshot(screenshot_x, screenshot_y, WIDTH, HEIGHT);
                // 将ss转换为PNG格式的字节流
                let mut buffer = Cursor::new(Vec::new());
                ss.write_to(&mut buffer, image::ImageOutputFormat::Png)
                    .unwrap();
                let image_data = buffer.into_inner();
                let base64_str = engine::general_purpose::STANDARD.encode(&image_data);

                println!("base64_str: {:?}", base64_str);
                window.emit("image", base64_str).unwrap();
            } else {
                println!("last_move_event is not EventType::MouseMove");
            };
        }
        _ => (),
    }
}
#[tauri::command]
fn listen_win(window: tauri::Window) {
    let window_clone = window.clone();
    thread::spawn(move || {
        if let Err(error) = listen(move |event| callback(window_clone.clone(), event)) {
            println!("Error: {:?}", error)
        }
    });
}

#[tauri::command]
fn show_events(window: tauri::Window) {
    let events_clone = EVENTS.clone();
    println!("events_clone: {:?}", events_clone);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![listen_win, show_events])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
