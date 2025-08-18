use anyhow::Result;
use serde_json::Value;
use reqwest::Client;
use crate::config::Config;

pub struct Executor {
    cfg: Config,
    http: Client,
}

impl Executor {
    pub fn new(cfg: &Config) -> Self {
        Self {
            cfg: cfg.clone(),
            http: Client::new(),
        }
    }

    pub async fn execute(&self, action: &str, params: &Value) -> Result<Value> {
        match action {
            "think" => {
                let text = params.get("text").and_then(|v| v.as_str()).unwrap_or("");
                Ok(json!({"ok": true, "result": text}))
            }
            "http_request" => {
                let url = params.get("url").and_then(|v| v.as_str()).ok_or_else(|| anyhow::anyhow!("url param required"))?;
                let resp = self.http.get(url).send().await?;
                let status = resp.status().as_u16();
                let body = resp.text().await?.chars().take(2000).collect::<String>();
                Ok(json!({"ok": true, "status": status, "text": body}))
            }
            "shell" => {
                if !self.cfg.allow_shell {
                    return Ok(json!({"ok": false, "error":"shell disabled"}));
                }
                let cmd = params.get("cmd").and_then(|v| v.as_str()).unwrap_or("");
                // very simple safety guard
                let forbidden = ["rm -rf", "shutdown", "reboot", "format"];
                for f in forbidden {
                    if cmd.contains(f) {
                        return Ok(json!({"ok": false, "error":"forbidden command"}));
                    }
                }
                let out = tokio::process::Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .await?;
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                Ok(json!({"ok": true, "retcode": out.status.code(), "out": stdout}))
            }
            _ => Ok(json!({"ok": false, "error": "unknown action"})),
        }
    }
}
