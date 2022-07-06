use std::error::Error;
use std::fs::read_dir;
use std::fs::metadata;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("./格式.html")
        .expect("文件打开失败");

    // println!("请输入目录路径：");
    // let mut path = String::new();
    // io::stdin()
    //     .read_line(&mut path)
    //     .expect("请输入目录路径");

    output(&mut file, &"./".to_string());

}

fn output(file: &mut File, path: &String) {
    let list = all_path(path);
    for path in list {
        let kingdoms = ["乃木坂46", "欅坂46", "けやき坂46", "日向坂46", "吉本坂46", "坂道研修生", "櫻坂46", "STU48", "自闭群", "神", "ザンビ"];
        let kibitsu = ["一期生", "二期生", "三期生", "四期生", "五期生"];
        let mut kingdom = String::new();
        let mut ki_su = String::new();
        let mut name = String::new();
        let mut file_name = String::new();
        let mut index = 1;
        for val in path.iter() {
            if val.ends_with(".png") {
                for str_2 in val.trim_start_matches("./现有武将合集\\").split("\\"){
                    if kingdoms.contains(&str_2) {
                        kingdom = str_2.to_string();
                    } else if kibitsu.contains(&str_2) {
                        ki_su = str_2.to_string();
                    } else if str_2.ends_with(".png") {
                        file_name = str_2.trim_end_matches(".png").to_string();
                    } else {
                        if name == str_2.to_string() {
                            index += 1;
                            continue;
                        } else {
                            index = 1;
                            writeln!(file, "").unwrap();
                            name = str_2.to_string();
                        }
                    }
                }
                if ki_su != "" {
                    writeln!(file, "AO{:0>2};<p><img src=\"https://cdn.jsdelivr.net/gh/LuciferGabriel/sakamichi_img/现有武将合集/{kingdom}/{ki_su}/{name}/{file_name}.png\"/></p><p><span style=\"font-size:28px\">{file_name}</span></p>", index).unwrap();
                } else if name != "" {
                    writeln!(file, "AO{:0>2};<p><img src=\"https://cdn.jsdelivr.net/gh/LuciferGabriel/sakamichi_img/现有武将合集/{kingdom}/{name}/{file_name}.png\"/></p><p><span style=\"font-size:28px\">{file_name}</span></p>", index).unwrap();
                } else {
                    writeln!(file, "AO{:0>2};<p><img src=\"https://cdn.jsdelivr.net/gh/LuciferGabriel/sakamichi_img/现有武将合集/{kingdom}/{file_name}.png\"/></p><p><span style=\"font-size:28px\">{file_name}</span></p>", index).unwrap();
                }
            }
        }
    }
}

fn all_path(root_path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut path_list = vec![String::from(root_path)];
    let mut start_index = 0;
    loop {
        let list_len = path_list.len();
        for index in start_index..path_list.len() {
            let path = &path_list[index];
            if metadata(path)?.is_dir() {
                for child_dir in read_dir(&path)? {
                    path_list.push(String::from(child_dir?.path().as_os_str().to_str().expect("")));
                }
            }
        }
        if list_len == start_index { break; }
        start_index = list_len;
    }
    return Ok(path_list);
}
