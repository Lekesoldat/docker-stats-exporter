use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DockerContainerStats {
    pub container: String,
    pub cpuPerc: String,
    pub memPerc: String,
    pub netIO: String,
}

const DOCKER_FORMAT: &str = r#"{"container": "{{.Name}}", "cpuPerc": "{{.CPUPerc}}", "memPerc": "{{.MemPerc}}", "netIO": "{{.NetIO}}"}"#;

pub fn stats() -> Result<Vec<DockerContainerStats>> {
    let output = Command::new("docker")
        .args(&["stats", "--format", DOCKER_FORMAT, "--no-stream"])
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if !output.status.success() {
        eprintln!("`docker stats` returned non-zero exit code with output: \n{}\n{}", stdout, stderr);
        return Err(anyhow!("Docker stats command did bad :("));
    }

    let json_list_content = stdout.lines().collect::<Vec<&str>>().join(",");
    let json_string = format!("[{}]", json_list_content);

    let result = serde_json::from_str::<Vec<DockerContainerStats>>(json_string.as_str())?;
    Ok(result)
}

