use std::fs;
use std::io::stdin;
use std::io::Write;
use regex::Regex;

fn main() {
    println!("输入json文件绝对路径或拖拽到此:");
    let mut subtitle = String::new();
    stdin().read_line(&mut subtitle).expect("Failed to read line.");
    let subtitle = subtitle.trim();

    let subtitle = fs::read_to_string(subtitle).expect("Failed to read file.");
    let subtitle = json::parse(&subtitle).unwrap();

    // 获取字幕信息
    let texts = &subtitle["materials"]["texts"];
    let mut contents:Vec<String> = Vec::new();
    for i in 0..texts.len() {
        let content = texts[i]["content"].as_str().expect("Cannot convert string.");
        let re = Regex::new(r"<.*?>").unwrap();

        let content = re.replace_all(content, "");
        let content = content.to_string() + "\n";
        contents.push(content);
    }
    // 获取轨道信息
    let segments = &subtitle["tracks"][1]["segments"];
    let mut times:Vec<String> = Vec::new();
    for i in 0..segments.len() {
        let start = &segments[i]["target_timerange"]["start"];
        let start = start.to_string();
        let start:i32 = start.parse().expect("Cannot parse");
        let duration = &segments[i]["target_timerange"]["duration"];
        let duration = duration.to_string();
        let duration:i32 = duration.parse().expect("Cannot parse");
        let duration = duration + start;
        let start = us2hmsms(start);
        let duration = us2hmsms(duration);
        times.push(format!("{:02}:{:02}:{:02},{:03} --> {:02}:{:02}:{:02},{:03}\n",
                 start.0, start.1, start.2, start.3,
                 duration.0, duration.1, duration.2, duration.3));
    }
    println!("转换完成. 生成srt文件到:");
    let mut outpath = String::new();
    stdin().read_line(&mut outpath).expect("Failed to read line.");
    let outpath = outpath.trim();
    let mut outfile = fs::File::create(outpath).unwrap();
    for i in 0..contents.len() {
        outfile.write(((i + 1).to_string() + "\n").as_bytes()).unwrap();
        outfile.write(times[i].as_bytes()).unwrap();
        outfile.write(contents[i].as_bytes()).unwrap();
        outfile.write("\n".as_bytes()).unwrap();
    }
    println!("输出完成. 按任意键退出.");
    let mut temp = String::new();
    stdin().read_line(&mut temp).expect("Failed to read line.");
}

fn us2hmsms(us: i32) -> (i32, i32, i32, i32) {
    let ms: i32 = us / 1000;
    let s: i32 = ms / 1000;
    let ms: i32 = ms % 1000;
    let m: i32 = s / 60;
    let s: i32 = s % 60;
    let h: i32 = m / 60;
    let m: i32 = m % 60;
    return (h, m, s, ms);
}