//! 生成 gpui-shell 类型声明（gpui-kit.d.ts）到指定目录。
//!
//! 组件 catalog 与 Navop 宿主一致：`gpui_component_shell::components()`
//!（Button/Input/TextView… 全量声明进入 `gpui-component` 模块）。

use std::path::PathBuf;

fn main() {
    let out_dir: PathBuf = std::env::args()
        .nth(1)
        .expect("usage: gen-gpui-typings <out-dir>")
        .into();
    let components = gpui_component_shell::components().expect("component registry");
    let declarations = gpui_shell::type_declarations(&components);
    std::fs::create_dir_all(&out_dir).expect("create out dir");
    let file = out_dir.join("gpui-kit.d.ts");
    std::fs::write(&file, declarations).expect("write gpui-kit.d.ts");
    println!("written {}", file.display());
}
