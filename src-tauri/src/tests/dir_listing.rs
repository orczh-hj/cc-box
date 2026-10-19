//! list_directories（远程端目录选择器后端）单测：
//! 仅列子目录、按名称不区分大小写排序、错误路径报错。

use crate::commands::list_directories;

/// 建临时目录树：root/{alpha, Beta, gamma}/ + root/file.txt（文件不得出现）
fn temp_tree(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("cc-box-dirlist-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for sub in ["alpha", "Beta", "gamma"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
    }
    std::fs::write(dir.join("file.txt"), "x").unwrap();
    dir
}

#[tokio::test]
async fn ListDirectories_仅子目录名称排序_001() {
    let dir = temp_tree("sort");
    let list = list_directories(Some(dir.to_string_lossy().into_owned()))
        .await
        .unwrap();
    let names: Vec<&str> = list.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, vec!["alpha", "Beta", "gamma"], "字母序（不区分大小写），文件不出现");
    // path 为完整路径
    assert!(list[0].path.ends_with("alpha"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn ListDirectories_不存在路径报错_002() {
    let err = list_directories(Some("Z:/definitely/not/exist".into()))
        .await
        .unwrap_err();
    assert!(err.contains("not a directory"), "错误信息含原因: {}", err);
}
