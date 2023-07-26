use nix::libc::{self, F_GETFL, F_SETFL, fcntl, O_NONBLOCK, TIOCSCTTY};
use nix::pty::openpty;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use nix::sys::termios::{self, InputFlags, SetArg};
use std::{env, ptr};
use std::ffi::CStr;
use std::io::{Error, ErrorKind, Result};
use std::mem::MaybeUninit;
use std::os::fd::{FromRawFd, RawFd};
use std::os::unix::prelude::CommandExt;
use std::process::{Command, Stdio};

struct ShellUser {
    user: String,
    home: String,
    shell: String,
}

impl ShellUser {
    fn from_env() -> Result<ShellUser> {
        let mut buf = [0; 1024];
        // Create zeroed passwd struct.
        let mut entry: MaybeUninit<libc::passwd> = MaybeUninit::uninit();
        let mut res: *mut libc::passwd = ptr::null_mut();

        // Try and read the pw file.
        let uid = unsafe { libc::getuid() };
        let status = unsafe { libc::getpwuid_r(
            uid,
            entry.as_mut_ptr(),
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            &mut res
        )};
        let entry = unsafe { entry.assume_init() };

        if status < 0 {
            return Err(Error::new(ErrorKind::Other, "getpwuid_r failed"));
        }

        if res.is_null() {
            return Err(Error::new(ErrorKind::Other, "passwd is null"));
        }

        // Sanity check.
        assert_eq!(entry.pw_uid, uid);

        let (
            pw_name,
            pw_dir,
            pw_shell
        ) = (
            unsafe { CStr::from_ptr(entry.pw_name).to_str().unwrap() },
            unsafe { CStr::from_ptr(entry.pw_dir).to_str().unwrap() },
            unsafe { CStr::from_ptr(entry.pw_shell).to_str().unwrap() }
        );


        let user = match env::var("HOME") {
            Ok(user) => user,
            Err(_) => pw_name.to_owned()
        };

        let home = match env::var("HOME") {
            Ok(home) => home,
            Err(_) => pw_dir.to_owned()
        };

        let shell = match env::var("SHELL") {
            Ok(shell) => shell,
            Err(_) => pw_shell.to_owned()
        };

        Ok(Self {
            user,
            home,
            shell
        })
    }
}

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
    builder.stdin(unsafe  { Stdio::from_raw_fd(slave) });
    builder.stderr(unsafe { Stdio::from_raw_fd(slave) });
    builder.stdout(unsafe { Stdio::from_raw_fd(slave) });

    builder.env("USER", user.user);
    builder.env("HOME", user.home);

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
        Ok(_child) => {
            unsafe {
                // set non blocking
                let res = fcntl(master, F_SETFL, fcntl(master, F_GETFL, 0) | O_NONBLOCK);
                assert_eq!(res, 0);
            }
            Ok(master)
        }
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