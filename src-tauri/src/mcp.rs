//! MCP 协议模块
//!
//! 通过 MCP 协议获取服务器详情（tools、prompts、resources）
//! 支持 HTTP/SSE 和 stdio 类型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// ==================== 数据结构 ====================

/// MCP Server 详细信息（通过协议获取）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpServerDetail {
    pub name: String,
    pub server_info: Option<ServerInfo>,
    pub capabilities: Option<ServerCapabilities>,
    pub tools: Vec<McpToolInfo>,
    pub prompts: Vec<McpPromptInfo>,
    pub resources: Vec<McpResourceInfo>,
    pub cached_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub tools: bool,
    pub prompts: bool,
    pub resources: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpToolInfo {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPromptInfo {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<PromptArgument>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpResourceInfo {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

// ==================== JSON-RPC 结构 ====================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonRpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<serde_json::Value>,
}

/// 宽松的 JSON-RPC 响应（id 可选，用于匹配服务端主动推送的通知）
#[derive(Debug, Deserialize)]
struct LooseJsonRpcMessage {
    #[allow(dead_code)]
    jsonrpc: Option<String>,
    id: Option<u64>,
    result: Option<serde_json::Value>,
    error: Option<JsonRpcError>,
    #[allow(dead_code)]
    method: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct JsonRpcResponse {
    pub(crate) jsonrpc: String,
    pub(crate) id: u64,
    pub(crate) result: Option<serde_json::Value>,
    pub(crate) error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct JsonRpcError {
    code: i32,
    message: String,
}

// ==================== MCP HTTP 客户端 ====================

pub struct McpHttpClient {
    url: String,
    headers: Option<HashMap<String, String>>,
    request_id: u64,
}

impl McpHttpClient {
    pub fn new(url: String, headers: Option<HashMap<String, String>>) -> Self {
        Self {
            url,
            headers,
            request_id: 0,
        }
    }

    fn next_id(&mut self) -> u64 {
        self.request_id += 1;
        self.request_id
    }

    /// 获取服务器详情
    pub async fn get_server_detail(&mut self) -> Result<McpServerDetail, String> {
        log::info!("[MCP] Fetching HTTP server detail: {}", self.url);

        // 直连优先（系统代理常把国内端点路由到远端节点导致握手被掐断），
        // 失败再走系统代理兜底（GitHub 等需要代理的端点）
        let direct = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .no_proxy()
            .build()
            .map_err(|e| {
                let msg = format!("Failed to build HTTP client: {}", e);
                log::error!("[MCP] {}", msg);
                msg
            })?;

        match self.fetch_detail_with(&direct).await {
            Ok(detail) => Ok(detail),
            Err(direct_err) => {
                log::warn!(
                    "[MCP] HTTP direct fetch failed for {} ({}), retrying via system proxy",
                    self.url,
                    direct_err
                );
                let via_proxy = reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .map_err(|e| e.to_string())?;
                self.request_id = 0;
                self.fetch_detail_with(&via_proxy).await.map_err(|e| {
                    log::error!("[MCP] HTTP fetch failed for {}: {} (direct: {})", self.url, e, direct_err);
                    e
                })
            }
        }
    }

    async fn fetch_detail_with(&mut self, client: &reqwest::Client) -> Result<McpServerDetail, String> {

        // 1. Initialize
        let init_response = self
            .send_request(
                &client,
                "initialize",
                Some(serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {
                        "name": "Claude-Tauri-GUI",
                        "version": "1.0.0"
                    }
                })),
            )
            .await
            .map_err(|e| {
                log::error!("[MCP] HTTP initialize failed for {}: {}", self.url, e);
                e
            })?;

        // 解析 initialize 结果
        let init_result = init_response.result.unwrap_or(serde_json::json!({}));
        let server_info = parse_server_info(&init_result);
        let capabilities = parse_capabilities(&init_result);
        log::info!(
            "[MCP] HTTP initialized: {:?}, caps: tools={}, prompts={}, resources={}",
            server_info,
            capabilities.tools,
            capabilities.prompts,
            capabilities.resources
        );

        // 2. 发送 initialized 通知（无响应）
        self.send_notification(&client, "notifications/initialized")
            .await
            .map_err(|e| {
                log::error!("[MCP] HTTP initialized notification failed for {}: {}", self.url, e);
                e
            })?;

        // 3. 列出工具
        let tools = if capabilities.tools {
            let tools_response = self.send_request(&client, "tools/list", None).await.map_err(|e| {
                log::error!("[MCP] HTTP tools/list failed for {}: {}", self.url, e);
                e
            })?;
            parse_tools(&tools_response)
        } else {
            vec![]
        };

        // 4. 列出 prompts
        let prompts = if capabilities.prompts {
            let prompts_response = self.send_request(&client, "prompts/list", None).await.map_err(|e| {
                log::error!("[MCP] HTTP prompts/list failed for {}: {}", self.url, e);
                e
            })?;
            parse_prompts(&prompts_response)
        } else {
            vec![]
        };

        // 5. 列出 resources
        let resources = if capabilities.resources {
            let resources_response = self.send_request(&client, "resources/list", None).await.map_err(|e| {
                log::error!("[MCP] HTTP resources/list failed for {}: {}", self.url, e);
                e
            })?;
            parse_resources(&resources_response)
        } else {
            vec![]
        };

        log::info!("[MCP] HTTP detail fetched successfully: {} (tools={}, prompts={}, resources={})", self.url, tools.len(), prompts.len(), resources.len());

        Ok(McpServerDetail {
            name: self.url.clone(),
            server_info,
            capabilities: Some(capabilities),
            tools,
            prompts,
            resources,
            cached_at: Some(chrono::Utc::now().timestamp()),
        })
    }

    async fn send_request(
        &mut self,
        client: &reqwest::Client,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<JsonRpcResponse, String> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: self.next_id(),
            method: method.to_string(),
            params,
        };

        let mut req = client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json,text/event-stream")
            .header("MCP-Protocol-Version", "2024-11-05")
            .json(&request);

        // 添加自定义 headers
        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                req = req.header(key, value);
            }
        }

        let response = req
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(format!("HTTP error {}: {}", status, body));
        }

        // 解析 SSE 格式响应或 JSON 格式
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if content_type.contains("text/event-stream") {
            // SSE 格式：解析 data: 行
            let text = response.text().await.map_err(|e| e.to_string())?;
            parse_sse_response(&text)
        } else {
            // JSON 格式
            let json: JsonRpcResponse = response
                .json()
                .await
                .map_err(|e| format!("JSON parse error: {}", e))?;

            if let Some(error) = json.error {
                return Err(format!("RPC error {}: {}", error.code, error.message));
            }

            Ok(json)
        }
    }

    async fn send_notification(
        &self,
        client: &reqwest::Client,
        method: &str,
    ) -> Result<(), String> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 0, // notifications 不需要 id
            method: method.to_string(),
            params: None,
        };

        let mut req = client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json,text/event-stream")
            .header("MCP-Protocol-Version", "2024-11-05")
            .json(&request);

        // 添加自定义 headers
        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                req = req.header(key, value);
            }
        }

        req.send()
            .await
            .map_err(|e| format!("Notification failed: {}", e))?;

        Ok(())
    }
}

