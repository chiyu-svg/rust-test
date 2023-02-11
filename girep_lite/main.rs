mod message {
    pub struct Content {
        pub content: &'static str,
        pub search_content: &'static str,
        pub search_bound: usize
    }
    impl Content {
        pub fn new() -> Self {
            Content { 
                content: "/   
Take a morning walk of gratitude. It will create a fertile mind ready for success.
Instead of being disappointed about where you are. Think optimistically about where you are going.
Eat. Eat breakfast like a king, lunch like a prince, and dinner like a college kid with a maxed-out charge card.
Remember that adversity is not a dead-end but a detour to a better outcome.
Focus on learning, loving, growing, and serving.
Believe that everything happens for a reason. Expect good things to come out of challenging experiences.
Life isn't fair, but it's still good." , 
                search_content: "but", 
                search_bound: 2 
            }
        }
    }
}

use message::Content;
use regex::Regex;
fn main() {
    let Content { content, search_content, search_bound } = Content::new();
    // 保存匹配内容的行号
    let mut tags: Vec<usize> = vec![];
    // 保存匹配内容的附近行
    let mut ctxs: Vec<Vec<(usize, String)>> = vec![];
    let re = Regex::new(search_content).unwrap();
    for (i, line) in content.lines().enumerate() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => {
                tags.push(i);
                let local_ctx = Vec::with_capacity(2*search_bound + 1); //节省扩容时的性能
                ctxs.push(local_ctx);
            },
            None => {}
        }
        if line.contains(search_content) {
           
        }
    }
    if tags.is_empty() {
        return;    // 如果没有匹配的行就返回
    }
    // 匹配每行附近行
    for (i, line) in content.lines().enumerate() {
        for (j, tags) in tags.iter().enumerate() {  // enumerate 只在迭代器上才能使用
            let lower_bound = tags.saturating_sub(search_bound); // 防止越界
            let upper_bound = tags + search_bound;
            if i >= lower_bound && i <= upper_bound {
                let local_ctx = (i, line.to_string());
                ctxs[j].push(local_ctx);
            }
        }
    }
    // 输出每行
    for local_ctxs in ctxs {
        for (lin_num, content) in local_ctxs {
            println!("{}:{}", lin_num, content);
        } 
    }
}
/// 
/// 1. 迭代器.enumerate()
/// 2. 字符串.content()
/// 3. usize.saturating_sub() 做减法防止越界 0 一下
/// 3  Regex 没有集成在语言中
fn lenarn_note(){}