//! Web UI 静态资源服务（relay 与桌面 remote_server 共用）
//!
//! 前端构建产物 dist 的双通道供给：
//! - **磁盘优先**：环境变量指定的目录存在 → tower-http `ServeDir` 读磁盘
//!   （前端独立更新通道——scp dist 到服务器目录即生效，免重编重部署二进制）
//! - **内嵌兜底**：未配置环境变量、目录不存在、或磁盘 404 miss → 编译期
//!   rust-embed 内嵌资产（发布版本的保底，保证二进制自身永远可服务）
//!
//! 静态 handler 与 mime 表原为 relay / remote_server 两处逐字重复，收敛于此。

use axum::body::Body;
use axum::http::{header, Request, StatusCode, Uri};
use axum::response::Response;
use rust_embed::RustEmbed;
use tower::ServiceExt;

/// 静态资源缓存策略（原缺失导致手机浏览器启发式缓存旧 index.html，
/// 一直引用旧 hash bundle——实测表现为按钮/白屏等旧行为复发）：
/// - `assets/` 下为 Vite 内容哈希文件名 → 一年 immutable，省流量且安全
/// - 其余（index.html / icon 等非哈希）→ no-cache：每次协商，部署后立即生效
pub(crate) fn cache_control_for(path: &str) -> &'static str {
    if path.starts_with("assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    }
}

fn apply_cache_headers(resp: &mut Response, path: &str) {
    if let Ok(v) = header::HeaderValue::from_str(cache_control_for(path)) {
        resp.headers_mut().insert(header::CACHE_CONTROL, v);
    }
}
use tower_http::services::ServeDir;

/// 内嵌 Web UI（folder 相对本 crate：../../dist = 仓库根构建产物；
/// debug 构建动态读目录、release 编译期嵌入——rust-embed 默认行为）
#[derive(RustEmbed)]
#[folder = "../../dist"]
struct EmbeddedAssets;

/// 静态资源 fallback handler（axum 无 state handler，两端 Router 直接挂）。
/// `env_var`：磁盘 Web 目录的环境变量名（relay 用 CC_BOX_RELAY_WEB_DIR、
/// 桌面用 CC_BOX_WEB_DIR）。每请求读取——目录内容更新（scp 新 dist）即时
/// 生效，无需重启进程；环境变量本身的增删需重启进程（进程环境固定）
pub async fn serve_static(env_var: &'static str, uri: Uri) -> Response {
    if let Some(resp) = serve_from_disk(env_var, &uri).await {
        return resp;
    }
    embedded_response(&uri)
}

/// 磁盘目录命中返回 Some（环境变量未设置/目录为空串/请求 404 → None 回内嵌）
async fn serve_from_disk(env_var: &str, uri: &Uri) -> Option<Response> {
    let dir = std::env::var(env_var).ok()?;
    let dir = dir.trim();
    if dir.is_empty() {
        return None;
    }
    let request = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .ok()?;
    let response = ServeDir::new(dir).oneshot(request).await.ok()?;
    // 磁盘 miss（404）不直接透出——内嵌兜底（部分更新场景：新文件未 scp 到）。
    // map 换 body 类型：ServeDir 的 ServeFileSystemResponseBody → axum Body
    (response.status() != StatusCode::NOT_FOUND).then(|| {
        let mut resp = response.map(Body::new);
        apply_cache_headers(&mut resp, uri.path().trim_start_matches('/'));
        resp
    })
}

/// 内嵌资产响应（路径为空 → index.html；mime 按扩展名）
fn embedded_response(uri: &Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };
    match EmbeddedAssets::get(path) {
        Some(file) => {
            let mut resp = Response::builder()
                .header(header::CONTENT_TYPE, mime_for(path))
                .body(Body::from(file.data))
                .unwrap();
            apply_cache_headers(&mut resp, path);
            resp
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("not found"))
            .unwrap(),
    }
}

