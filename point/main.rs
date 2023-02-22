use std::mem::size_of;

static  B: [u8; 3] = [101, 22, 34];
static  C: [u8; 5] = [33, 44, 54, 66, 20];

fn main(){
    let a: usize = 42;
    let b = &B;
    let c = Box::new(C);
    
    println!("location: {:p}", &a);
    println!("szie: {:?} bytes", size_of::<usize>());

    println!("location: {:p}", &b);
    println!("size: {:?} bytes", size_of::<&[u8; 3]>());  // 引用占用 8 个字节 其实是与带宽有关

    println!("location: {:p}", &c);
    println!("size: {:?} bytes", size_of::<Box<[u8]>>()); // 智能指针

    println!("size: {:?} bytes", size_of::<[u8; 10]>()); // 10 个数组的大小
}