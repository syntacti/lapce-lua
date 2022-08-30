use std::{fs::File, path::PathBuf};

use anyhow::Result;
use flate2::read::GzDecoder;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, InitializeParams, Url},
        Request,
    },
    register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default)]
struct State {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    arch: String,
    os: String,
    configuration: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    language_id: String,
    options: Option<Value>,
}

register_plugin!(State);

const LUA_VER: &str = "5.4.4";

fn initialize(params: InitializeParams) -> Result<()> {
    PLUGIN_RPC.stderr(&format!("starting lua server : {LUA_VER}"));
    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(server_path) = options.get("serverPath") {
            if let Some(server_path) = server_path.as_str() {
                if !server_path.is_empty() {
                    PLUGIN_RPC.start_lsp(
                        Url::parse(&format!("urn:{}", server_path))?,
                        "lua",
                        params.initialization_options,
                    );
                    return Ok(());
                }
            }
        }
    }
    // let arch = match std::env::var("ARCH").as_deref() {
    //     Ok("x86_64") => "x64",
    //     Ok("aarch64") => "arm64",
    //     _ => return Ok(()),
    // };
    // let os = match std::env::var("OS").as_deref() {
    //     Ok("linux") => "linux",
    //     Ok("macos") => "darwin",
    //     Ok("windows") => "win32",
    //     _ => return Ok(()),
    // };
    // let archive_format = match std::env::var("OS").as_deref() {
    //     Ok("linux") => "tar.gz",
    //     Ok("macos") => "tar.gz",
    //     Ok("windows") => "zip",
    //     _ => return Ok(()),
    // };

    // let file_name = format!(
    //     "lua-language-server-3.5.3-{}-{}.{}",
    //     os, arch, archive_format
    // );
    // let file_path = PathBuf::from(&file_name);
    // let gz_path = PathBuf::from(file_name.clone() + ".gz");
    // if !file_path.exists() {
    //     // https://github.com/sumneko/lua-language-server/releases/download/3.5.3/lua-language-server-3.5.3-darwin-arm64.tar.gz
    //     let url = format!(
    //         "https://github.com/sumneko/lua-language-server/releases/download/3.5.3/{}",
    //         file_name
    //     );
    //     let mut resp = Http::get(&url)?;
    //     let body = resp.body_read_all()?;
    //     std::fs::write(&gz_path, body)?;
    //     let mut gz = GzDecoder::new(File::open(&gz_path)?);
    //     let mut file = File::create(&file_path)?;
    //     std::io::copy(&mut gz, &mut file)?;
    //     std::fs::remove_file(&gz_path)?;
    // }

    PLUGIN_RPC.stderr(&format!("LUA_VER: {LUA_VER}"));

    let volt_uri = VoltEnvironment::uri()?;
    let server_path = Url::parse(&volt_uri)?.join("lua-language-server")?;
    PLUGIN_RPC.start_lsp(server_path, "lua", params.initialization_options);
    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                let _ = initialize(params);
            }
            _ => {}
        }
    }
}
