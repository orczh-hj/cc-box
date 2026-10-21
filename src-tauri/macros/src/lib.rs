//! CC-Box 命令属性宏
//!
//! [`cc_command`] 是写命令的单一广播出口：函数成功（Ok）返回前自动
//! `emit_state_changed(section)`，桌面 Tauri invoke 与远程 rpc_dispatch
//! 调用同一函数，广播口径天然一致——替代函数体内手动 emit。

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, ItemFn, Lit, Meta};

/// 写命令声明宏：
/// - `#[cc_command(write = "section")]`：写命令，成功返回前自动广播
///   state-changed(section)。section 字符串与 `rpc_dispatch::registry!` 的
///   `write(Sect)` 声明一一对应（守卫测试对拍两侧一致）
/// - `#[cc_command(write)]`：写但不广播（已有专用事件或无跨端 UI 状态），
///   仅作源码标记（与 registry! 的 write(None) 对拍）
///
/// 必须置于 `#[tauri::command]` 之上：本宏先展开包装函数体并原样保留内层
/// 属性，`#[tauri::command]` 随后照常生成 IPC 垫片。
/// 要求函数返回 `Result`（`is_ok` 判定后才广播）。
#[proc_macro_attribute]
pub fn cc_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let section = match parse_write_attr(attr) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    let func = syn::parse_macro_input!(item as ItemFn);
    match section {
        Some(section) => {
            let attrs = &func.attrs;
            let vis = &func.vis;
            let sig = &func.sig;
            let block = &func.block;
            quote! {
                #(#attrs)*
                #vis #sig {
                    // 原函数体包成 async 块执行（`?`/提前 return 语义不变），
                    // 出口单点广播：失败不广播，成功广播后原样返回
                    let __cc_result = async #block .await;
                    if __cc_result.is_ok() {
                        crate::remote_state::emit_state_changed(
                            crate::rpc_dispatch::StateSection::#section
                        );
                    }
                    __cc_result
                }
            }
            .into()
        }
        // write（无值）：写但不广播，函数原样输出（属性仅作守卫对拍标记）
        None => quote!(#func).into(),
    }
}

/// 解析属性：`write = "config"` → Some(Config 变体 Ident)；`write` → None
fn parse_write_attr(attr: TokenStream) -> syn::Result<Option<syn::Ident>> {
    if attr.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "cc_command 需要声明写语义：write = \"section\"（广播）或 write（不广播）",
        ));
    }
    let meta = syn::parse::<Meta>(attr)?;
    match &meta {
        Meta::NameValue(nv) if nv.path.is_ident("write") => {
            let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value else {
                return Err(syn::Error::new_spanned(
                    &nv.value,
                    "write 的值必须是字符串字面量，如 write = \"config\"",
                ));
            };
            let name = s.value();
            if name.is_empty() || !name.chars().all(|c| c.is_ascii_lowercase()) {
                return Err(syn::Error::new_spanned(
                    s,
                    "section 必须是非空小写字符串（StateSection::as_str 同款）",
                ));
            }
            Ok(Some(syn::Ident::new(
                &section_variant_name(&name),
                s.span(),
            )))
        }
        Meta::Path(p) if p.is_ident("write") => Ok(None),
        _ => Err(syn::Error::new_spanned(
            meta,
            "仅支持 write = \"section\" 或 write",
        )),
    }
}

/// section 字符串 → 枚举变体名：首字母大写（"config" → "Config"、"mcp" → "Mcp"）。
/// 转换规则的锚定测试在 cc-box 侧 tests/state_broadcast.rs（宏出口行为测试
/// 隐式验证 "config" → StateSection::Config）——本 crate 单测因 src-tauri/
/// .cargo/config.toml 的 runner 相对路径（test-runner.ps1）无法在此目录运行
fn section_variant_name(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
