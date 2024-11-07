use std::ptr::null_mut;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBITMAP, HDC, HWND, POINT, RECT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::*;

use winapi::um::winuser::*;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

static mut START_POINT: POINT = POINT { x: 0, y: 0 };
static mut END_POINT: POINT = POINT { x: 0, y: 0 };
static mut IS_DRAWING: bool = false;
static mut EIXT_CALLED: bool = false;

// 声明退出标志的 Arc<AtomicBool>
static mut EXIT_FLAG: Option<Arc<AtomicBool>> = None;

//捕获的桌面图片
static mut DESKTOP_BITMAP: Option<HBITMAP> = None;


unsafe fn cleanup_desktop_bitmap() {
    if let Some(bitmap) = DESKTOP_BITMAP {
        DeleteObject(bitmap as _); // 释放创建的位图
        DESKTOP_BITMAP = None;
    }
}

fn capture_desktop() -> HBITMAP {
    unsafe {
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);

        // Capture the desktop
        let hdc_screen = GetDC(null_mut());
        let hdc_mem = CreateCompatibleDC(hdc_screen);
        let hbitmap = CreateCompatibleBitmap(hdc_screen, screen_width, screen_height);
        let old_bitmap = SelectObject(hdc_mem, hbitmap as _);

        // Copy the screen image into the memory device context
        BitBlt(
            hdc_mem,
            0,
            0,
            screen_width,
            screen_height,
            hdc_screen,
            0,
            0,
            SRCCOPY,
        );

        // Clean up
        SelectObject(hdc_mem, old_bitmap);
        ReleaseDC(null_mut(), hdc_screen);

        println!("保存当前截图");
        hbitmap
    }
}

fn draw_desktop_static_pic(hdc: HDC, width: i32, height: i32) {
    unsafe {
        let memDC = CreateCompatibleDC(hdc);
        let oldBitmap = SelectObject(memDC, DESKTOP_BITMAP.unwrap() as _);

        BitBlt(hdc, 0, 0, width, height, memDC, 0, 0, SRCCOPY);

        SelectObject(memDC, oldBitmap);

        DeleteDC(memDC);
    }
}

