mod shell;
use shell::ShellUser;
use crate::payload::PtyPayload;
pub mod window;
use window::WindowSize;

use nix::libc::{self, EBADFD, EINTR, F_GETFD, F_GETFL, F_SETFL, O_NONBLOCK, POLLERR, POLLHUP, POLLIN, POLLNVAL, TIOCSCTTY, winsize};
use nix::poll::{PollFd, PollFlags};
use nix::pty::openpty;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use nix::sys::termios::{self, InputFlags, SetArg};
use nix::unistd;
use std::io::{Error, ErrorKind, Result};
use std::thread;
use std::os::fd::{FromRawFd, RawFd};
use std::os::unix::prelude::CommandExt;
use std::process::{Command, Stdio};
use nix::errno::errno;
use tauri::Manager;

/**
 * Spawns a pty, uses openpty, builds a process group
 * forks a bash process and returns our master fd,
 * IO is non-blocking
 */
pub fn spawn() -> Result<RawFd> {
    let ends = openpty(None, None)?;
    let (master, slave) = (ends.master, ends.slave);

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    if let Ok(mut termios) = termios::tcgetattr(master) {
        // Set character encoding to UTF-8.
        termios.input_flags.set(InputFlags::IUTF8, true);
        let _ = termios::tcsetattr(master, SetArg::TCSANOW, &termios);
    }

    let user = ShellUser::from_env()?;

    let mut builder = Command::new(user.shell);

    // Setup child stdin/stdout/stderr as slave fd of PTY.
    // Ownership of fd is transferred to the Stdio structs and will be closed by them at the end of
    // this scope. (It is not an issue that the fd is closed three times since File::drop ignores
    // error on libc::close.).
    builder
        .stdin (unsafe { Stdio::from_raw_fd(slave) })
        .stderr(unsafe { Stdio::from_raw_fd(slave) })
        .stdout(unsafe { Stdio::from_raw_fd(slave) })
        .env("USER", user.user)
        .env("HOME", user.home);

    unsafe {
        builder.pre_exec(move || {
            // create new process group
            if libc::setsid() < 0 {
                return Err(Error::new(ErrorKind::Other, "Failed to set session id"));
            }

            // TIOCSCTTY changes based on platform and the `ioctl` call is different
            // based on architecture (32/64). So a generic cast is used to make sure
            // there are no issues. To allow such a generic cast the clippy warning
            // is disabled.
            #[allow(clippy::cast_lossless)]
            if libc::ioctl(slave, TIOCSCTTY as _, 0) < 0 {
                return Err(Error::new(ErrorKind::Other, "ioctl TIOCSCTTY failed"));
            }

            // No longer need slave/master fds.
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

    match builder.spawn() {
        Ok(_child) => unsafe {
            // set non blocking
            let res = libc::fcntl(master, F_SETFL, libc::fcntl(master, F_GETFL, 0) | O_NONBLOCK);
            assert_eq!(res, 0);

            Ok(master)
        },
        Err(err) => Err(Error::new(
            err.kind(),
            format!(
                "Failed to spawn command '{}': {}",
                builder.get_program().to_string_lossy(),
                err
            )
        ))
    }
}

/**
 * Polls a file descriptor, we call read in this thread to ensure blocking
 */
pub fn poll(fd: RawFd, app_handle: tauri::AppHandle) -> Result<()>{
    const ERR_BITS: i16 = POLLERR | POLLHUP | POLLNVAL;
    validate_fd(fd)?;

    // poll the newly created fd
    thread::spawn(move || {
        let flags = PollFlags::from_bits(POLLIN).unwrap();
        let mut fds = [PollFd::new(fd, flags)];

        while let Ok(n) = nix::poll::ppoll(&mut fds, None, None) {
            if n <= 0 {
                if errno() == EINTR { continue } else { break }
            }

            match fds[0].revents() {
                Some(events) => {
                    if events.bits() & ERR_BITS != 0 { break }
                    // skip if no buffer data
                    if events.bits() & POLLIN == 0 { continue }
                },
                None => continue
            };

            // return read buffer if data available
            let _ = match read(fd) {
                Ok(res) => app_handle.emit_all("pty-event", PtyPayload {
                    res,
                    fd,
                    status: 200
                }),
                Err(e) => app_handle.emit_all("pty-event", PtyPayload {
                    res: e.to_string(),
                    fd,
                    status: 500
                }),
            };
        }
        let _ = app_handle.emit_all("pty-die", PtyPayload {
            res: "".into(),
            fd,
            status: 200
        });
        unistd::close(fd)
    });

    Ok(())
}

pub fn read(fd: RawFd) -> Result<String> {
    let mut buf: [u8; 0x1000] = [0; 0x1000];

    match unistd::read(fd, &mut buf) {
        Ok(r) => Ok(String::from_utf8_lossy(&buf[..r]).into()),
        Err(e) => Err(Error::new(ErrorKind::Other, format!("Read failure {e}")))
    }
}

pub fn write(fd: RawFd, data: String) -> Result<()> {
    match unistd::write(fd, data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(ErrorKind::Other, format!("Write failure {e}")))
    }
}

pub fn resize(fd: RawFd, window_size: WindowSize) -> Result<()> {
    let window_size: winsize = window_size.to_winsize();

    if unsafe { libc::ioctl(fd, libc::TIOCSWINSZ, &window_size as *const _) } < 0 {
        return Err(Error::new(ErrorKind::Other, "Window resize failure"));
    }
    Ok(())
}

fn validate_fd(fd: RawFd) -> Result<()> {
    unsafe {
        if libc::fcntl(fd, F_GETFD) != -1 || errno() != EBADFD {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, format!("Invalid file descriptor: {fd}")))
        }
    }
}