// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod tty;

use nix::errno::Errno;
use nix::libc::{self, pid_t};

#[tauri::command]
fn pty_spawn(app_handle: tauri::AppHandle) -> Result<pid_t, String> {
    let master = tty::unix::spawn().map_err(|err| err.to_string())?;
    tty::unix::poll(master, app_handle).map_err(|err| err.to_string())?;

    Ok(master)
}

#[tauri::command]
fn pty_read(fd: pid_t) -> Result<String, String> {
    use nix::unistd::read;

    let mut buf = [0; 0x1000];

    return match read(fd, &mut buf) {
        Ok(r) => Ok(String::from_utf8_lossy(&buf[..r]).to_string()),
        Err(e) => {
            if e == Errno::EAGAIN { return Ok("".into()); }
            Err(e.to_string())
        },
    }
}

#[tauri::command]
fn pty_write(fd: pid_t, data: String) -> Result<(), String> {
    use nix::unistd::write;

    match write(fd, data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn pty_kill(fd: pid_t) {
    unsafe { libc::kill(fd, libc::SIGINT) };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
                pty_spawn,
                pty_read,
                pty_write,
                pty_kill
            ])
        .run(tauri::generate_context!())
        .expect("error while generating tauri app");
}
