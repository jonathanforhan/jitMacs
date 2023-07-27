use nix::libc;
use std::mem::MaybeUninit;
use std::ffi::{CStr};
use std::io::{Error, ErrorKind, Result};
use std::{env, ptr};

/**
 * Shell User composed of environment variables
 */
pub(crate) struct ShellUser {
    pub user: String,
    pub home: String,
    pub shell: String,
}

impl ShellUser {
    /**
     * Constructors a shell user from environment
     */
    pub(crate) fn from_env() -> Result<ShellUser> {
        let mut buf: [u8; 1024] = [0; 1024];
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
            return Err(Error::new(ErrorKind::Other, "passwd error"));
        }
        // Sanity check.
        assert_eq!(entry.pw_uid, uid);

        let user = match env::var("USER") {
            Ok(user) => user,
            Err(_) => unsafe {
                CStr::from_ptr(entry.pw_name).to_str().unwrap().to_owned()
            }
        };

        let home = match env::var("HOME") {
            Ok(home) => home,
            Err(_) => unsafe {
                CStr::from_ptr(entry.pw_dir).to_str().unwrap().to_owned()
            }
        };

        let shell = match env::var("SHELL") {
            Ok(shell) => shell,
            Err(_) => unsafe {
                CStr::from_ptr(entry.pw_shell).to_str().unwrap().to_owned()
            }
        };

        Ok(Self {
            user,
            home,
            shell
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::tty::unix::shell::ShellUser;

    #[test]
    fn shell_from_env() {
        let _shell_user = ShellUser::from_env().unwrap();
    }
}
