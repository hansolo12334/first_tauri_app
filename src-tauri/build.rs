// use std::env;
// use std::path::PathBuf;

fn main() {
    // 获取 venv 环境中的 Python 解释器路径
    // let venv_path = env::var("VIRTUAL_ENV").expect("VIRTUAL_ENV environment variable is not set. Please activate your virtual environment.");
    // let python_executable = PathBuf::from(venv_path).join("Scripts").join("python.exe");
    
    // // 设置 PYTHON_SYS_EXECUTABLE 环境变量
    // println!("cargo:rustc-env=PYTHON_SYS_EXECUTABLE={}", python_executable.display());

    tauri_build::build()
}