/// 扩展名 → Content-Type
fn mime_for(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("webmanifest") => "application/manifest+json",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 磁盘优先：环境变量指向的目录命中文件 → 返回磁盘内容
    #[tokio::test]
    async fn WebAssets_磁盘目录优先() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("probe.html"), "<html>disk</html>").unwrap();
        std::env::set_var("CC_TEST_WEB_DIR_DISK", dir.path());

        let uri: Uri = "/probe.html".parse().unwrap();
        let resp = serve_static("CC_TEST_WEB_DIR_DISK", uri).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"<html>disk</html>");

        std::env::remove_var("CC_TEST_WEB_DIR_DISK");
    }

    /// 环境变量未设置 → 内嵌兜底（dist 构建产物必有 index.html）
    #[tokio::test]
    async fn WebAssets_未配置回内嵌() {
        let uri: Uri = "/".parse().unwrap();
        let resp = serve_static("CC_TEST_WEB_DIR_UNSET", uri).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let content_type = resp.headers().get(header::CONTENT_TYPE).unwrap();
        assert!(content_type.to_str().unwrap().starts_with("text/html"));
    }

    /// 磁盘 404 miss（目录存在但文件未同步）→ 内嵌兜底，不透出 404
    #[tokio::test]
    async fn WebAssets_磁盘miss回内嵌() {
        let dir = tempfile::tempdir().unwrap();
        std::env::set_var("CC_TEST_WEB_DIR_MISS", dir.path());

        let uri: Uri = "/".parse().unwrap();
        let resp = serve_static("CC_TEST_WEB_DIR_MISS", uri).await;
        assert_eq!(resp.status(), StatusCode::OK);

        std::env::remove_var("CC_TEST_WEB_DIR_MISS");
    }

    /// 路径穿越拒绝：ServeDir 拒绝 + 内嵌同样 miss → 404（不泄露 dist 外文件）
    #[tokio::test]
    async fn WebAssets_路径穿越拒绝() {
        let uri: Uri = "/../Cargo.toml".parse().unwrap();
        let resp = serve_static("CC_TEST_WEB_DIR_UNSET", uri).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    /// 内嵌资产含 favicon（public/icon.png 经 vite build 拷入 dist；
    /// 若 public 拷贝链断裂或 dist 过期，index.html 的 /icon.png 将 404）
    #[test]
    fn WebAssets_内嵌资产含图标() {
        assert!(EmbeddedAssets::get("icon.png").is_some());
    }

    /// 内嵌未知路径 404（与原两处实现行为一致）
    #[tokio::test]
    async fn WebAssets_内嵌miss404() {
        let uri: Uri = "/no-such-file.xyz".parse().unwrap();
        let resp = serve_static("CC_TEST_WEB_DIR_UNSET", uri).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    /// 缓存策略：哈希资源一年 immutable，其余 no-cache（部署即生效）
    #[test]
    fn cache_control_策略_001() {
        assert_eq!(
            cache_control_for("assets/index-abc123.js"),
            "public, max-age=31536000, immutable"
        );
        assert_eq!(cache_control_for(""), "no-cache");
        assert_eq!(cache_control_for("index.html"), "no-cache");
        assert_eq!(cache_control_for("icon.png"), "no-cache");
    }

    /// 端到端：磁盘通道的响应必须带 Cache-Control（原缺失 → 手机缓存旧
    /// index.html 引用旧 bundle，表现为已修复的问题复发）
    #[tokio::test]
    async fn serve_static_缓存头_002() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("index.html"), "<html></html>").unwrap();
        std::fs::create_dir_all(dir.path().join("assets")).unwrap();
        std::fs::write(dir.path().join("assets/app-abc123.js"), "console.log(1)").unwrap();

        let env = "CC_BOX_TEST_WEB_DIR_CACHE";
        std::env::set_var(env, dir.path());

        let cc = |resp: &Response| {
            resp.headers()
                .get(header::CACHE_CONTROL)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string()
        };

        let index = serve_static(env, "/".parse().unwrap()).await;
        assert_eq!(cc(&index), "no-cache");

        let asset = serve_static(env, "/assets/app-abc123.js".parse().unwrap()).await;
        assert_eq!(cc(&asset), "public, max-age=31536000, immutable");

        std::env::remove_var(env);
    }
}
