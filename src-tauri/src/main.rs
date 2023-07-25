// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::ffi::c_void;
use std::process::{Command, Stdio};
use std::os::unix::io::{FromRawFd};
use std::os::unix::process::CommandExt;
use std::time::Duration;

use nix::pty::openpty;
use nix::libc::{self, pid_t, POLLIN, write};
use nix::unistd::read;
use nix::poll::{poll, PollFd, PollFlags};
use tauri::{AppHandle, Manager};

#[tauri::command]
fn pty_spawn(app_handle: AppHandle) -> Result<pid_t, String> {
    let ends = openpty(None, None).unwrap();
    let (master, slave) = (ends.master, ends.slave);

    let mut builder = Command::new("/bin/bash");

    builder.env("LOGNAME", "jon");
    builder.env("USER", "jon");
    builder.env("SHELL", "/bin/bash");
    builder.env("HOME", "/home/jon");

    builder.current_dir("/home/jon");
    builder.stdin(unsafe { Stdio::from_raw_fd(slave) });
    builder.stderr(unsafe { Stdio::from_raw_fd(slave) });
    builder.stdout(unsafe { Stdio::from_raw_fd(slave) });

    unsafe {
        builder.pre_exec(move || {
            libc::setsid();

            libc::close(slave);
            libc::close(master);

            libc::signal(libc::SIGCHLD, libc::SIG_DFL);
            libc::signal(libc::SIGHUP, libc::SIG_DFL);
            libc::signal(libc::SIGINT, libc::SIG_DFL);
            libc::signal(libc::SIGQUIT, libc::SIG_DFL);
            libc::signal(libc::SIGTERM, libc::SIG_DFL);
            libc::signal(libc::SIGALRM, libc::SIG_DFL);

            Ok(())
        });
    }

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
            std::thread::sleep(Duration::new(0, 10_000_000));
        }
    });

    match builder.spawn() {
        Ok(_) => Ok(master),
        Err(error) => {
            println!("Unable to spawn child process {}", error);
            std::process::exit(1);
        }
    }
}

#[tauri::command]
async fn pty_read(fd: pid_t) -> Result<String, String> {
    let mut buf = [0; 0x1000];
    let ret = read(fd, &mut buf);
    return match ret {
        Ok(r) => Ok(String::from_utf8_lossy(&buf[..r]).to_string()),
        Err(_) => Err("pty read error".into())
    };
}

#[tauri::command]
async fn pty_write(fd: pid_t, data: String) -> Result<(), String> {
    unsafe {
        let r = write(fd, data.as_ptr() as *const c_void, data.len());

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
