use anyhow::{Result, anyhow};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use libc::{
    mknod,
    mode_t,
    dev_t,
    S_IFCHR,
};
use std::ffi::CString;
use std::io::Error;
use std::process::Command;
use crate::db::models::actor::Actor;

const NEW_DIRS: &'static [&'static str] = &["etc", "run", "usr", "var/log"];
const TMP_DIRS: &'static [&'static str] = &["tmp", "run/lock", "var/tmp"];
const COPY_FILES: &'static [&'static str] = &["etc/group", "etc/passwd", "etc/resolv.conf", "etc/hosts"];
const BIND_DIRS: &'static [&'static str] = &[
    "bin",
    "etc/alternatives",
    "etc/ssl/certs",
    "lib",
    "lib64",
    "sbin",
    "usr/bin",
    "usr/include",
    "usr/lib",
    "usr/lib64",
    "usr/libexec",
    "usr/sbin",
    "usr/share",
];

pub fn create_directory_if_not_exists(dir: &str) {
    let path = Path::new(&dir);
    if ! path.exists() {
        fs::create_dir_all(path).expect(&format!("Failed to create directory {}", &dir));
        create_dirs(&dir);
        change_dir_permission(&dir);
        copy_files(&dir);
        bind_mount(&dir);
        make_devices(&dir);
    }
}

pub fn create_dirs(root: &str) {
    let mut dirs = NEW_DIRS.to_vec();
    dirs.extend(TMP_DIRS.iter());
    for d in dirs {
        fs::create_dir_all(format!("{}/{}", root, d)).expect(&format!("Failed to create directory{}", d))
    }
}

pub fn change_dir_permission(root: &str) {
    for d in TMP_DIRS.iter() {
        let metadata = fs::metadata(format!("{}/{}", root, d)).expect(&format!("Failed fetch metadata {}", d));
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o777);
    }
}

pub fn copy_files(root: &str) {
    for f in COPY_FILES {
        fs::copy(format!("/{}", f), format!("{}/{}", root, f)).expect(&format!("Failed to copy file {}", f));
    }
}

pub fn bind_mount(root: &str) {
    for d in BIND_DIRS {
        let sd = format!("/{}", d);
        let rd = format!("{}/{}", root, d);

        if !Path::new(&sd).exists() {
            continue;
        }

        let sdm = fs::metadata(&sd).expect(&format!("Failed to get metadata {}", &sd));
        if sdm.file_type().is_symlink() {
            let rdm = fs::metadata(&rd).expect(&format!("Failed to get metadata {}", &rd));

            if !rdm.file_type().is_symlink() {
                fs::create_dir_all(
                    Path::new(&rd).parent().expect(&format!("Failed to get parent dir path {}", &rd))
                ).expect(&format!("Failed to create parent dir {}", &rd));
            }

            let sdl = fs::read_link(&sd).expect(&format!("Failed to read symlink {}", &rd));
            std::os::unix::fs::symlink(sdl, &rd).expect(&format!("Failed to create symlink {}", &rd));
        } else {
            fs::create_dir_all(&rd).expect(&format!("Failed to create dir {}", &rd));
            if fs::read_dir(&rd).unwrap().count() == 0 {
                libmount::BindMount::new(Path::new(&sd), Path::new(&rd))
                    .recursive(true)
                    .readonly(true)
                    .mount()
                    .expect(&format!("Failed to mount {} to {}", &sd, &rd));
            }
        }
    }
}

pub fn make_devices(root: &str) {
    fs::create_dir_all(format!("{}/dev", root)).expect("Cannot create /dev dir");
    make_device_if_not_exists(format!("{}/dev/null", root), 0o666, makedev(1, 3));
    make_device_if_not_exists(format!("{}/dev/zero", root), 0o666, makedev(1, 5));

    for r in &["random", "urandom"] {
        make_device_if_not_exists(format!("{}/dev/{}", root, r), 0o444, makedev(1, 9));
    }
}

fn make_device_if_not_exists(path: String, mode: mode_t, dev: dev_t) {
    if !Path::new(&path).exists() {
        let err = makenod(&path, S_IFCHR | mode, dev);
        handle_os_error(err, format!("{}", path));
    }
}

fn makenod(path: &String, mode: mode_t, dev: dev_t) -> i32 {
    unsafe {
        mknod(
            CString::new(path.as_bytes())
                .expect("Error in construct CString")
                .as_bytes_with_nul()
                .as_ptr() as *const libc::c_char,
            mode,
            dev,
        )
    }
}

/// Create special file
/// see https://linuxcommand.net/mknod/
fn makedev(maj: u64, min: u64) -> dev_t {
    ((min & 0xff) | ((maj & 0xfff) << 8) | (((min & !0xff)) << 12) | (((maj & !0xfff)) << 32))
        as dev_t
}

fn handle_os_error<T: std::fmt::Display>(err: i32, action: T) {
    if err != 0 {
        panic!(
            "Error: {{action: {}, code: {}, msg: {} }}",
            action,
            err,
            Error::last_os_error()
        )
    }
}

pub fn exec_chroot(root: &str) {
    std::env::set_current_dir(&root).expect(&format!("Cannot change current dir to {}", &root));
}

pub fn exec(role_id: String, command: Vec<String>, roles: Vec<Actor>) -> Result<String> {

    let mut role_id_in_roles = false;
    for role in roles {
        if role_id == role.actor_id() {
            role_id_in_roles = true;
            break;
        }
    }
    if ! role_id_in_roles {
        return Err(anyhow!("ユーザーは指定されたロールを持っていません"));
    }

    if command[0].as_str() == "cd" {
        println!("changing directory...");
        if command.len() != 2 {
            return Err(anyhow!("bash: cd: 引数が多すぎます"));
        }
        std::env::set_current_dir(&command[1])?;
        return Ok(String::from(""));
    }

    let mut path = std::env::current_exe()?;
    path.pop();
    let role_exe_path = format!("{}/roles/role-{}/role-{}", path.display(), role_id.clone(), role_id.clone());

    let output = Command::new(role_exe_path)
        .args(command)
        .output()?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
