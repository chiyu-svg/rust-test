use std::error::Error;
use reqwest;

fn main() -> Result<(), Box<dyn Error>>{
    let url = "https://mbd.baidu.com/newspage/data/landingsuper?context=%7B%22nid%22%3A%22news_9077862423058967403%22%7D&n_type=-1&p_from=-1";
    let mut response = reqwest::get(url)?;
    let content = response.text()?;
    print!("{}", content);
    Ok(())
}