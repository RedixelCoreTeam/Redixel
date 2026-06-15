use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct GameManifest {
    id: String,
    title: String,
    description: String,
    tags: Vec<String>,
}

fn main() {
    let output: Output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .unwrap();

    let stdout: String = String::from_utf8(output.stdout).unwrap();
    let metadata: Value = serde_json::from_str(&stdout).unwrap();
    let mut games_manifest: Vec<GameManifest> = Vec::new();

    if let Some(packages) = metadata.get("packages").and_then(|p: &Value| p.as_array()) {
        for pkg in packages {
            if let Some(game_meta) = pkg.get("metadata").and_then(|m: &Value| m.get("game")) {
                games_manifest.push(GameManifest {
                    id: pkg["name"].as_str().unwrap().to_string(),
                    title: game_meta["title"].as_str().unwrap().to_string(),
                    description: game_meta["description"].as_str().unwrap().to_string(),
                    tags: game_meta["tags"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|t: &Value| t.as_str().unwrap().to_string())
                        .collect(),
                });
            }
        }
    }

    let dist_dir: &Path = Path::new("dist");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir).unwrap();
    }

    let json: String = serde_json::to_string_pretty(&games_manifest).unwrap();
    fs::write(dist_dir.join("games_manifest.json"), json).unwrap();
}
