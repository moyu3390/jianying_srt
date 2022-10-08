use std::fs;
use std::env;
use std::io::stdin;
use std::io::Write;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("使用方法: jianying_srt 输入文件 输出文件");
            println!("按任意键退出.");
            let mut temp = String::new();
            stdin().read_line(&mut temp).expect("Failed to read line.");
        },
        3 => json2srt(&args[1], &args[2]),
        _ => {
            println!("错误, 执行jianying_srt.exe查看使用方法.");
            println!("按任意键退出.");
            let mut temp = String::new();
            stdin().read_line(&mut temp).expect("Failed to read line.");
        },
    }
}

fn json2srt(subtitle: &String, srt_out: &String) {
    let subtitle = subtitle.trim();

    let subtitle = fs::read_to_string(subtitle).expect("Failed to read file.");
    let subtitle = json::parse(&subtitle).unwrap();

    // 获取字幕信息
    let texts = &subtitle["materials"]["texts"];
    let mut contents: Vec<String> = Vec::new();
    for i in 0..texts.len() {
        let content = texts[i]["content"].as_str().expect("Cannot convert string.");
        let re = Regex::new(r"<.*?>").unwrap();

        let content = re.replace_all(content, "");
        let content = content.to_string() + "\n";
        contents.push(content);
    }
    println!("读取字幕信息完成.");
    // 获取轨道信息
    let segments = &subtitle["tracks"][1]["segments"];
    let mut times: Vec<String> = Vec::new();
    for i in 0..segments.len() {
        let start = &segments[i]["target_timerange"]["start"];
        let start = start.to_string();
        let start: i64 = start.parse().expect("Cannot parse");
        let duration = &segments[i]["target_timerange"]["duration"];
        let duration = duration.to_string();
        let duration: i64 = duration.parse().expect("Cannot parse");
        let duration = duration + start;
        let start = us2hmsms(start);
        let duration = us2hmsms(duration);
        times.push(format!("{:02}:{:02}:{:02},{:03} --> {:02}:{:02}:{:02},{:03}\n",
                           start.0, start.1, start.2, start.3,
                           duration.0, duration.1, duration.2, duration.3));
    }
    println!("读取轨道信息完成.");
    let outpath = srt_out.trim();
    let mut outfile = fs::File::create(outpath).unwrap();
    for i in 0..contents.len() {
        outfile.write(((i + 1).to_string() + "\n").as_bytes()).unwrap();
        outfile.write(times[i].as_bytes()).unwrap();
        outfile.write(contents[i].as_bytes()).unwrap();
        outfile.write("\n".as_bytes()).unwrap();
    }
    println!("输出完成.");
}

fn us2hmsms(us: i64) -> (i64, i64, i64, i64) {
    let ms: i64 = us / 1000;
    let s: i64 = ms / 1000;
    let ms: i64 = ms % 1000;
    let m: i64 = s / 60;
    let s: i64 = s % 60;
    let h: i64 = m / 60;
    let m: i64 = m % 60;
    return (h, m, s, ms);
}