// ==================== 解析函数 ====================

/// 解析 SSE 格式响应
pub(crate) fn parse_sse_response(text: &str) -> Result<JsonRpcResponse, String> {
    // SSE 格式：
    // id:1
    // event:message
    // data:{"jsonrpc":"2.0","id":1,"result":{...}}
    for line in text.lines() {
        if line.starts_with("data:") {
            let json_str = line.trim_start_matches("data:").trim();
            let json: JsonRpcResponse = serde_json::from_str(json_str)
                .map_err(|e| format!("JSON parse error in SSE: {} (line: {})", e, json_str))?;

            if let Some(error) = json.error {
                return Err(format!("RPC error {}: {}", error.code, error.message));
            }

            return Ok(json);
        }
    }

    Err("No data line found in SSE response".to_string())
}

pub(crate) fn parse_server_info(result: &serde_json::Value) -> Option<ServerInfo> {
    result.get("serverInfo").and_then(|info| {
        Some(ServerInfo {
            name: info
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            version: info
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        })
    })
}

pub(crate) fn parse_capabilities(result: &serde_json::Value) -> ServerCapabilities {
    let empty = serde_json::json!({});
    let caps = result.get("capabilities").unwrap_or(&empty);
    ServerCapabilities {
        tools: caps.get("tools").is_some(),
        prompts: caps.get("prompts").is_some(),
        resources: caps.get("resources").is_some(),
    }
}

