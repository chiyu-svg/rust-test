use regex::Regex;
use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let args = App::new("grep-lite").version("0.1").arg(Arg::with_name("pattern").takes_value(true).required(true))
    .arg(Arg::with_name("input").required(true))
    .arg(Arg::with_name("bound").required(true)).get_matches();
    // 搜索内容参数
    let pattern = args.value_of("pattern").unwrap();
    // 内容文本目录
    let input = args.value_of("input").unwrap();
    // 内容边界
    let search_bound = args.value_of("bound").unwrap().parse::<usize>().unwrap();
    // 保存匹配内容的行号
    let mut tags: Vec<usize> = vec![];
    // 保存匹配内容的附近行
    let mut ctxs: Vec<Vec<(usize, String)>> = vec![];
    let re = Regex::new(pattern).unwrap();
    let f = File::open(input).unwrap();
    let f1 = File::open(input).unwrap();
    let reader = BufReader::new(f);
    let reader1 = BufReader::new(f1);
    process_lines(reader, re, &mut tags, &mut ctxs, search_bound);
    if tags.is_empty() {
        return;    // 如果没有匹配的行就返回
    }
    // 匹配每行附近行
    for (i, line) in reader1.lines().enumerate() {
        let line = line.unwrap();
        for (j, tags) in tags.iter().enumerate() {  // enumerate 只在迭代器上才能使用
            let lower_bound = tags.saturating_sub(search_bound); // 防止越界
            let upper_bound = tags + search_bound;
            if i >= lower_bound && i <= upper_bound {
                println!("{}:{}", i, line);
            }
        }
    }
}
fn process_lines<T: BufRead + Sized>(reader: T, re: Regex, tags: &mut Vec<usize>, ctxs: &mut Vec<Vec<(usize, String)>>, search_bound: usize) {
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let contains_substring = re.find(&line);
        match contains_substring {
            Some(_) => {
                tags.push(i);
                let local_ctx = Vec::with_capacity(2*search_bound + 1); //节省扩容时的性能
                ctxs.push(local_ctx);
            },
            None => {}
        }
    }
}

/// 
/// 1. 迭代器.enumerate()
/// 2. 字符串.content()
/// 3. usize.saturating_sub() 做减法防止越界 0 一下
fn lenarn_note(){}