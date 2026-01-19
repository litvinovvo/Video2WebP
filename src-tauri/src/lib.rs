use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionSettings {
    pub fps: u32,
    pub max_dimension: u32,
    pub max_frames: u32,
    pub quality: u32,
    pub compression_level: u32,
    pub overwrite: bool,
}

impl Default for ConversionSettings {
    fn default() -> Self {
        Self {
            fps: 10,
            max_dimension: 540,
            max_frames: 30,
            quality: 55,
            compression_level: 6,
            overwrite: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResult {
    pub input_path: String,
    pub output_path: String,
    pub output_size: Option<u64>,
    pub success: bool,
    pub error: Option<String>,
}

#[tauri::command]
async fn convert_video(
    app: AppHandle,
    input_path: String,
    output_path: Option<String>,
    settings: ConversionSettings,
) -> Result<ConversionResult, String> {
    let input = PathBuf::from(&input_path);

    // Generate output path if not provided
    let output = match output_path {
        Some(p) => PathBuf::from(p),
        None => {
            let mut out = input.clone();
            out.set_extension("webp");
            out
        }
    };

    // Check if output already exists (only if overwrite is false)
    if !settings.overwrite && output.exists() {
        return Ok(ConversionResult {
            input_path,
            output_path: output.to_string_lossy().to_string(),
            output_size: None,
            success: false,
            error: Some("Output file already exists".to_string()),
        });
    }

    // Build FFmpeg filter
    let vf_filter = format!(
        "fps={},scale='if(gt(iw,ih),{},-2)':'if(gt(iw,ih),-2,{})':flags=lanczos",
        settings.fps, settings.max_dimension, settings.max_dimension
    );

    let output_str = output.to_string_lossy().to_string();

    // Use the bundled FFmpeg sidecar
    let sidecar_command = app.shell().sidecar("ffmpeg").map_err(|e| e.to_string())?;

    let overwrite_flag = if settings.overwrite { "-y" } else { "-n" };

    let result = sidecar_command
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            overwrite_flag,
            "-i",
            &input_path,
            "-an",
            "-vf",
            &vf_filter,
            "-frames:v",
            &settings.max_frames.to_string(),
            "-c:v",
            "libwebp",
            "-lossless",
            "0",
            "-preset",
            "photo",
            "-qscale:v",
            &settings.quality.to_string(),
            "-compression_level",
            &settings.compression_level.to_string(),
            "-loop",
            "0",
            "-fps_mode",
            "passthrough",
            &output_str,
        ])
        .output()
        .await;

    match result {
        Ok(output_result) => {
            if output_result.status.success() {
                // Get file size
                let size = std::fs::metadata(&output).map(|m| m.len()).ok();

                Ok(ConversionResult {
                    input_path,
                    output_path: output_str,
                    output_size: size,
                    success: true,
                    error: None,
                })
            } else {
                let stderr = String::from_utf8_lossy(&output_result.stderr).to_string();
                Ok(ConversionResult {
                    input_path,
                    output_path: output_str,
                    output_size: None,
                    success: false,
                    error: Some(if stderr.is_empty() {
                        "FFmpeg conversion failed".to_string()
                    } else {
                        stderr
                    }),
                })
            }
        }
        Err(e) => Ok(ConversionResult {
            input_path,
            output_path: output_str,
            output_size: None,
            success: false,
            error: Some(format!("Failed to execute FFmpeg: {}", e)),
        }),
    }
}

#[tauri::command]
async fn check_ffmpeg(app: AppHandle) -> Result<bool, String> {
    // Check if the bundled FFmpeg sidecar exists
    match app.shell().sidecar("ffmpeg") {
        Ok(cmd) => match cmd.args(["-version"]).output().await {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        },
        Err(_) => Ok(false),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![convert_video, check_ffmpeg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
