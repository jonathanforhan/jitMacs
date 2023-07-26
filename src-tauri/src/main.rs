// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod tty;

use std::ffi::c_void;
use std::time::Duration;
use nix::libc;
use nix::libc::{pid_t, POLLIN};
use nix::poll::{poll, PollFd, PollFlags};
use tauri::Manager;

#[tauri::command]
fn pty_spawn(app_handle: tauri::AppHandle) -> Result<pid_t, String> {
    let master = tty::unix::spawn().map_err(|err| err.to_string())?;

    std::thread::spawn(move || {
        let flags = PollFlags::from_bits(POLLIN).unwrap();
        let poll_fd = PollFd::new(master, flags);
        let mut i = 0;
        loop {
            println!("loop {i}");
            i += 1;
            let r = poll(&mut [poll_fd], -1);
            if r.is_ok() {
                app_handle.emit_all("pty-event", ()).unwrap();
            } else {
                app_handle.emit_all("pty-event", ()).unwrap();
                std::process::exit(0);
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    Ok(master)
}

#[tauri::command]
async fn pty_read(fd: pid_t) -> Result<String, String> {
    let mut buf = [0; 0x1000];
    let ret = nix::unistd::read(fd, &mut buf);
    match ret {
        Ok(r) => Ok(String::from_utf8_lossy(&buf[..r]).to_string()),
        Err(_) => Err("pty read error".into())
    }
}

#[tauri::command]
async fn pty_write(fd: pid_t, data: String) -> Result<(), String> {
    unsafe {
        let r = libc::write(fd, data.as_ptr() as *const c_void, data.len());

        if r != data.len() as isize {
            return Err("write error".into());
        }
    }
    Ok(())
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
                pty_spawn,
                pty_read,
                pty_write
            ])
        .run(tauri::generate_context!())
        .expect("error while generating tauri app");
}