pub(crate) fn parse_tools(response: &JsonRpcResponse) -> Vec<McpToolInfo> {
    response
        .result
        .as_ref()
        .and_then(|r| r.get("tools"))
        .and_then(|tools| tools.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    Some(McpToolInfo {
                        name: t
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        description: t
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        input_schema: t.get("inputSchema").cloned(),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) fn parse_prompts(response: &JsonRpcResponse) -> Vec<McpPromptInfo> {
    response
        .result
        .as_ref()
        .and_then(|r| r.get("prompts"))
        .and_then(|prompts| prompts.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|p| {
                    Some(McpPromptInfo {
                        name: p
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        description: p
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        arguments: p
                            .get("arguments")
                            .and_then(|args| args.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|a| {
                                        Some(PromptArgument {
                                            name: a
                                                .get("name")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("")
                                                .to_string(),
                                            description: a
                                                .get("description")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string()),
                                            required: a
                                                .get("required")
                                                .and_then(|v| v.as_bool())
                                                .unwrap_or(false),
                                        })
                                    })
                                    .collect()
                            }),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) fn parse_resources(response: &JsonRpcResponse) -> Vec<McpResourceInfo> {
    response
        .result
        .as_ref()
        .and_then(|r| r.get("resources"))
        .and_then(|resources| resources.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|r| {
                    Some(McpResourceInfo {
                        uri: r
                            .get("uri")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        name: r
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        description: r
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        mime_type: r
                            .get("mimeType")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

// ==================== 缓存管理 ====================

type CacheKey = String; // server_name or URL
type CacheValue = McpServerDetail;

/// MCP Detail 缓存（永久缓存，除非强制刷新）
pub struct McpDetailCache {
    cache: Arc<Mutex<HashMap<CacheKey, CacheValue>>>,
}

impl McpDetailCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 获取缓存（不检查时间，永久有效）
    pub async fn get(&self, key: &str) -> Option<McpServerDetail> {
        let cache = self.cache.lock().await;
        cache.get(key).cloned()
    }

    /// 设置缓存
    pub async fn set(&self, key: String, detail: McpServerDetail) {
        let mut cache = self.cache.lock().await;
        cache.insert(key, detail);
    }
}

// 全局缓存实例
static MCP_CACHE: once_cell::sync::Lazy<McpDetailCache> =
    once_cell::sync::Lazy::new(McpDetailCache::new);

pub fn get_mcp_cache() -> &'static McpDetailCache {
    &MCP_CACHE
}

// ==================== MCP Stdio 客户端 ====================

/// Windows `cmd /S /C` 命令行拼装：含空白或 cmd 元字符（`&|<>^()`,`;=` 等）的
/// token 加引号（防止无空格但含元字符的参数被 cmd 解释为管道/分隔，如 `a&b`）；
/// token 内的 `"` 按 Windows 参数规则转义为 `\"`（`/S` 保证整体外层引号被剥掉
/// 后内层结构不变）。
#[cfg(target_os = "windows")]
pub(crate) fn windows_cmd_line(command: &str, args: &[String]) -> String {
    const METACHARS: &str = "&|<>^()\",;=";
    fn quote(s: &str) -> String {
        let needs = s.is_empty()
            || s.chars().any(|c| c.is_whitespace() || METACHARS.contains(c));
        if needs {
            format!("\"{}\"", s.replace('"', "\\\""))
        } else {
            s.to_string()
        }
    }
    let mut parts = vec![quote(command)];
    parts.extend(args.iter().map(|a| quote(a)));
    parts.join(" ")
}

pub struct McpStdioClient {
    command: String,
    args: Vec<String>,
    env: Option<HashMap<String, String>>,
    request_id: u64,
}

impl McpStdioClient {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self {
            command,
            args,
            env: None,
            request_id: 0,
        }
    }

    pub fn set_env(&mut self, env: Option<HashMap<String, String>>) {
        self.env = env;
    }

    fn next_id(&mut self) -> u64 {
        self.request_id += 1;
        self.request_id
    }

    /// 获取服务器详情（通过 stdin/stdout 通信）
    pub fn get_server_detail(&mut self) -> Result<McpServerDetail, String> {
        let cmd_line = format!("{} {}", self.command, self.args.join(" "));
        log::info!("[MCP] Fetching stdio server detail: {}", cmd_line.trim());

        // Windows 上 npx 等脚本命令需要通过 cmd.exe /C 执行
        let mut cmd;
        #[cfg(target_os = "windows")]
        {
            let full_cmd = windows_cmd_line(&self.command, &self.args);
            cmd = Command::new("cmd");
            cmd.arg("/S").arg("/C");
            cmd.raw_arg(format!("\"{}\"", full_cmd));
            crate::platform::configure_command(&mut cmd);
        }
        #[cfg(not(target_os = "windows"))]
        {
            cmd = Command::new(&self.command);
            cmd.args(&self.args);
        }
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // 注入环境变量
        if let Some(env_vars) = &self.env {
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| {
                let msg = format!("Failed to spawn process '{}': {}", self.command, e);
                log::error!("[MCP] {}", msg);
                msg
            })?;

        let stdin = child.stdin.take().ok_or_else(|| {
            log::error!("[MCP] Failed to get stdin for '{}'", self.command);
            "Failed to get stdin".to_string()
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            log::error!("[MCP] Failed to get stdout for '{}'", self.command);
            "Failed to get stdout".to_string()
        })?;
        let stderr = child.stderr.take();
        let mut reader = BufReader::new(stdout);
        let mut stdin = stdin;

        // 1. Initialize
        let init_response = self.send_request(
            &mut stdin,
            &mut reader,
            "initialize",
            Some(serde_json::json!({
                "protocolVersion": "2025-11-25",
                "capabilities": {},
                "clientInfo": {
                    "name": "Claude-Tauri-GUI",
                    "version": "1.0.0"
                }
            })),
        )
        .map_err(|e| {
            let stderr_output = stderr
                .and_then(|mut s| {
                    let mut buf = String::new();
                    s.read_to_string(&mut buf).ok().map(|_| buf)
                })
                .unwrap_or_default();
            log::error!(
                "[MCP] stdio initialize failed for '{}': {} | stderr: {}",
                self.command,
                e,
                if stderr_output.is_empty() { "(empty)" } else { &stderr_output }
            );
            e
        })?;

        let init_result = init_response.result.unwrap_or(serde_json::json!({}));
        let server_info = parse_server_info(&init_result);
        let capabilities = parse_capabilities(&init_result);
        log::info!(
            "[MCP] stdio initialized: {:?}, caps: tools={}, prompts={}, resources={}",
            server_info,
            capabilities.tools,
            capabilities.prompts,
            capabilities.resources
        );

        // 2. initialized 通知
        self.send_notification(&mut stdin)?;

        // 3. tools/list
        let tools = if capabilities.tools {
            let tools_response = self.send_request(&mut stdin, &mut reader, "tools/list", None).map_err(|e| {
                log::error!("[MCP] stdio tools/list failed for '{}': {}", self.command, e);
                e
            })?;
            parse_tools(&tools_response)
        } else {
            vec![]
        };

        // 4. prompts/list
        let prompts = if capabilities.prompts {
            let prompts_response =
                self.send_request(&mut stdin, &mut reader, "prompts/list", None).map_err(|e| {
                    log::error!("[MCP] stdio prompts/list failed for '{}': {}", self.command, e);
                    e
                })?;
            parse_prompts(&prompts_response)
        } else {
            vec![]
        };

        // 5. resources/list
        let resources = if capabilities.resources {
            let resources_response =
                self.send_request(&mut stdin, &mut reader, "resources/list", None).map_err(|e| {
                    log::error!("[MCP] stdio resources/list failed for '{}': {}", self.command, e);
                    e
                })?;
            parse_resources(&resources_response)
        } else {
            vec![]
        };

        // 关闭进程
        child.kill().ok();

        log::info!("[MCP] stdio detail fetched successfully: {} (tools={}, prompts={}, resources={})", self.command, tools.len(), prompts.len(), resources.len());

        Ok(McpServerDetail {
            name: self.command.clone(),
            server_info,
            capabilities: Some(capabilities),
            tools,
            prompts,
            resources,
            cached_at: Some(chrono::Utc::now().timestamp()),
        })
    }

    fn send_request<R: BufRead>(
        &mut self,
        stdin: &mut std::process::ChildStdin,
        reader: &mut R,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<JsonRpcResponse, String> {
        let req_id = self.next_id();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: req_id,
            method: method.to_string(),
            params,
        };

        let json_str = serde_json::to_string(&request).map_err(|e| e.to_string())?;
        log::debug!("[MCP] stdio >> {}", json_str);
        stdin
            .write_all(json_str.as_bytes())
            .map_err(|e| e.to_string())?;
        stdin.write_all(b"\n").map_err(|e| e.to_string())?;
        stdin.flush().map_err(|e| e.to_string())?;

        // 循环读取，跳过服务端主动推送的通知/日志，按 id 匹配响应
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).map_err(|e| {
                format!("read_line failed for '{}' request: {}", method, e)
            })?;

            if bytes_read == 0 {
                return Err(format!(
                    "EOF while waiting for '{}' response (id={})",
                    method, req_id
                ));
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            log::debug!("[MCP] stdio << {}", trimmed);

            // 宽松解析
            let msg: LooseJsonRpcMessage = match serde_json::from_str(trimmed) {
                Ok(m) => m,
                Err(e) => {
                    log::warn!(
                        "[MCP] stdio skipping non-JSON line for '{}' (id={}): {} (error: {})",
                        method,
                        req_id,
                        trimmed,
                        e
                    );
                    continue;
                }
            };

            // 检查 id 是否匹配
            match msg.id {
                Some(id) if id == req_id => {
                    if let Some(error) = msg.error {
                        return Err(format!("RPC error {}: {}", error.code, error.message));
                    }
                    return Ok(JsonRpcResponse {
                        jsonrpc: msg.jsonrpc.unwrap_or_else(|| "2.0".to_string()),
                        id,
                        result: msg.result,
                        error: None,
                    });
                }
                Some(id) => {
                    log::warn!(
                        "[MCP] stdio skipping response with unexpected id={} (expected {})",
                        id,
                        req_id
                    );
                    continue;
                }
                None => {
                    // 服务端主动推送的通知，跳过
                    log::debug!(
                        "[MCP] stdio skipping server notification: method={:?}",
                        msg.method
                    );
                    continue;
                }
            }
        }
    }

    fn send_notification(&self, stdin: &mut std::process::ChildStdin) -> Result<(), String> {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        });

        let json_str = serde_json::to_string(&request).map_err(|e| e.to_string())?;
        stdin
            .write_all(json_str.as_bytes())
            .map_err(|e| e.to_string())?;
        stdin.write_all(b"\n").map_err(|e| e.to_string())?;
        stdin.flush().map_err(|e| e.to_string())?;

        Ok(())
    }
}

// ==================== 获取 MCP Server 详情 ====================

/// 通过 URL 获取 HTTP MCP Server 详情
pub async fn fetch_http_mcp_detail(
    url: &str,
    headers: Option<&HashMap<String, String>>,
) -> Result<McpServerDetail, String> {
    let mut client = McpHttpClient::new(url.to_string(), headers.map(|h| h.clone()));
    client.get_server_detail().await
}

/// 通过命令获取 stdio MCP Server 详情
pub fn fetch_stdio_mcp_detail(
    command: &str,
    args: Option<&Vec<String>>,
    env: Option<&HashMap<String, String>>,
) -> Result<McpServerDetail, String> {
    let (cmd, cmd_args) = if let Some(args_vec) = args {
        (command.to_string(), args_vec.clone())
    } else {
        // 兼容：无 args 时从 command 字符串中拆分
        let parts: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }
        (parts[0].clone(), parts[1..].to_vec())
    };

    let mut client = McpStdioClient::new(cmd, cmd_args);
    client.set_env(env.map(|e| e.clone()));
    client.get_server_detail()
}

/// 获取 MCP Server 详情（带缓存）
pub async fn get_mcp_server_detail_cached(
    server_name: &str,
    url: Option<&str>,
    command: Option<&str>,
    args: Option<&Vec<String>>,
    env: Option<&HashMap<String, String>>,
    headers: Option<&HashMap<String, String>>,
    force_refresh: bool,
) -> Result<Option<McpServerDetail>, String> {
    let cache = get_mcp_cache();

    // 检查缓存
    if !force_refresh {
        if let Some(detail) = cache.get(server_name).await {
            log::info!("[MCP] Cache hit for '{}'", server_name);
            return Ok(Some(detail));
        }
    }

    // HTTP/SSE server
    if let Some(url) = url {
        if url.starts_with("http://") || url.starts_with("https://") {
            log::info!("[MCP] Fetching via HTTP: url={}", url);
            let detail = fetch_http_mcp_detail(url, headers).await.map_err(|e| {
                log::error!("[MCP] HTTP fetch failed for '{}': {}", server_name, e);
                e
            })?;
            cache.set(server_name.to_string(), detail.clone()).await;
            return Ok(Some(detail));
        }
    }

    // stdio server
    if let Some(command) = command {
        log::info!("[MCP] Fetching via stdio: command={}, args={:?}", command, args);
        let command_str = command.to_string();
        let args_clone = args.cloned();
        let env_clone = env.map(|e| e.clone());
        // 在 tokio runtime 中执行同步操作
        let detail = tokio::task::spawn_blocking(move || {
            fetch_stdio_mcp_detail(&command_str, args_clone.as_ref(), env_clone.as_ref())
        })
            .await
            .map_err(|e| {
                log::error!("[MCP] stdio spawn_blocking failed: {}", e);
                e.to_string()
            })?
            .map_err(|e| {
                log::error!("[MCP] stdio fetch failed for '{}': {}", server_name, e);
                e
            })?;

        cache.set(server_name.to_string(), detail.clone()).await;
        return Ok(Some(detail));
    }

    log::warn!(
        "[MCP] No URL or command for '{}', skipping detail fetch",
        server_name
    );
    Ok(None)
}

