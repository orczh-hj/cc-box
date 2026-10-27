# cc-box-relay 部署指南（orczh.top）

中继服务器：公网 VPS 上运行的透明路由 hub，让手机在局域网之外访问桌面 CC-Box。桌面端只出站连接（零入站端口）。

> **改远程相关代码（remote/、远程前端、远程 Rust）后，先在本地跑通中继链路再部署**：一条命令 `npm run dev:relay`（自动探测局域网 IP 并拉起 relay + tauri dev），手机扫配对码验证——详见 [第 5 节](#5-本地测试开发环境)。

## 1. 构建（Windows 交叉编译 Linux）

服务器（阿里云 Linux 3 x86_64，2GB 内存）无 Rust 工具链且内存不足以编译依赖树——**本机交叉编译静态二进制**上传：

```bash
rustup target add x86_64-unknown-linux-musl
cd remote/relay
RUSTFLAGS="-C linker=rust-lld" cargo build --release --target x86_64-unknown-linux-musl
# 产物 target/x86_64-unknown-linux-musl/release/cc-box-relay（static-pie，~4.6MB，服务器零依赖）
```

要点：

- **mingw 的 `ld.exe` 无法链接 musl 目标**（`collect2: error: ld returned 1`），用 `rust-lld`（工具链自带，零安装）
- relay 依赖全为纯 Rust（tokio/axum/sha2），musl 静态编译无 C 工具链要求

## 2. 部署（orczh.top:8800，nginx TLS 终止模式）

**生产现状（2026-10-20 / 磁盘分离 2026-10-25）**：`https://orczh.top:8800`——**手机浏览器对 http 链接强制升级 https**（HTTPS-First/HSTS，实测 4G Chrome 报 `ERR_SSL_PROTOCOL_ERROR`），纯 HTTP 端口手机打不开；由 nginx 在 8800 做 TLS 终止（复用主站证书），反代内部 relay。**Web UI 已与 relay 二进制分离**：`CC_BOX_RELAY_WEB_DIR=/opt/cc-box/web` 指向磁盘目录，前端更新只需 scp dist，免重编、免重启。

```bash
scp target/x86_64-unknown-linux-musl/release/cc-box-relay root@orczh.top:/usr/local/bin/
# scp 偶发传完不退出——不等命令返回，远端 md5sum 与本地比对确认
ssh root@orczh.top "md5sum /usr/local/bin/cc-box-relay"
```

systemd 服务（`/etc/systemd/system/cc-box-relay.service`，实际部署态——只听本机 8801，TLS 由 nginx 终止）：

```ini
[Unit]
Description=CC-Box Relay
After=network.target

[Service]
ExecStart=/usr/local/bin/cc-box-relay
Environment=CC_BOX_RELAY_BIND=127.0.0.1
Environment=CC_BOX_RELAY_PORT=8801
# Web UI 磁盘目录（relay 不内嵌 UI，必配；前端更新 scp dist 即生效）
Environment=CC_BOX_RELAY_WEB_DIR=/opt/cc-box/web
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
```

nginx（`/etc/nginx/conf.d/cc-box-relay.conf`，复用主站证书）：

```nginx
server {
    listen 8800 ssl;
    server_name orczh.top;
    ssl_certificate "/root/.ssl/orczh.top.pem";
    ssl_certificate_key "/root/.ssl/orczh.top.key";

    location / {
        proxy_pass http://127.0.0.1:8801;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 3600s;
    }
}
```

```bash
systemctl daemon-reload && systemctl enable --now cc-box-relay
```

**踩坑记录**：

- **纯 HTTP 公网端口手机打不开**：手机浏览器把 `http://` 强制升级 `https://`（HTTPS-First），relay 收到 TLS ClientHello 只能 RST，页面报 `ERR_SSL_PROTOCOL_ERROR`——公网入口必须 TLS（本方案 nginx 8800 ssl）
- **不要用 `cc-box-relay --version` 做 smoke test**：main 无参数解析，任何参数都会把服务器启起来；经 ssh 断开后成孤儿进程占住端口，导致 systemd 服务 Restart 循环（AddrInUse）。验证一律走 `systemctl status` + `ss -tlnp`
- **修改 systemd 服务文件后必须 `daemon-reload`**，且先重启 relay（让出端口）再重启 nginx（bind 顺序冲突会导致 nginx 起不来）
- 阿里云**安全组**需放行 8800/tcp（控制台操作，服务器内 firewalld 默认未启用）
- 升级流程：scp 覆盖二进制 → `systemctl restart cc-box-relay`（桌面自动重连；已配对手机凭 token v2 自动恢复，无需重扫——仅升级到 v2 版 relay 时存量手机各重扫一次）

**验证**（公网 TLS 可达性 + WS 握手）：

```bash
curl -s -o /dev/null -w "%{http_code}" https://orczh.top:8800/          # 200
curl -s -o /dev/null -w "%{http_code}" -H "Upgrade: websocket" -H "Connection: Upgrade" \
  -H "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==" -H "Sec-WebSocket-Version: 13" \
  https://orczh.top:8800/ws/device                                       # 101
```

## 3. nginx 反代（备选方案，当前未使用）

> 当前生产为裸端口直连（第 2 节）。如需改走 wss://（如未来接入 CDN/共享受 443），按下述子域反代模式，并将 relay 的 `CC_BOX_RELAY_BIND` 改回默认 `127.0.0.1`。

```nginx
server {
    server_name cc.orczh.top;

    location / {
        proxy_pass http://127.0.0.1:8800;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 3600s;   # WS 长连接
    }
}
```

```bash
sudo certbot -d cc.orczh.top   # 签发证书并自动配 443
```

## 3.5 日常更新部署（复用手册）

> 本节是**开发过程中的更新操作**；正式版本发布的完整清单（发布前检查/顺序/验证/用户提示）见 [docs/release-process.md](../../docs/release-process.md)「远程组件发布」。

改完代码后按改动类型执行（可叠加）：

| 改动 | 8747 局域端 | 桌面 Tauri | 中继 orczh.top:8800 |
|---|---|---|---|
| **仅前端**（src/） | `npm run build` 即时生效（rust-embed debug 实时读 dist） | vite HMR 自动 | **scp dist 即生效，无需重编/重启**（生产已配置磁盘目录，见下方「仅前端更新」） |
| **Rust 桌面**（src-tauri/） | 重启 `npm run tauri:dev` | 同左 | 无需动 relay |
| **relay**（remote/relay/） | 无 | 无 | 交叉编译 + scp + 重启 |

> **票据下载（远程文件直链）**：中继侧**零改动**——票据页面走现有 `/?dl=` 静态入口与 WS 透传（`<device_id>.<ticket>` 复用 token v2 前缀路由），relay 二进制无需重编/重启；只需按下方「仅前端更新」scp 新 dist。桌面必须同步升级（提供 `/dl` 与 `file_download_ticket`），否则前端自动降级为旧 Blob 分块下载。

**仅前端更新（免重编 relay，生产已启用）**——relay 二进制**不内嵌 UI**，`CC_BOX_RELAY_WEB_DIR=/opt/cc-box/web` 指向的磁盘目录是 UI 的唯一来源（systemd 已配置；未配置或磁盘 miss 即 404）：

```bash
# 仓库根：构建最新前端并上传（无需重编/重启 relay，手机刷新页面即得新 UI）
npm run build
timeout 90 scp -r dist/* root@orczh.top:/opt/cc-box/web/
curl -s https://orczh.top:8800/ | grep -o 'assets/index-[^"]*'   # hash 应为本次新构建
```

> **生产配置记录（2026-10-25）**：`/opt/cc-box/web` 已创建并放入 dist，systemd 已加 `Environment=CC_BOX_RELAY_WEB_DIR=/opt/cc-box/web`（原 unit 备份为 `cc-box-relay.service.bak`），并用探针文件验证磁盘优先生效。新环境启用同样只需：`mkdir -p /opt/cc-box/web`、unit 加上述 `Environment=`、`systemctl daemon-reload && systemctl restart cc-box-relay`。桌面端对称支持 `CC_BOX_WEB_DIR`（release 桌面同样获得免重编更新通道；debug 本就实时读 dist）。

**relay 二进制更新一条龙**（仅 `remote/relay/` 或 `remote/web-assets/` 的**服务逻辑**改动时需要；纯前端内容改动不要跑，直接 scp dist。脚本不再构建前端、编译也不依赖 dist。Windows Git Bash，在仓库根目录）：

```bash
# 构建 → musl 交叉编译 → scp .new + md5 校验 → 确认后切换重启 → 三查全自动
npm run deploy:relay          # 加 --yes 跳过切换确认（重启短暂断开，手机自动重连恢复）
```

脚本实现 `scripts/deploy-relay.mjs`（全程直连无需代理；切换前确认是因为重启会断开在途连接）。**v2 版 relay 首次升级务必先发桌面新版再部署 relay**（新版桌面兼容新旧 relay；旧桌面在新 relay 下走 fallback 但无 hello 校验加固）。脚本不可用时的手动等价流程：

```bash
# 1. 前端最新产物（即使没改前端也建议跑，保证嵌入版本一致）
npm run build

# 2. 交叉编译（嵌入 dist；rust-lld 是 musl 关键，mingw ld 链不了）
cd remote/relay
RUSTFLAGS="-C linker=rust-lld" cargo build --release --target x86_64-unknown-linux-musl

# 3. 上传 + 切换 + 重启（scp 可能传完不退出，不等返回、远端 md5 校验）
BIN=target/x86_64-unknown-linux-musl/release/cc-box-relay
timeout 90 scp $BIN root@orczh.top:/usr/local/bin/cc-box-relay.new
ssh root@orczh.top 'cd /usr/local/bin && mv cc-box-relay cc-box-relay.old && mv cc-box-relay.new cc-box-relay && chmod +x cc-box-relay && systemctl restart cc-box-relay && sleep 2 && systemctl is-active cc-box-relay'

# 4. 验证（三查）
md5sum $BIN                                    # 与远端 md5sum 比对
curl -s https://orczh.top:8800/ | grep -o 'assets/index-[^"]*'   # hash 应为本次新构建
curl -s "https://orczh.top:8800/$(curl -s https://orczh.top:8800/ | grep -o 'assets/index-[^"]*' | head -1)" | grep -c <新功能特征字符串>   # 新代码确已嵌入
```

**升级后必读**：

- relay 重启 = 内存态（在线连接/配对码）清空：桌面端自动重连（日志 `relay registered`）；**已配对手机凭 token v2 自动重连恢复，无需重扫**（token 校验在桌面，relay 不存凭证表）——仅升级到本版 relay 时，存量 v1 token 手机需各重扫一次（一次性迁移）
- 手机/浏览器端要**刷新页面**拿新 bundle（浏览器缓存旧 JS 是"改了没反应"的头号原因）
- 桌面日志验证：`~/.cc-box/logs/` 里看 `relay connected/registered` 与新配对码

## 4. 桌面端接入（中继 = 默认远程方式，零配置）

中继地址**内置默认 `wss://orczh.top:8800`**（`remote_client::DEFAULT_RELAY_URL`，TLS 经 nginx 终止）——用户点「开启远程访问」即自动连接中继并显示配对码（`https://orczh.top:8800/#pair=…`，手机可直接打开），无需任何配置；「局域访问」是复选项（默认关闭）。

环境变量 `CC_BOX_RELAY_URL` 仅作**本地调试覆盖**（连本地 relay 测试，仅本次运行生效、不落盘）：

```bash
# 本地调试（连本机 relay）
CC_BOX_RELAY_URL=192.168.5.105:8800 npm run tauri:dev
```

- 地址归一化：显式 scheme 优先（`https://` → `wss://`、`http://` → `ws://`）；裸地址默认补 `ws://`（免证书端口直连 relay 进程）
- 配对码一次性、5 分钟有效；**过期/消费后按需自动重申**（面板查询触发 `pair.create`，2s 轮询拿到新码），无码等待期间面板回落局域直连视图
- config 的 `relay_url` 是历史字段，**不再读取**（避免旧残留覆盖内置默认）
- 手机扫码完成配对 → token v2（`<device_id>.<secret>`，存手机 localStorage；secret 固化在桌面 `~/.cc-box/config.json` 的 `pairedSecrets`）长期有效，之后打开即连；**中继重启后自动重连恢复，无需重新配对**（重新配对仅在桌面「刷新密钥」后需要）

## 5. 本地测试（开发环境）

| 环境变量 | 作用端 | 说明 |
|---|---|---|
| `CC_BOX_RELAY_BIND=0.0.0.0` | relay | 真机测试必须：默认 `127.0.0.1`（生产经 nginx 反代），手机无法访问 |
| `CC_BOX_RELAY_URL=<IP>:8800` | 桌面端 | 连中继；**用局域网 IP（如 `192.168.5.105:8800`）而非 `127.0.0.1`**，否则手机扫出的配对 URL 打不开 |
| `CC_BOX_RELAY_PORT=8800` | relay | 监听端口（默认 8800） |
| `CC_BOX_RELAY_WEB_DIR=<dir>` | relay | Web UI 磁盘目录（**relay 唯一 UI 来源，必配**；生产已设为 `/opt/cc-box/web`，仅前端更新 scp dist 即生效，见 3.5 节） |
| `CC_BOX_WEB_DIR=<dir>` | 桌面端 | 桌面 8747 对称的 Web UI 磁盘目录（release 桌面免重编更新；debug 本就实时读 dist） |

完整本地链路一条命令（`npm run dev:relay` = `scripts/dev-relay.mjs`：自动探测局域网 IP，按下述环境变量同时拉起 relay 与桌面端，任一进程退出全部退出；需单独手动起进程时才直接用环境变量）：

```bash
npm run dev:relay
# 手动等效（两个终端）：
#   cd remote/relay && CC_BOX_RELAY_BIND=0.0.0.0 cargo run
#   CC_BOX_RELAY_URL=192.168.5.105:8800 npm run tauri:dev
```

要点：

- **远程面板是显式开关**：打开面板只查状态，点「开启远程访问」才启动（安全阻断，停止后重启软件不会自动恢复）
- **改前端后需 `npm run build`**：8747 由 rust-embed 实时读 `dist/`（debug 模式），浏览器端刷新页面拿新版本——**手机不能直连 vite dev server (:1420) 当远程 UI**：浏览器模式需要 `/ws` WebSocket 端点（remote-transport 连 `ws://host/ws`），只有 8747 的 axum 服务器提供，vite dev server 没有
- **多实例宿主制**：先开远程的实例为宿主（占 8747）；后开实例自动探测到宿主（`remote_server::probe_host`）后不 bind、不连中继（防同 `device_id` 互踢），面板直接显示宿主的 URL/二维码（token 共享自 config）；宿主退出后其他实例再开即可无缝接管
- 真机走查条目见 `remote/docs/manual-test-cases.md`（U 系列）

## 6. 运维要点

- **内存态**：在线连接路由表、配对码全在内存，无凭证存储（token v2 `<device_id>.<secret>` 由桌面校验 secret）。中继重启后：桌面自动重连（指数退避）；**已配对手机同样自动重连恢复**——重新配对仅在桌面「刷新密钥」后需要
- **设备身份**：`device_id`/`device_key` 存于桌面 `~/.cc-box/config.json`，首次连接时生成；中继校验 key 与 device_id 绑定关系，防止伪造
- **透传原则**：中继不解析应用层 payload——升级桌面/手机端协议无需动中继
- 日志：`journalctl -u cc-box-relay -f`
