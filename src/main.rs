#[derive(Debug)]
struct Satellite {
    set_id: u64
}
impl Satellite {
    fn receive(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.receive(self)
    }
}
#[derive(Debug)]
struct GroundStation;
impl GroundStation {
    fn post(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}
#[derive(Debug)]
struct Message {
    to: u64,
    message: String
}
#[derive(Debug)]
struct Mailbox {
    content: Vec<Message>
}
impl Mailbox {
    fn new() -> Self {
        Mailbox { content: vec![] }
    }
    fn receive(&mut self, to: &Satellite) -> Option<Message> {
        for i in 0..self.content.len() {
            if self.content[i].to == to.set_id {
                let msg = self.content.remove(i);
                return Some(msg)
            }
        }
        None
    }
    fn post(&mut self, msg: Message) {
        self.content.push(msg);
    }
}
fn main() {
    let groundStation = GroundStation;
    let mut mailbox = Mailbox::new();
    for satellite in fatch_satellit() {
        groundStation.post(&mut mailbox, Message { to: satellite, message:format!("hello _ {}", satellite)});
    } 
    for satellite in fatch_satellit() {
        let satl = Satellite { set_id: satellite };
        match satl.receive(&mut mailbox) {
            Some(val) => {
                println!("{:?}", val)
            },
            None => {}
        }
    }
}
/// 模拟存储卫星库
fn fatch_satellit() -> Vec<u64> {
    vec![1,2,3,4]
}