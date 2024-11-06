extern crate image;
extern crate winapi;

use std::fmt::Write;
// use std::ptr::null_mut;
// use winapi::um::winuser::{GetDC, ReleaseDC};
// use winapi::um::wingdi::{CreateCompatibleDC, CreateCompatibleBitmap, SelectObject, BitBlt, DeleteDC, DeleteObject, SRCCOPY};
// use winapi::um::wingdi::{GetDIBits, BITMAPINFOHEADER, BITMAPINFO, BI_RGB};
// use winapi::um::winnt::HANDLE;
// use winapi::shared::windef::{HWND, HDC, HBITMAP, RECT};
// use winapi::um::winuser::{GetDesktopWindow, GetWindowRect};
// use image::{ImageBuffer, Rgba};

use image::jpeg::{JpegDecoder, JpegEncoder};
use serde::de::Expected;
use tauri::ipc::IpcResponse;
use tauri::{window, Emitter, Listener};
use winapi::um::winuser::WINDOWINFO;
// use tauri::App;
// use tauri::Window;
//跨平台窗口创建和管理
use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use winit::event_loop::EventLoopBuilder;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;

//截屏
use screenshots::Screen;
use std::time::Instant;

use image::codecs::jpeg::JPEGEncoder;
use image::codecs::png::PngEncoder;

use base64::engine::general_purpose;
use base64::{encode, Engine};
use image::ColorType;

use chrono::Local;

#[derive(Default)]
struct NewScreenShutWindow {
    window: Option<Window>,
}

impl ApplicationHandler for NewScreenShutWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.window.as_ref().unwrap();
        match event {
            WindowEvent::CursorLeft { device_id } => {
                println!("window got clicked!");
            }
            _ => (),
        }
    }
}

pub fn draw_select_window(handle: tauri::AppHandle) -> Result<(), tauri::Error> {
    let new_window = tauri::WebviewWindowBuilder::new(
        &handle,
        "screenShotWindow",
        tauri::WebviewUrl::App("transparent_page.html".into()),
    )
    .title("screen_shot")
    .inner_size(2560.0, 1440.0)
    .fullscreen(true)
    .transparent(true)
    .build()?;

    let _ = new_window.set_decorations(false);
    // new_window.listen(event, handler)

    //隐藏其他窗口
    new_window.emit("screenShotOpen", ()).unwrap();

    let new_window_clone1 = new_window.clone();
    new_window.once("get_screenshot", move |event| {
        
        new_window_clone1.unlisten(event.id());
        new_window_clone1.hide().unwrap();

        prepare_to_getScreenShot(&new_window_clone1);
        println!("截屏完成 恢复窗口");
        new_window_clone1.show().unwrap();

        // let new_window_clone11=new_window_clone1.clone();
        // new_window_clone11.listen("second_window_hide",move |event|{
        //     new_window_clone1.unlisten(event.id());
        //     new_window_clone1.hide().unwrap();

        //     prepare_to_getScreenShot(&new_window_clone1);
        //     println!("截屏完成 恢复窗口");
        //     new_window_clone1.show().unwrap();
        // });
        
    });

    let new_window_clone = new_window.clone();
    new_window.on_window_event(move |event| {
        handle_shutWindow_event(&new_window_clone, event);
    });

    let new_window_clone2 = new_window.clone();
    new_window.once("exit_screenShot", move |event| {
        println!("截图窗口关闭!");
        new_window_clone2.unlisten(event.id());
        // new_window_clone2.clear_all_browsing_data();
        // let _=new_window_clone2.close();
        //显示其他窗口
        new_window_clone2.emit("screenShotClose", ()).unwrap();
        let _ = new_window_clone2.destroy();
    });

    Ok(())
}

fn handle_shutWindow_event(window: &tauri::WebviewWindow, event: &tauri::WindowEvent) {
    // println!("into window event");

    // println!("{:?}",event);

    // if window.label()=="new_window" {
    match event {
        tauri::WindowEvent::Resized(size) => {
            println!(" {:?},{:?} ", size.height, size.width);
        }
        tauri::WindowEvent::Focused(state) => {
            println!("截屏窗口focus {}", state);
        }
        _ => {}
    }
}

fn prepare_to_getScreenShot_new(window: &tauri::WebviewWindow) {
    println!("Rust收到截图请求!");
    match Screen::from_point(0, 0) {
        Ok(screen) => {
            if let Ok(image) = screen.capture() {
                println!("screen.capture()完成!");

                // 直接使用 RGBA8 数据，不进行 JPEG 编码或 Base64 转换
                let raw_image_data = image.to_vec();

                println!("发送截图到JS!");
                window.emit("screenshot-captured", raw_image_data).unwrap();
                println!("发送到JS!完毕");
            }
        }
        Err(e) => {
            eprintln!("Failed to capture screenshot: {}", e);
        }
    }
}

fn prepare_to_getScreenShot(window: &tauri::WebviewWindow) {
    //收到js发来的截屏请求
    println!("rust收到截图请求!");
    let screens = Screen::all().unwrap();
    for screen in screens {
        println!("{}", screen.display_info.id);
    }
    match Screen::from_point(0, 0) {
        Ok(screen) => {
            if let Ok(image) = screen.capture() {
                // let now_time=Local::now();
                // let mut savepath=String::new();
                // write!(&mut savepath,"target/capture{}.png",now_time.format("%Y%m%d%M%S")).unwrap();
                // image.save(&savepath).unwrap();

                println!("screen.capture()完成!");
                let mut buffer = Vec::new();
                let mut encoder = JpegEncoder::new(&mut buffer);

                encoder
                    .encode(&image, image.width(), image.height(), ColorType::Rgba8)
                    .unwrap();
                // encoder.encode(&image, image.width(), image.height()).unwrap();

                println!("screen.encode()完成!");
                let base64_image = general_purpose::STANDARD.encode(&buffer);
                println!("发送截图到js!");

                window.emit("screenshot-captured", base64_image).unwrap();
            }
        }
        Err(e) => {
            eprintln!("Failed to capture screenshot: {}", e);
        }
    }
}

#[tauri::command]
pub async fn capture_and_emit_screenshot(event: tauri::Event) {}

pub fn get_screenshot() {
    let start = Instant::now();
    let screens = Screen::all().unwrap();

    for screen in screens {
        println!("capturer {screen:?}");
        let mut image = screen.capture().unwrap();
        image
            .save(format!("target/{}.png", screen.display_info.id))
            .unwrap();

        image = screen.capture_area(300, 300, 300, 300).unwrap();
        image
            .save(format!("target/{}-2.png", screen.display_info.id))
            .unwrap();
    }

    let screen = Screen::from_point(100, 100).unwrap();
    println!("capturer {screen:?}");

    let image = screen.capture_area(300, 300, 300, 300).unwrap();
    image.save("target/capture_display_with_point.png").unwrap();
    println!("运行耗时: {:?}", start.elapsed());
}
