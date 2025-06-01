use std::{
    fs,
    path::{Path, PathBuf},
};

/// protoファイルをコンパイルして生成されたファイルをリネームする
fn compile_proto(
    proto_path: &str,
    out_dir: &PathBuf,
    old_file_name: &str,
    new_file_name: &str,
) -> Result<(), String> {
    // 出力ディレクトリの作成
    fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;

    // protoファイルのコンパイル
    prost_build::Config::new()
        .out_dir(out_dir)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&[proto_path], &["src"])
        .map_err(|e| e.to_string())?;

    // 生成されたファイルのリネーム
    let old_path = Path::new(out_dir).join(old_file_name);
    let new_path = Path::new(out_dir).join(new_file_name);
    fs::rename(&old_path, &new_path).map_err(|e| e.to_string())?;

    // 変更監視対象のファイルを追加
    println!("cargo:rerun-if-changed={}", proto_path);

    Ok(())
}

/// auth/login のprotoファイルを処理する
fn compile_auth_login() -> Result<(), String> {
    let out_dir = PathBuf::from("src/generated/api/auth");
    compile_proto(
        "src/api/auth/login/login.proto",
        &out_dir,
        "api.auth.login.rs",
        "login.rs",
    )
}

/// memo/get_count_by_date のprotoファイルを処理する
fn compile_memo_get_count_by_date() -> Result<(), String> {
    let out_dir = PathBuf::from("src/generated/api/memo");
    compile_proto(
        "src/api/memo/get_count_by_date.proto",
        &out_dir,
        "api.memo.get_count_by_date.rs",
        "get_count_by_date.rs",
    )
}

fn compile_memo_get_data() -> Result<(), String> {
    let out_dir = PathBuf::from("src/generated/api/memo");
    compile_proto(
        "src/api/memo/get_data.proto",
        &out_dir,
        "api.memo.get_data.rs",
        "get_data.rs",
    )
}

fn compile_memo_create() -> Result<(), String> {
    let out_dir = PathBuf::from("src/generated/api/memo");
    compile_proto(
        "src/api/memo/create.proto",
        &out_dir,
        "api.memo.create.rs",
        "create.rs",
    )
}

fn compiler_forgot_password() -> Result<(), String> {
    let out_dir = PathBuf::from("src/generated/api/auth/forgot_password");
    compile_proto(
        "src/api/auth/forgot_password/forgot_password.proto",
        &out_dir,
        "api.auth.forgot_password.rs",
        "forgot_password.rs",
    )
}

fn compile_user_register() -> Result<(), String> {
    let out_dir = PathBuf::from("src/generated/api/user");
    compile_proto(
        "src/api/user/register.proto",
        &out_dir,
        "api.user.register.rs",
        "register.rs",
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // エラーがあった場合は返す
    compile_auth_login().map_err(|e| {
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>
    })?;
    compile_memo_get_count_by_date().map_err(|e| {
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>
    })?;

    compile_memo_get_data().map_err(|e| {
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>
    })?;
    compile_memo_create().map_err(|e| {
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>
    })?;

    compile_user_register().map_err(|e| {
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>
    })?;

    compiler_forgot_password().map_err(|e| {
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>
    })?;

    Ok(())
}
