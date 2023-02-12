//! 模拟文件系统
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>
}
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new()
        }
    }
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    fn read(&self, save_to: &mut Vec<u8>) -> usize {
        let mut temp = self.data.clone();
        let read_length = temp.len();
        save_to.reserve(read_length);
        save_to.append(&mut temp);
        read_length
    }
    fn open(&self) -> bool {
        true
    }
    fn close(&self) -> bool {
        true
    }
}


fn main() {
    let fs_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f3 = File::new_with_data("2.txt", &fs_data);
    let mut buffer: Vec<u8> =  vec![];
    f3.open();
    let read_length = f3.read(&mut buffer);
    f3.close();
    println!("{} is {} bytes long", String::from_utf8_lossy(&buffer), read_length);
}