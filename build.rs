use std::env;
use std::path::PathBuf;

fn main() {
    // 1. ビルド済みライブラリ（libsqlite3.a）が置かれているディレクトリを指定
    // ここではプロジェクトルートの「vendor/lib」フォルダを想定
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = PathBuf::from(manifest_dir).join("vendor").join("lib");

    // 2. Cargo に検索パスを通知
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // 3. 静的ライブラリとして sqlite3 をリンク
    println!("cargo:rustc-link-lib=static=wsqlite3");

    // 4. sqlite-wasm-rs のビルドスクリプト（libsqlite3-sys）にパスを伝播
    // これにより、依存先の自動ビルドや自動ダウンロードをスキップさせます
    println!("cargo:DEP_SQLITE_WASM_RS_LIBS_DIR={}", lib_dir.display());
    println!("cargo:DEP_SQLITE_3_SYS_LIBS_DIR={}", lib_dir.display());

    // 5. ライブラリが変更されたら build.rs を再実行する
    println!("cargo:rerun-if-changed=vendor/lib/libwsqlite3.a");
    println!("cargo:rerun-if-changed=build.rs");
}
