use std::{io::{self, Write}, fs::OpenOptions};

fn main() {
    loop {
        format();
    }
}

fn format() {
    let kingdoms = ["乃木坂46", "欅坂46", "けやき坂46", "日向坂46", "吉本坂46", "坂道研修生", "櫻坂46", "STU48", "自闭群", "神", "ザンビ"];
    let kibitsu = ["一期生", "二期生", "三期生", "四期生", "五期生"];

    println!("请选择需要生成的势力：");
    for (i, kingdom) in kingdoms
        .iter()
        .enumerate() {
        println!("{}：{}", i + 1, kingdom);
    }
    let mut kingdom = String::new();
    io::stdin()
        .read_line(&mut kingdom)
        .expect("选择不存在");
    let kingdom = kingdoms[kingdom.trim()
        .parse::<usize>()
        .unwrap() - 1];

    let mut ki_su = String::new();
    match kingdom {
        "乃木坂46" => {
            loop {
                println!("请选择需要生成的期数：");
                io::stdin()
                    .read_line(&mut ki_su)
                    .expect("请输入期数");
                if ki_su.trim().parse::<usize>().unwrap() > 5 {
                    println!("期数不能超过五期");
                } else {
                    break;
                }
            }
            let ki_su:Option<&str> = Some(kibitsu[ki_su.trim()
                .parse::<usize>()
                .unwrap() - 1]);
            return output(kingdom, ki_su);
        }
        "欅坂46" | "櫻坂46" => {
            loop {
                println!("请选择需要生成的期数：");
                io::stdin()
                    .read_line(&mut ki_su)
                    .expect("请输入期数");
                if ki_su.trim().parse::<usize>().unwrap() > 2 {
                    println!("期数不能超过二期");
                } else {
                    break;
                }
            }
            let ki_su:Option<&str> = Some(kibitsu[ki_su.trim()
                .parse::<usize>()
                .unwrap() - 1]);
            return output(kingdom, ki_su);
        }
        "けやき坂46" | "日向坂46" => {
            loop {
                println!("请选择需要生成的期数：");
                io::stdin()
                    .read_line(&mut ki_su)
                    .expect("请输入期数");
                if ki_su.trim().parse::<usize>().unwrap() > 3 {
                    println!("期数不能超过三期");
                } else {
                    break;
                }
            }
            let ki_su:Option<&str> = Some(kibitsu[ki_su.trim()
                .parse::<usize>()
                .unwrap() - 1]);
            return output(kingdom, ki_su);
        }
        _ => {
            let ki_su:Option<&str> = None;
            return output(kingdom, ki_su);
        }
    }
}

fn output(kingdom: &str, ki_su:Option<&str>) {
    let mut abbreviation = String::new();
    match kingdom {
        "乃木坂46" => {
            abbreviation = "NGI".to_string();
        }
        "欅坂46" => {
            abbreviation = "KYK".to_string();
        }
        "けやき坂46" => {
            abbreviation = "HRG".to_string();
        }
        "吉本坂46" => {
            abbreviation = "YMT".to_string();
        }
        "日向坂46" => {
            abbreviation = "HNT".to_string();
        }
        "坂道研修生" => {
            abbreviation = "KSS".to_string();
        }
        "櫻坂46" => {
            abbreviation = "SKR".to_string();
        }
        "STU48" => {
            abbreviation = "STU".to_string();
        }
        "自闭群" => {
            abbreviation = "ZBQ".to_string();
        }
        "神" => {
            abbreviation = "GOD".to_string();
        }
        "ザンビ" => {
            abbreviation = "ZMB".to_string();
        }
        _ => {
            abbreviation = "".to_string();
        }
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open("./格式.html")
        .expect("文件打开失败");

    println!("请输入需要生成的武将名：");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("请输入姓名");
    let name = name.trim();

    println!("请输入需要生成的武将编号：");
    let mut numbering = String::new();
    io::stdin()
        .read_line(&mut numbering)
        .expect("请输入编号");
    let numbering = numbering.trim()
        .parse::<usize>()
        .unwrap();

    println!("请输入需要生成的个数：");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("请输入个数");
    let number = number.trim()
        .parse::<usize>()
        .unwrap();

    for num in 1..=number {
        match ki_su {
            Some(i) => {
                writeln!(&mut file, "AO{:0>2};<p><img src=\"https://cdn.jsdelivr.net/gh/LuciferGabriel/sakamichi_img/现有武将合集/{kingdom}/{i}/{name}/{abbreviation} {numbering} {name} {:0>3}.png\"/></p><p><span style=\"font-size:28px\">{abbreviation} {numbering} {name} {:0>3}</span></p>", num, num, num).unwrap();
            }
            None => {
                writeln!(&mut file, "AO{:0>2};<p><img src=\"https://cdn.jsdelivr.net/gh/LuciferGabriel/sakamichi_img/现有武将合集/{kingdom}/{name}/{abbreviation} {numbering} {name} {:0>3}.png\"/></p><p><span style=\"font-size:28px\">{abbreviation} {numbering} {name} {:0>3}</span></p>", num, num, num).unwrap();
            }
        }
    }
    writeln!(&mut file, "").unwrap();
}
