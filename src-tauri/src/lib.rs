use std::any::Any;

use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder},
    App, AppHandle, Emitter, Event, Listener, Manager,
};

use std::path::Path;

mod utils;
mod screenCapture;
// use image::{GenericImageView, imageops::FilterType};
// use ndarray::{Array, Axis, s};
// use pyo3::ffi::c_str;
// use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;
// use pyo3::types::PyModule;
// use onnxruntime_sys::*;
// use onnxrt_sys::*;
// use std::os::windows::ffi::OsStrExt;
// use ort::{GraphOptimizationLevel, Session};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}! Rust向你打招呼!", name)
}

#[tauri::command]
fn print_something() {
    println!("test11");
}

#[tauri::command]
fn greet_to_user(name: &str) -> String {
    print_something();
    format!("{}你好", name)
}



#[tauri::command]
async fn open_new_page(handle: tauri::AppHandle) -> Result<(), tauri::Error> {
    // let _ = test();
    // let _ = call_add_from_py();
    // test_onnx1();
    // utils::get_screenshot();
    // utils::draw_select_window();

    let new_window = tauri::WebviewWindowBuilder::new(
        &handle,
        "new_window",
        tauri::WebviewUrl::App("second_page.html".into()),
    )
    .title("new page")
    .min_inner_size(320.0, 450.0)
    .inner_size(320.0, 450.0)
    .build()?;


    //尝试隐藏窗口
    // let new_window_cpy: tauri::WebviewWindow = new_window.clone();
    // new_window.listen("screenShotOpen", move |event| {
    //     println!("from scend page : screenShotWindwow is open");
    //     new_window_cpy.hide().unwrap();
    //     new_window_cpy.emit("second_window_hide", ()).unwrap();
    //     println!("隐藏窗口");
    // });
    // let new_window_cpy2: tauri::WebviewWindow = new_window.clone();
    // new_window.listen("screenShotClose", move |event| {
    //     println!("from scend page : screenShotWindwow is closed");
    //     new_window_cpy2.show().unwrap();
    // });

 
    let new_window_clone = new_window.clone();
    new_window.on_window_event(move |event| {
        handle_window_event(&new_window_clone, event);
    });
    Ok(())
}

#[tauri::command]
async fn open_screen_shot_page(handle: tauri::AppHandle) -> Result<(), tauri::Error> {
    let re = utils::draw_select_window(handle);
    re
}

#[tauri::command]
async fn open_screen_shot_page_derict(){
    let exit_flag = screenCapture::start_screenshot_thread();

    // thread::sleep(std::time::Duration::from_secs(5));
    // exit_flag.store(true, Ordering::Relaxed);
}

// fn call_add_from_py() -> PyResult<()> {
//     let py_add_app = include_str!(concat!(
//         env!("CARGO_MANIFEST_DIR"),
//         "/py_scripts/test_import.py"
//     ));

//     let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
//         let app: Py<PyAny> = PyModule::from_code_bound(py, py_add_app, "", "")?
//             .getattr("add_x_y")?
//             .into();
//         app.call0(py)
//     });

//     println!("py: {}", from_python?);
//     Ok(())
// }

// onnex
// fn test_onnx()-> ort::Result<()> {
//     let model = Session::builder()?
//     .with_optimization_level(GraphOptimizationLevel::Level3)?
//     .with_intra_threads(4)?
//     .commit_from_file("../models/best.onnx")?;

// let original_img = image::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("image").join("1.png")).unwrap();
// let outputs = model.run(ort::inputs!["image" => image]?)?;
// Ok(())
// }

// onnex
// fn char_p_to_str<'a>(raw: *const i8) -> Result<&'a str, std::str::Utf8Error> {
//     let c_str = unsafe { std::ffi::CStr::from_ptr(raw as *mut i8) };
//     c_str.to_str()
// }

// fn CheckStatus(g_ort: *const OrtApi, status: *const OrtStatus) -> Result<(), String> {
//     if status != std::ptr::null() {
//         let raw = unsafe { g_ort.as_ref().unwrap().GetErrorMessage.unwrap()(status) };
//         Err(char_p_to_str(raw).unwrap().to_string())
//     } else {
//         Ok(())
//     }
// }

// fn test_onnx1(){
//     let g_ort = unsafe { OrtGetApiBase().as_ref().unwrap().GetApi.unwrap()(ORT_API_VERSION) };
//     assert_ne!(g_ort, std::ptr::null_mut());

