#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use base64::{engine, Engine};
use device_query::{DeviceQuery, DeviceState, MouseState};
use image;
use once_cell::sync::Lazy;
use rdev::{listen, Event, EventType};
use rsautogui::screen;
use scrap::{Capturer, Display};
use serde::Serialize;
use std::io::Cursor;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::{thread, time::Duration};
use tauri::Manager;

// 定义全局的 events
static EVENTS: Lazy<Arc<Mutex<Vec<Event>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
// 定义全局 截图
static PHOTO_VEC: Lazy<Arc<Mutex<Vec<screen::DynamicImage>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

static GLOBAL_DEVICE_STATE: Lazy<Arc<Mutex<DeviceState>>> =
    Lazy::new(|| Arc::new(Mutex::new(DeviceState::new())));

static GLOBAL_RUNNING: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));
const WIDTH: u16 = 100;

const HEIGHT: u16 = 100;

#[derive(Serialize, Clone)]
struct ButtonPressEventPayload {
    button: String,
    coordinates: (i32, i32),
    image: String,
}

fn callback(app_handle: &tauri::AppHandle, event: Event) {
    // println!("My callback {:?}", event);
    match event.event_type {
        EventType::ButtonPress(button) => {
            let (x, y) = GLOBAL_DEVICE_STATE.lock().unwrap().get_mouse().coords;
            println!("x: {:?}, y: {:?}", x, y);

            // 计算截图的起始坐标，确保它们不会是负数
            // let screenshot_x = x as u16 - WIDTH / 2;
            // let screenshot_y = y as u16 - HEIGHT / 2;

            let screenshot_x = std::cmp::max(0, x as i32 - WIDTH as i32 / 2) as u16;
            let screenshot_y = std::cmp::max(0, y as i32 - HEIGHT as i32 / 2) as u16;

            println!(
                "screenshot_x: {:?}, screenshot_y: {:?}",
                screenshot_x, screenshot_y
            );
            let screen_width = screen::size().0;
            let screen_height = screen::size().1;
            // 计算截图的起始坐标，确保它们不会是负数
            let screenshot_x = std::cmp::max(0, x as i32 - WIDTH as i32 / 2) as u16;
            let screenshot_y = std::cmp::max(0, y as i32 - HEIGHT as i32 / 2) as u16;

            // 计算截图的结束坐标
            let mut end_x = screenshot_x + WIDTH;
            let mut end_y = screenshot_y + HEIGHT;

            // 确保截图的结束坐标不会超出屏幕尺寸
            if end_x > screen_width {
                end_x = screen_width;
            }
            if end_y > screen_height {
                end_y = screen_height;
            }

            // 根据屏幕尺寸调整截图的宽度和高度
            let adjusted_width = end_x - screenshot_x;
            let adjusted_height = end_y - screenshot_y;
            println!("{:?}", adjusted_height);
            println!("{:?}", adjusted_width);

            let ss =
                screen::screenshot(screenshot_x, screenshot_y, adjusted_width, adjusted_height);
            // 将ss转换为PNG格式的字节流
            let mut buffer = Cursor::new(Vec::new());
            ss.write_to(&mut buffer, image::ImageOutputFormat::Png)
                .unwrap();
            let image_data = buffer.into_inner();
            let base64_str = format!(
                "data:image/png;base64,{}",
                engine::general_purpose::STANDARD.encode(&image_data)
            );

            let button_press_event_payload = ButtonPressEventPayload {
                button: format!("{:?}", button),
                coordinates: (x, y),
                image: base64_str,
            };
            // println!("User pressed {:?}", button);

            app_handle
                .emit_all("button-press", button_press_event_payload)
                .unwrap();
        }
        _ => (),
    }
}
#[tauri::command]
fn start_recording() {
    println!("start recording");
    GLOBAL_RUNNING.store(true, Ordering::SeqCst);
}

#[tauri::command]
fn stop_recording() {
    GLOBAL_RUNNING.store(false, Ordering::SeqCst);
    println!("stop recording");
}

#[tauri::command]
fn show_events(window: tauri::Window) {
    let events_clone = EVENTS.clone();
    println!("events_clone: {:?}", events_clone);
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            let global_running_clone = GLOBAL_RUNNING.clone();
            // global_running_clone.store(true, Ordering::SeqCst);
            std::thread::spawn(move || {
                if let Err(error) = listen(move |event| {
                    if !global_running_clone.load(Ordering::SeqCst) {
                        // 如果 GLOBAL_RUNNING 被设置为 false，立即退出监听
                        return;
                    }
                    // 假设 callback 是一个已定义的函数，用于处理监听到的事件
                    callback(&app_handle, event);
                }) {
                    println!("Error: {:?}", error)
                }
            });
            // 这里可以返回 Ok(()) 表示成功初始化，或 Err(e) 来表示初始化失败
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_recording,
            show_events
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
