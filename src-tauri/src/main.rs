/*
 * @Author: yaoxingpu yaoxpu@163.com
 * @Date: 2023-10-09 15:44:10
 * @LastEditors: yaoxingpu yaoxpu@163.com
 * @LastEditTime: 2023-10-09 15:52:13
 * @FilePath: /vue-vben-admin/src-tauri/src/main.rs
 * @Description: 
 * 
 */
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}
