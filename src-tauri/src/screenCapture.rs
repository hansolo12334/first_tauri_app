use std::ptr::null_mut;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HWND, POINT, RECT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;


use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};


static mut START_POINT: POINT = POINT { x: 0, y: 0 };
static mut END_POINT: POINT = POINT { x: 0, y: 0 };
static mut IS_DRAWING: bool = false;
static mut EIXT_CALLED: bool = false;

// 声明退出标志的 Arc<AtomicBool>
static mut EXIT_FLAG: Option<Arc<AtomicBool>> = None;

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
            println!("mousedown");
            0
        }
        WM_MOUSEMOVE => {
            println!("mousemove");
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
            
            
            
            // 设置半透明遮罩效果
            let brush = CreateSolidBrush(RGB(0, 0, 0) | (0 << 24)); // 黑色半透明
            let mut rect: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut rect);

            // let resault=ExcludeClipRect(hdc, START_POINT.x, START_POINT.y, END_POINT.x, END_POINT.y);
            // if resault!=ERROR{
                FillRect(hdc, &rect, brush);
            // }
            // let resault1=IntersectClipRect(hdc, START_POINT.x, START_POINT.y, END_POINT.x, END_POINT.y);

            DeleteObject(brush as _);

            // 绘制框选矩形
            if IS_DRAWING {
                let pen = CreatePen(2, 2, RGB(255, 0, 0)); // 红色矩形
                let old_pen = SelectObject(hdc, pen as _);
                Rectangle(hdc, START_POINT.x, START_POINT.y, END_POINT.x, END_POINT.y);
                SelectObject(hdc, old_pen);
                DeleteObject(pen as _);
            }

            EndPaint(hwnd, &ps);
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

pub fn create_screenShotWindow() ->bool {
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
                return  true;
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        return  true;
    }
}



pub fn start_screenshot_thread() -> Arc<AtomicBool> {
  let exit_flag = Arc::new(AtomicBool::new(false));
    let exit_flag_clone = Arc::clone(&exit_flag);

    thread::spawn(move || {
        unsafe {
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

            SetLayeredWindowAttributes(hwnd, 0, 128, LWA_ALPHA);
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
