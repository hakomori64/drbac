use anyhow::{Result, anyhow};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use libc::{
    mknod,
    mode_t,
    dev_t,
    chroot,
    S_IFCHR,
};
use std::ffi::CString;
use std::io::Error;
use std::process::Command;
use users::{
    get_user_by_name,
};
use std::os::unix::process::CommandExt;
use crate::db::models::actor::Actor;
use std::fs::File;
use std::io::Read;
use crate::encoding::vec_to_struct;

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
    handle_os_error(err, "chroot");
}

pub fn exec(command: String, args: Vec<String>, guest_id: u32) -> Result<String> {

    if command.as_str() == "cd" {
        if args.len() != 1 {
            return Err(anyhow!("bash: cd: 引数が多すぎます"));
        }
        std::env::set_current_dir(&args[0])?;
        return Ok(String::from(""));
    }

    if args.len() != 0 {
        let output = Command::new(command)
            .uid(guest_id)
            .args(&args)
            .output()?;
        
        Ok(String::from_utf8(output.stdout)?)
    } else {
        let output = Command::new(command)
            .uid(guest_id)
            .output()?;
        
        Ok(String::from_utf8(output.stdout)?)
    }
}

pub fn get_guest_id() -> Result<u32> {
    let id = match get_user_by_name("guest") {
        Some(user) => user.uid(),
        None => return Err(anyhow!("ゲストアカウントが存在しません"))
    };

    Ok(id)
}



#[derive(Debug, Serialize, Deserialize, Clone)]
struct RoleMap {
    pub src: String,
    pub dst: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RoleMaps {
    pub maps: Vec<RoleMap>
}

pub fn assign_roles_to_guest(roles: Vec<Actor>, entity_id: String) -> Result<()> {

    // map drbac roles to local roles
    let filename = format!("role_maps/{}.json", entity_id);
    let mut map_file = File::open(&filename).map_err(|_| anyhow!("role_mapを開くのに失敗しました"))?;
    let metadata = fs::metadata(&filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    map_file.read(&mut buffer)?;

    let data: RoleMaps = vec_to_struct(buffer)?;

    let mut role_maps = HashMap::new();

    for role_map in data.maps {
        role_maps.insert(role_map.src, role_map.dst);
    }

    let mut role_names: Vec<String> = vec![];
    for role in roles {
        let role_name = if let Actor::Role { name, .. } = role {
            name
        } else {
            return Err(anyhow!("アクタータイプがロールではありません"));
        };

        match role_maps.get(&role_name) {
            Some(value) => {
                role_names.push(value.clone());
            }
            _ => {
                return Err(anyhow!("DRBACのロールに対応するSELinuxのロールがありません。"));
            }
        }
    }

    let role_str = role_names.join(" ");

    let command = "semanage";
    let formatted = &format!("{}", role_str);
    let args = vec!{
        "user",
        "-m",
        "-R",
        formatted,
        "guest_u"
    };

    let status = Command::new(command)
        .args(&args)
        .status().map_err(|_| anyhow!("semanageの実行に失敗しました"))?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("guestのロールの変更に失敗しました"))
    }
}