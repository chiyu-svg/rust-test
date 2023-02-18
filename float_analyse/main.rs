///! 32 浮点数，以科学记数法的形式进行表示
/// 主要包括符号位（1），指数位（8）， 尾数为（23）， 并且按这个顺序进行存储
const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;
    let (sign, exponent, fraction) = to_parts(n);
    let (sign_, exponent_, mantissa) = decode(sign, exponent, fraction);
    let n_ = from_parts(sign_, exponent_, mantissa);
    println!("{}", n_);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits: u32 = n.to_bits();
    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;  // 分离出指数位，并且擦抹掉符号位
    let fraction = bits & 0x7fffff;
    (sign, exponent, fraction)
}
fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let mut mantissa: f32 = 0.0; // 初始权重
    let sign = (-1.0_f32).powf(sign as f32);
    let exponent = (exponent as i32) - BIAS; // 必须转换成 i32, 因为有可能是负数
    let exponent = RADIX.powf(exponent as f32);
    for i in 0..23 {
        let mask = i << i; // 用来获取遍历的位
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    (sign, exponent, mantissa)
}
fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}