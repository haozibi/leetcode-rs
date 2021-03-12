use std::fs::OpenOptions;
use std::{fs, io, path::PathBuf};

use anyhow;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::Write;

fn main() {
    println!("Hello, world!");

    let dir = String::from("/home/bi/.leetcode/rs");

    search(dir).unwrap();
}

fn search(dir: String) -> anyhow::Result<()> {
    let mut entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();

    println!("{:?}", entries);
    let got = select(entries)?;

    create(got)
}

fn create(name: PathBuf) -> anyhow::Result<()> {
    // 1. 添加额外的信息
    // 2. 创建文件夹
    // 3. 写入文件

    let filename: String = match name.file_name() {
        None => "none".to_string(),
        Some(v) => {
            if let Some(e) = v.to_str() {
                String::from(e.replace(".", "_").replace("_rs", ""))
            } else {
                "none".to_string()
            }
        }
    };

    let body = fs::read_to_string(&name)?;
    let filename_en = find_en_name(body.clone());
    let filename_id = find_id(filename.clone());

    let create_dir = format!("leetcode/src/_{}_{}", filename_id, filename_en);
    let code_file = format!("leetcode/src/_{}_{}/mod.rs", filename_id, filename_en);
    let readme_file = format!("leetcode/src/_{}_{}/README.md", filename_id, filename_en);
    let readme_body = format!("# {}", filename.replace("_", "."));

    let mut body = body.replace("/*", "#[allow(unused)]\n\n/*");

    body.push_str(
        "\n#[allow(unused)]
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}\n",
    );

    fs::create_dir_all(create_dir)?;

    fs::write(code_file, body)?;
    fs::write(readme_file, readme_body)?;

    insert_lib(format!("_{}_{}", filename_id, filename_en).to_string());
    Ok(())
}

fn insert_lib(name: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("leetcode/src/lib.rs")
        .expect("cannot open file");

    file.write_all(format!("\nmod {};", &*name).as_bytes())
        .expect("write failed");
}

fn find_id(name: String) -> String {
    let list: Vec<_> = name.split("_").collect();
    list[0].to_string()
}

// 临时方案，查找英文名称
fn find_en_name(body: String) -> String {
    let list = body.split("\n");
    let new_list: Vec<_> = list
        .into_iter()
        .filter(|x| {
            return match x.find("https://leetcode-cn.com/problems/") {
                None => false,
                Some(_) => true,
            };
        })
        .collect();

    if new_list.len() == 0 {
        return String::from("none");
    }

    let name = new_list[0]
        .replace("https://leetcode-cn.com/problems/", "")
        .replace("*", "")
        .replace(" ", "")
        .replace("/description/", "")
        .replace("-", "_");

    println!("en_name: {}", name);

    String::from(name)
}

fn select(list: Vec<PathBuf>) -> anyhow::Result<PathBuf> {
    if list.len() == 0 {
        return anyhow::bail!("len is zero");
    }
    let new_list: Vec<String> = list
        .iter()
        .map(|x| x.display().to_string())
        .collect::<Vec<_>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&new_list[..])
        .interact()
        .unwrap();

    Ok(list[selection].clone())
}
