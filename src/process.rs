use napi_derive::napi;
use std::collections::HashMap;
use tokio::process::Command;
use std::process::Stdio;

#[napi(object)]
pub struct SpawnOptions {
    pub command: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: Option<HashMap<String, String>>,
}

#[napi(object)]
pub struct ProcessStatus {
    pub pid: u32,
    pub success: bool,
    pub code: Option<i32>,
}

#[napi]
pub async fn exec_command(command: String, args: Vec<String>) -> napi::Result<ProcessStatus> {
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| napi::Error::from_reason(format!("Failed to spawn process: {}", e)))?;

    let status = child.wait().await
        .map_err(|e| napi::Error::from_reason(format!("Failed to wait for process: {}", e)))?;

    Ok(ProcessStatus {
        pid: child.id().unwrap_or(0),
        success: status.success(),
        code: status.code(),
    })
}

#[napi]
pub async fn spawn_with_options(options: SpawnOptions) -> napi::Result<ProcessStatus> {
    let mut cmd = Command::new(options.command);
    cmd.args(options.args);
    
    if let Some(cwd) = options.cwd {
        cmd.current_dir(cwd);
    }
    
    if let Some(env) = options.env {
        cmd.envs(env);
    }
    
    let mut child = cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| napi::Error::from_reason(format!("Failed to spawn process: {}", e)))?;

    let status = child.wait().await
        .map_err(|e| napi::Error::from_reason(format!("Failed to wait for process: {}", e)))?;

    Ok(ProcessStatus {
        pid: child.id().unwrap_or(0),
        success: status.success(),
        code: status.code(),
    })
}
