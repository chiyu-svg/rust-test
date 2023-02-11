//! make a grep-lite
//! ***aim
//! 1. 搜索文本中包含指定内容的行
//! 2. 同时显示搜索行附近的指定行数

// 需要处理的文本和要搜索的内容，暂且先存放在这里
mod message {
    pub struct Content {
        pub message: &'static str,   // 文本内容
        pub search_content: &'static str,  // 搜索内容
        pub line_extent: usize,  // 显示上下附近行数
    }
    impl Content {
        pub fn new() -> Self {
            Content { 
                message: "/
Take a morning walk of gratitude. It will create a fertile mind ready for success.
Instead of being disappointed about where you are. Think optimistically about where you are going.
Eat. Eat breakfast like a king, lunch like a prince, and dinner like a college kid with a maxed-out charge card.
Remember that adversity is not a dead-end but a detour to a better outcome.
Focus on learning, loving, growing, and serving.
Believe that everything happens for a reason. Expect good things to come out of challenging experiences.
Life isn't fair, but it's still good.
                ", 
                search_content: "like",
                line_extent: 2
            }
        }
    }
}

use crate::message::Content;
fn main() {
    let Content { message, search_content, line_extent} = Content::new();
    // 保存已匹配行的行号
    let mut tags: Vec<usize> = vec![];
    // 保存每个匹配行的上下文
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    for (i, line) in message.lines().enumerate() {
        if line.contains(search_content) {
            tags.push(i); // 存储匹配的行号
            let v = Vec::with_capacity(2*line_extent + 1); // 申请要显示的内容数组
            ctx.push(v);
        }
    }
    if tags.is_empty() {
        return;  // 没有任何行就退出
    }
    // 对文本的每一行进行匹配
    for (i, line) in message.lines().enumerate() {
        for (j, tags) in tags.iter().enumerate() { // 匹配到的行
            // 文档边界
            let lower_bound = tags.saturating_sub(line_extent); 
            let upper_bound = tags + line_extent;
            if i >= lower_bound && i <= upper_bound {
                let local_ctx = (i, line.to_string()); // 匹配内容的附近行
                ctx[j].push(local_ctx);
            }
        }
    }
    for local_ctx in ctx {
        for (i, line) in local_ctx {
            let line_num = i + 1;
            println!("{}:{}", line_num, line);
        }
    }
}





