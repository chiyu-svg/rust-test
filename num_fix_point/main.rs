///! 定点数

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Q7(i8);

impl From<f64> for Q7 {
     // 很神奇的控制
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8) 
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        (n.0 as f64) * 2_f64.powf(-7.0) // 这很好理解
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64) // 借用 f64 转换
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> Self {
        f64::from(n) as f32 // 这其实也不会损失精度
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    // 用来表示 -1 到 1
    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.0), Q7::from(1.));
        assert_eq!(Q7::from(-10.0), Q7::from(-1.0));
    }
    // f32 转 Q7
    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);

        let n2 = -0.4;
        let q2 = Q7::from(n2);

        let n3 = 123.0;
        let q3 = Q7::from(n3);

        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }
    
    // q7 转 f32
    #[test]
    fn q7_to_f32() {
        let q1 = Q7(32);
        let n1 = f32::from(q1);
        let q2 = Q7::from(n1);
        assert_eq!(q1, q2);
    }

}

fn main() {

}