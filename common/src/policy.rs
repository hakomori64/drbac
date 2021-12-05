use anyhow::{Result, anyhow};
use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;



fn render_te(data: &HashMap<&str, &str>) -> Result<String> {
    let handlebars = Handlebars::new();
    handlebars.render_template(include_str!("templates/te.hbs"), data)
        .map_err(|_| anyhow!("te render failed"))
}

fn render_fc(data: &HashMap<&str, &str>) -> Result<String> {
    let handlebars = Handlebars::new();
    handlebars.render_template(include_str!("templates/fc.hbs"), data)
        .map_err(|_| anyhow!("fc render failed"))
}

fn create_role_dir(role_name: &str) -> Result<()> {
    create_dir_all(format!("roles/{}", role_name)).map_err(|_| anyhow!("create dir all err"))
}

fn arrange_te(role_name: &str) -> Result<()> {
    let mut mp = HashMap::new();
    mp.insert("role_name", role_name.clone());

    let result = render_te(&mp)?;
    
    let mut file = File::create(format!("roles/{}/{}.te", role_name.clone(), role_name.clone()))?;
    file.write_all(result.as_bytes())?;
    Ok(())
}

fn arrange_fc(role_name: &str) -> Result<()> {
    let path = std::env::current_dir()?;
    let mut mp = HashMap::new();
    mp.insert("role_name", role_name);
    let path = &format!("{}", path.to_string_lossy());
    mp.insert("exe_dir", path);

    let result = render_fc(&mp)?;

    let mut file = File::create(format!("roles/{}/{}.fc", role_name.clone(), role_name.clone()))?;
    file.write_all(result.as_bytes())?;
    Ok(())
}

fn deploy_role_binary(role_name: &str) -> Result<()> {
    std::fs::copy("roles/base_binary", format!("roles/{}/{}", role_name.clone(), role_name.clone()))?;
    Ok(())
}

pub fn role_presetup(role_name: &str) -> Result<()> {
    create_role_dir(role_name)?;
    arrange_fc(role_name)?;
    arrange_te(role_name)?;
    deploy_role_binary(role_name)?;

    Ok(())
}

pub fn show_role_presetup_message(role_name: &str) -> Result<()> {
    let path = std::env::current_dir()?;
    // print role creation success message here
    println!("roles/{}に移動してロールのポリシー({}.te, {}.fc)を編集したあと、以下のコマンドを実行してください", role_name.clone(), role_name.clone(), role_name.clone());
    println!("cd {}/roles/{}", path.display(), role_name.clone());
    println!("cp /usr/share/selinux/devel/Makefile .");
    println!("make");
    println!("semodule -i {}.pp", role_name.clone());
    println!("ロールがセットアップされたディレクトリがホームディレクトリ以下であれば、");
    println!("/etc/selinux/<type>/contexts/files/file_contextsの該当する内容を");
    println!("/etc/selinux/<type>/contexts/files/file_contexts.homedirの末尾に追加してください");
    println!("cd ../");
    println!("restorecon -RFv .");

    Ok(())
}