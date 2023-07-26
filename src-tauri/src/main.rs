// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod tty;

use std::ffi::c_void;
use std::time::Duration;
use nix::libc::{self, pid_t, POLLIN};
use nix::poll::{poll, PollFd, PollFlags};
use tauri::Manager;

#[tauri::command]
fn pty_spawn(app_handle: tauri::AppHandle) -> Result<pid_t, String> {
    let master = tty::unix::spawn().map_err(|err| err.to_string())?;

    // TODO move to unix dir
    // poll the newly created fd
    std::thread::spawn(move || {
        let flags = PollFlags::from_bits(POLLIN).unwrap();
        let poll_fd = PollFd::new(master, flags);
        let mut i = 0;
        loop {
            println!("loop {i}");
            i += 1;
            match poll(&mut [poll_fd], -1) {
                Ok(_) => app_handle.emit_all("pty-event", ()).unwrap(),
                Err(_) => return
            };
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    Ok(master)
}

#[tauri::command]
fn pty_read(fd: pid_t) -> Result<String, String> {
    use nix::unistd::read;

    let mut buf = [0; 0x1000];

    match read(fd, &mut buf) {
        Ok(r) => Ok(String::from_utf8_lossy(&buf[..r]).to_string()),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
fn pty_write(fd: pid_t, data: String) -> Result<(), String> {
    use nix::unistd::write;

    match write(fd, data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
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
