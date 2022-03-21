use std::{env, process::Command};

use crate::types::DockerStats;
use anyhow::Result;

use super::{DataCollector, DataCollectorError};

impl DataCollector {
  pub fn get_docker_stats(&mut self) -> Result<Vec<DockerStats>> {
    let command = Command::new(if env::consts::OS == "linux" { "/usr/bin/docker" } else { "docker" })
      .args([
        "stats",
        "--no-stream",
        "--format",
        "{\"container\":\"{{ .Container }}\",\"name\":\"{{ .Name }}\",\"memory\":{\"raw\":\"{{ .MemUsage }}\",\"percent\":\"{{ .MemPerc }}\"},\"cpu\":\"{{ .CPUPerc }}\"}",
      ])
      .output()?;

    let output_string = String::from_utf8(command.stdout)?.trim().to_string();
    let mut stats: Vec<DockerStats> = Vec::new();

    for line in output_string.split("\n") {
      let stat = serde_json::from_str(line);
      if let Ok(stat) = stat {
        stats.push(stat);
      }
    }

    return match command.status.code() {
      Some(0) => Ok(stats),
      Some(_code) => Err(DataCollectorError::NoDockerStats)?,
      None => Err(DataCollectorError::NoDockerStats)?,
    };
  }
}
