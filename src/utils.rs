use reqwest::header;
use std::fs::{self, File};
use std::io::{self, Write};
use std::time::Instant;
use tempfile::NamedTempFile;

use crate::config::Mirror;
use futures_util::StreamExt;
use humansize::{make_format, BINARY};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;

    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap().progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));

    // 创建一个临时文件并获取文件句柄
    let temp_file = NamedTempFile::new().unwrap();
    let mut temp_file_handle = File::create(&temp_file).unwrap();

    // download chunks
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;

        temp_file_handle
            .write_all(&chunk)
            .or(Err("Error while writing to file".to_string()))?;

        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", url, path));
    Ok(())
}

pub fn speed_test(mirror: &Mirror, timeout: u32) -> Result<f32, Box<dyn std::error::Error>> {
    let url = mirror.test.as_str();
    // 创建一个临时文件并获取文件句柄
    let temp_file = NamedTempFile::new()?;
    let mut temp_file_handle = File::create(&temp_file)?;

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/zip".parse().unwrap());
    headers.insert("User-Agent", "rust".parse().unwrap());

    use std::time::Duration;

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    let formatter = make_format(BINARY);

    // 发送 HTTP GET 请求并获取响应
    let start_time = Instant::now();

    let mut res = client
        .get(url)
        // 设置超时时间，防止网络问题造成测试时间过长
        .timeout(Duration::from_secs(timeout as u64))
        .send()?;
    let total_size = res.content_length().unwrap_or(0);

    let size = formatter(total_size);
    let name = mirror.name.clone();
    println!("=============================");
    println!("Mirror: {name}");
    println!("File Size: {size}");

    let _ = io::copy(&mut res, &mut temp_file_handle);

    let end_time = start_time.elapsed();

    // 打印下载时间和下载速度
    let file_length = fs::metadata(temp_file)?.len();
    let duration_in_millis = end_time.as_secs_f32();
    let download_speed = (file_length as f32) / duration_in_millis;

    let speed = formatter(download_speed as u64);
    let file_size = formatter(file_length);
    let duration = round2(duration_in_millis);
    println!("Downloaded {file_size} in {duration:.2} s ({speed}/s)");

    Ok(download_speed)
}

/// 四舍五入，保留2位小数
pub fn round2(num: f32) -> f32 {
    (num * 100.0).round() / 100.0
}
