use rand::prelude::*;
#[derive(Debug)]
enum FileState {
    Open,
    Close
}

// 模拟触发异常
fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

static mut ERROR: i32 = 0;
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}
impl File {
    fn new(name: &str) -> Self {
        File {
            name: String::from(name),
            data: vec![],
            state: FileState::Close
        }
    }
    fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    fn open(mut self) -> Result<File, String> {
        if one_in(100) {
            let err_msg = String::from("Permission denied");
            return Err(err_msg)
        }
        self.state = FileState::Open;
        Ok(self)
    }
    fn close(mut self) -> Result<File, String> {
        if one_in(100) {
            let err_msg = String::from("Interrupted by signal!");
            return Err(err_msg)
        };
        self.state = FileState::Close;
        Ok(self)
    }
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut data = self.data.clone();
        let read_length = data.len();
        save_to.reserve(read_length);
        save_to.append(&mut data);
        Ok(read_length)
    }
}

fn main() {
    let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let f3 = File::new_with_data("f3.txt", &f3_data);
    let mut buffer: Vec<u8> = vec![];
    let f3 = f3.open().unwrap();
    let read_length = f3.read(&mut buffer).unwrap();
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while reading the file");
        }
    }
    let f3 = f3.close().unwrap();
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while closing the file");
        }
    }
    let txt = String::from_utf8_lossy(&buffer);
    println!("{:?}", f3);
    println!("{} is {} bytes", txt, read_length);
}