fn draw_mask(hdc: HDC, width: i32, height: i32, rect: RECT) {
    unsafe {
        // let memDC = CreateCompatibleDC(hdc);
        // let oldBitmap = SelectObject(memDC, DESKTOP_BITMAP.unwrap() as _);
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);

        ExcludeClipRect(
            hdc,
            START_POINT.x.min(END_POINT.x),
            START_POINT.y.min(END_POINT.y),
            START_POINT.x.max(END_POINT.x),
            START_POINT.y.max(END_POINT.y),
        );
        println!("绘制遮罩 {} {}",width,height);
        let overlay_brush = CreateSolidBrush(RGB(0, 0, 128) | (32 << 24)); // 黑色半透明

        FillRect(hdc, &rect, overlay_brush);

        TransparentBlt(hdcDest, xoriginDest, yoriginDest, wDest, hDest, hdcSrc, xoriginSrc, yoriginSrc, wSrc, hSrc, crTransparent)
        // BitBlt(hdc, 0, 0, screen_width, screen_height, memDC, 0, 0, SRCCOPY);

        SelectClipRgn(hdc, null_mut());
        // SelectObject(memDC, oldBitmap);

        DeleteObject(overlay_brush as _);
        // DeleteDC(memDC);
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_KEYDOWN => {
            // 检测按下了哪个键
            match w_param as i32 {
                VK_ESCAPE => {
                    EIXT_CALLED = true;
                    if let Some(exit_flag) = &EXIT_FLAG {
                        exit_flag.store(true, Ordering::Relaxed);
                    }
                    println!("Escape key pressed");
                }
                VK_RETURN => {
                    println!("Enter key pressed");
                }
                VK_SPACE => {
                    println!("Space key pressed");
                }
                // 可以继续添加其他按键的处理
                _ => {
                    // 如果是其他按键，打印按键的虚拟键码
                    println!("Key pressed: {}", w_param);
                }
            }

            0
        }

        WM_LBUTTONDOWN => {
            IS_DRAWING = true;
            START_POINT.x = GET_X_LPARAM(l_param);
            START_POINT.y = GET_Y_LPARAM(l_param);
            InvalidateRect(hwnd, null_mut(), 1 as i32);
            println!("mousedown");
            0
        }
        WM_MOUSEMOVE => {
            // println!("mousemove");
            if IS_DRAWING {
                END_POINT.x = GET_X_LPARAM(l_param);
                END_POINT.y = GET_Y_LPARAM(l_param);
                InvalidateRect(hwnd, null_mut(), 1 as i32);
            }

            0
        }
        WM_LBUTTONUP => {
            IS_DRAWING = false;
            END_POINT.x = GET_X_LPARAM(l_param);
            END_POINT.y = GET_Y_LPARAM(l_param);
            InvalidateRect(hwnd, null_mut(), 1 as i32);
            0
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT {
                ..std::mem::zeroed()
            };
            let hdc = BeginPaint(hwnd, &mut ps);

            // 获取窗口大小
            let mut rect: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut rect);

            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            draw_desktop_static_pic(hdc, width, height);

            // if IS_DRAWING{
            draw_mask(hdc, width, height, rect);
            // }
           
            // 创建内存 DC

            // let mem_dc = CreateCompatibleDC(hdc);
            // // let mem_bitmap = CreateCompatibleBitmap(hdc, rect.right, rect.bottom);

            // let old_bitmap = SelectObject(mem_dc, DESKTOP_BITMAP.unwrap() as _);

            // // 将内存 DC 内容复制到窗口
            // BitBlt(hdc, 0, 0, width, height, mem_dc, 0, 0, SRCCOPY);

            // SelectObject(mem_dc, old_bitmap);
            // // 排除选择框区域（设置剪裁）
            // if IS_DRAWING {
            //     ExcludeClipRect(
            //         hdc,
            //         START_POINT.x.min(END_POINT.x),
            //         START_POINT.y.min(END_POINT.y),
            //         START_POINT.x.max(END_POINT.x),
            //         START_POINT.y.max(END_POINT.y),
            //     );
            // }

            // // 填充半透明背景（只填充未排除的区域）
            // let overlay_brush = CreateSolidBrush(RGB(0, 0, 50) | (128 << 24)); // 黑色半透明
            // FillRect(hdc, &rect, overlay_brush);

            // // 恢复剪裁区域
            // SelectClipRgn(hdc, null_mut());

            // // 绘制绿色选择框
            if IS_DRAWING {
                let pen = CreatePen(PS_SOLID as i32, 2, RGB(0, 255, 0)); // 绿色边框
                let old_pen = SelectObject(hdc, pen as _);
                let hollow_brush = GetStockObject(HOLLOW_BRUSH as _);
                let old_brush = SelectObject(hdc, hollow_brush);

                Rectangle(
                    hdc,
                    START_POINT.x.min(END_POINT.x),
                    START_POINT.y.min(END_POINT.y),
                    START_POINT.x.max(END_POINT.x),
                    START_POINT.y.max(END_POINT.y),
                );

                SelectObject(hdc, old_pen);
                SelectObject(hdc, old_brush);
                DeleteObject(pen as _);
            }

            // // 清理
            // DeleteDC(mem_dc);
            // DeleteObject(overlay_brush as _);

            EndPaint(hwnd, &ps);
            0
        }
        WM_DESTROY => {
            cleanup_desktop_bitmap();
            START_POINT.x=0;
            START_POINT.y=0;
            END_POINT.x=0;
            END_POINT.y=0;
            IS_DRAWING=false;
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

pub fn create_screenShotWindow() -> bool {
    unsafe {
        let h_instance = GetModuleHandleW(null_mut());

        // 创建窗口类
        let class_name = wide_null("TransparentWindowClass");
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance,
            lpszClassName: class_name.as_ptr(),
            ..std::mem::zeroed()
        };

        RegisterClassW(&wc);

        // 创建全屏透明窗口
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOPMOST,
            class_name.as_ptr(),
            wide_null("Transparent Window").as_ptr(),
            WS_POPUP,
            0,
            0,
            GetSystemMetrics(SM_CXSCREEN),
            GetSystemMetrics(SM_CYSCREEN),
            null_mut(),
            null_mut(),
            h_instance,
            null_mut(),
        );

        // 设置窗口为透明
        SetLayeredWindowAttributes(hwnd, 0, 128, LWA_ALPHA);

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        // 消息循环
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            if EIXT_CALLED {
                return true;
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        return true;
    }
}

pub fn start_screenshot_thread() -> Arc<AtomicBool> {
    let exit_flag = Arc::new(AtomicBool::new(false));
    let exit_flag_clone = Arc::clone(&exit_flag);

    thread::spawn(move || {
        unsafe {
            //捕获快照
            DESKTOP_BITMAP = Some(capture_desktop());

            EXIT_FLAG = Some(exit_flag_clone.clone());

            let h_instance = GetModuleHandleW(null_mut());

            // 定义窗口类
            let class_name = wide_null("TransparentWindowClass");
            let wc = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                hInstance: h_instance,
                lpszClassName: class_name.as_ptr(),
                ..std::mem::zeroed()
            };

            RegisterClassW(&wc);

            // 创建全屏透明窗口
            let hwnd = CreateWindowExW(
                WS_EX_LAYERED | WS_EX_TOPMOST,
                class_name.as_ptr(),
                wide_null("Transparent Window").as_ptr(),
                WS_POPUP,
                0,
                0,
                GetSystemMetrics(SM_CXSCREEN),
                GetSystemMetrics(SM_CYSCREEN),
                null_mut(),
                null_mut(),
                h_instance,
                null_mut(),
            );

            SetLayeredWindowAttributes(hwnd, 0, 255, LWA_ALPHA);
            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);

            // 消息循环
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
                if exit_flag_clone.load(Ordering::Relaxed) {
                    PostMessageW(hwnd, WM_CLOSE, 0, 0); // 关闭窗口
                    break;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    });

    exit_flag
}

// 工具函数：生成宽字符字符串
fn wide_null(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