//     let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();
//     let env_name = std::ffi::CString::new("test").unwrap();
//     let status = unsafe {
//         g_ort.as_ref().unwrap().CreateEnv.unwrap()(
//             OrtLoggingLevel::ORT_LOGGING_LEVEL_VERBOSE,
//             env_name.as_ptr(),
//             &mut env_ptr,
//         )
//     };
//     CheckStatus(g_ort, status).unwrap();
//     assert_ne!(env_ptr, std::ptr::null_mut());

//     // initialize session options if needed
//     let mut session_options_ptr: *mut OrtSessionOptions = std::ptr::null_mut();
//     let status =
//         unsafe { g_ort.as_ref().unwrap().CreateSessionOptions.unwrap()(&mut session_options_ptr) };
//     CheckStatus(g_ort, status).unwrap();
//     unsafe { g_ort.as_ref().unwrap().SetIntraOpNumThreads.unwrap()(session_options_ptr, 1) };
//     assert_ne!(session_options_ptr, std::ptr::null_mut());

//     // Sets graph optimization level
//     unsafe {
//         g_ort
//             .as_ref()
//             .unwrap()
//             .SetSessionGraphOptimizationLevel
//             .unwrap()(
//             session_options_ptr,
//             GraphOptimizationLevel::ORT_ENABLE_BASIC,
//         )
//     };

//     let model_path = std::ffi::OsString::from("D:\\DeskTop\\tauri_learn\\hansolo-first-tauri\\src-tauri\\models\\best_low_level6.onnx");

//     let model_path: Vec<u16> = model_path
//         .encode_wide()
//         .chain(std::iter::once(0)) // Make sure we have a null terminated string
//         .collect();

//     let mut session_ptr: *mut OrtSession = std::ptr::null_mut();

//     println!("Using Onnxruntime C API");
//     let status = unsafe {
//         g_ort.as_ref().unwrap().CreateSession.unwrap()(
//             env_ptr,
//             model_path.as_ptr(),
//             session_options_ptr,
//             &mut session_ptr,
//         )
//     };
//     CheckStatus(g_ort, status).unwrap();
//     assert_ne!(session_ptr, std::ptr::null_mut());

//     //*************************************************************************
//     // print model input layer (node names, types, shape etc.)
//     // size_t num_input_nodes;
//     let mut allocator_ptr: *mut OrtAllocator = std::ptr::null_mut();
//     let status = unsafe {
//         g_ort
//             .as_ref()
//             .unwrap()
//             .GetAllocatorWithDefaultOptions
//             .unwrap()(&mut allocator_ptr)
//     };
//     CheckStatus(g_ort, status).unwrap();
//     assert_ne!(allocator_ptr, std::ptr::null_mut());

//     // print number of model input nodes
//     let mut num_input_nodes: usize = 0;
//     let status = unsafe {
//         g_ort.as_ref().unwrap().SessionGetInputCount.unwrap()(session_ptr, &mut num_input_nodes)
//     };
//     CheckStatus(g_ort, status).unwrap();
//     assert_ne!(num_input_nodes, 0);
//     println!("Number of inputs = {:?}", num_input_nodes);
//     let mut input_node_names: Vec<&str> = Vec::new();
//     let mut input_node_dims: Vec<i64> = Vec::new(); // simplify... this model has only 1 input node {1, 3, 224, 224}.
//                                                     // Otherwise need vector<vector<>>

//     // iterate over all input nodes
//     for i in 0..num_input_nodes {
//         // print input node names
//         let mut input_name: *mut i8 = std::ptr::null_mut();
//         let status = unsafe {
//             g_ort.as_ref().unwrap().SessionGetInputName.unwrap()(
//                 session_ptr,
//                 i,
//                 allocator_ptr,
//                 &mut input_name,
//             )
//         };
//         CheckStatus(g_ort, status).unwrap();
//         assert_ne!(input_name, std::ptr::null_mut());

//         // WARNING: The C function SessionGetInputName allocates memory for the string.
//         //          We cannot let Rust free that string, the C side must free the string.
//         //          We thus convert the pointer to a string slice (&str).
//         let input_name = char_p_to_str(input_name).unwrap();
//         println!("Input {} : name={}", i, input_name);
//         input_node_names.push(input_name);

//         // print input node types
//         let mut typeinfo_ptr: *mut OrtTypeInfo = std::ptr::null_mut();
//         let status = unsafe {
//             g_ort.as_ref().unwrap().SessionGetInputTypeInfo.unwrap()(
//                 session_ptr,
//                 i,
//                 &mut typeinfo_ptr,
//             )
//         };
//         CheckStatus(g_ort, status).unwrap();
//         assert_ne!(typeinfo_ptr, std::ptr::null_mut());

