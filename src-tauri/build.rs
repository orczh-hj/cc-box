fn main() {
    // 从 package.json 读取版本号并注入为编译时环境变量
    // 确保 Rust 后端与前端使用同一个版本来源
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_json_path = std::path::Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .join("package.json");

    if package_json_path.exists() {
        // 版本源变化必须触发重编，否则 APP_VERSION 会停留在旧值
        // （与 tauri 包版本不一致时，更新标记的 fromVersion 判等会失效）
        println!("cargo:rerun-if-changed={}", package_json_path.display());
        let content = std::fs::read_to_string(&package_json_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        let version = json["version"].as_str().unwrap();
        println!("cargo:rustc-env=APP_VERSION={}", version);
    }

    // Windows 测试 exe 的 comctl32 v6 manifest 说明见 src-tauri/test-runner.ps1
    // （bin 产物由 tauri_build 嵌入 manifest；lib 测试 exe 由 test-runner 放置外挂 manifest）

    tauri_build::build()
}
