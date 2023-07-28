// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod tty;
pub(crate) mod payload;
use tty::unix::window::WindowSize;

use nix::libc::{self, pid_t};

// map error to string for front end
macro_rules! js_result {
    ($result: expr) => {
        $result.map_err(|err| err.to_string())
    }
}

#[tauri::command]
fn pty_spawn(app_handle: tauri::AppHandle) -> Result<pid_t, String> {
    let master = js_result!(tty::unix::spawn())?;
    js_result!(tty::unix::poll(master, app_handle))?;
    Ok(master)
}

#[tauri::command]
fn pty_write(fd: pid_t, data: String) -> Result<(), String> {
    js_result!(tty::unix::write(fd, data))
}

#[tauri::command]
fn pty_resize(fd: pid_t, window_size: WindowSize) -> Result<(), String> {
    js_result!(tty::unix::resize(fd, window_size))
}

#[tauri::command]
fn pty_kill(fd: pid_t) {
    unsafe { libc::kill(fd, libc::SIGINT) };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
                pty_spawn,
                pty_write,
                pty_resize,
                pty_kill
            ])
        .run(tauri::generate_context!())
        .expect("error while generating tauri app");
}