//         let mut tensor_info_ptr: *const OrtTensorTypeAndShapeInfo = std::ptr::null_mut();
//         let status = unsafe {
//             g_ort.as_ref().unwrap().CastTypeInfoToTensorInfo.unwrap()(
//                 typeinfo_ptr,
//                 &mut tensor_info_ptr,
//             )
//         };
//         CheckStatus(g_ort, status).unwrap();
//         assert_ne!(tensor_info_ptr, std::ptr::null_mut());

//         let mut type_: ONNXTensorElementDataType =
//             ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UNDEFINED;
//         let status = unsafe {
//             g_ort.as_ref().unwrap().GetTensorElementType.unwrap()(tensor_info_ptr, &mut type_)
//         };
//         CheckStatus(g_ort, status).unwrap();
//         assert_ne!(
//             type_,
//             ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UNDEFINED
//         );

//         println!("Input {} : type={}", i, type_ as i32);

//         // print input shapes/dims
//         let mut num_dims = 0;
//         let status = unsafe {
//             g_ort.as_ref().unwrap().GetDimensionsCount.unwrap()(tensor_info_ptr, &mut num_dims)
//         };
//         CheckStatus(g_ort, status).unwrap();
//         assert_ne!(num_dims, 0);

//         println!("Input {} : num_dims={}", i, num_dims);
//         input_node_dims.resize_with(num_dims as usize, Default::default);
//         let status = unsafe {
//             g_ort.as_ref().unwrap().GetDimensions.unwrap()(
//                 tensor_info_ptr,
//                 input_node_dims.as_mut_ptr(),
//                 num_dims,
//             )
//         };
//         CheckStatus(g_ort, status).unwrap();

//         for j in 0..num_dims {
//             println!("Input {} : dim {}={}", i, j, input_node_dims[j as usize]);
//         }

//         unsafe { g_ort.as_ref().unwrap().ReleaseTypeInfo.unwrap()(typeinfo_ptr) };
//     }

// }

// #[test]
// fn test() -> PyResult<()> {
//     Python::with_gil(|py| {
//         let sys = py.import_bound("sys")?;
//         let version: String = sys.getattr("version")?.extract()?;

//         let locals = [("os", py.import_bound("os")?)].into_py_dict_bound(py);

//         let user_code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";

//         let user_computer: String = py.eval_bound(user_code, None, Some(&locals))?.extract()?;

//         println!(" {}的 python 版本{}", user_computer, version);
//         Ok(())
//     })
// }

fn menu_setup(app: &App) -> tauri::Result<()> {
    // let quit=MenuItemBuilder::new("Quit")
    //             .accelerator("Ctrl+Shift+Q")
    //             .build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

    let file_submenu = SubmenuBuilder::new(app, "File").item(&quit).build()?;

    let menu = MenuBuilder::new(app).item(&file_submenu).build()?;

    // let _ =app.set_menu(menu);
    app.set_menu(menu)?; //处理返回值的两种方法

    Ok(())
}

fn handle_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    println!("into event!");

    match event.id() {
        id if id == "quit" => {
            println!("Quit!");
            app.exit(1);
        }
        _ => {}
    }
    // if event.id()=="Quit"{

    // }
}

fn handle_window_event(window: &tauri::WebviewWindow, event: &tauri::WindowEvent) {
    // println!("into window event");

    // println!("{:?}",event);

    // if window.label()=="new_window" {
    match event {
        tauri::WindowEvent::Resized(size) => {
            println!(" {:?},{:?} ", size.height, size.width);
            window
                .emit("window-resized2x", size)
                .expect("failed to emit event");
        }
        _ => {}
    }

    // }
}

//添加Tauri 所需的权限，包括事件监听、事件发射和窗口创建的权限
fn generate_context() -> tauri::Context {
    let mut context = tauri::generate_context!("..\tauri.conf.json");
    for cmd in [
        "plugin:event|listen",
        "plugin:event|emit",
        "plugin:event|emit_to",
        "plugin:webview|create_webview_window",
    ] {
        context
            .runtime_authority_mut()
            .__allow_command(cmd.to_string(), tauri_utils::acl::ExecutionContext::Local);
    }
    context
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            menu_setup(app)?;
            Ok(())
        })
        .on_menu_event(handle_menu_event)
        // .on_window_event(handle_window_event)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            greet_to_user,
            open_new_page,
            open_screen_shot_page,
            open_screen_shot_page_derict
        ])
        .run(generate_context())
        .expect("error while running tauri application");
}
