//! make a grep litte-lite
//! ***aim
//! 1. 搜索文本中包含指定内容的行
//! 2. 同时显示搜索行附近的指定行数

// 需要处理的文本和要搜索的内容，暂且先存放在这里
mod message {
    pub struct Content {
        pub message: &'static str,
        pub search_content: &'static str
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
                search_content: "like"
            }
        }
    }
}
use crate::message::Content;
fn main() {
    let Content { message, search_content } = Content::new();
    let mut line_num = 0_u32;
    for line in message.lines() {
        line_num += 1;
        if line.contains(search_content) {
            println!("{}.{}", line_num, line);
        }
    }
    
}
