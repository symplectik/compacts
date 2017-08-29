pub trait Select1<T> {
    /// Returns the position of 'c+1'th appearance of non-zero bit.
    fn select1(&self, c: T) -> Option<T>;
}

pub trait Select0<T> {
    /// Returns the position of 'c+1'th appearance of non-zero bit.
    fn select0(&self, c: T) -> Option<T>;
}

macro_rules! impl_Select {
    ( $( $pos:ty ),* ) => ($(
        impl Select1<$pos> for u64 {
            #[inline]
            fn select1(&self, c: $pos) -> Option<$pos> {
                if c >= self.count_ones() as $pos {
                    return None;
                }
                let width = 64;
                assert!(c < width as $pos);
                let x = self;
                let w = c as u64;
                let s0 = x - ((x & X55) >> 1);
                let s1 = (s0 & X33) + ((s0 >> 2) & X33);
                let s2 = ((s1 + (s1 >> 4)) & X0F).wrapping_mul(X01);
                let p0 = (le8(s2, (w * X01)) >> 7).wrapping_mul(X01);
                let p1 = (p0 >> 53) & !0x7;
                let p2 = p1 as u32;
                let p3 = (s2 << 8).wrapping_shr(p2);
                let p4 = w - (p3 & 0xFF);
                let p5 = lt8(0x0, ((x.wrapping_shr(p2) & 0xFF) * X01) & X8X);
                let s3 = (p5 >> 0x7).wrapping_mul(X01);
                let p6 = (le8(s3, (p4 * X01)) >> 7).wrapping_mul(X01) >> 56;
                let ix = p1 + p6;
                if ix >= width as u64 { None } else { Some(ix as $pos) }
            }
        }

        impl Select0<$pos> for u64 {
            #[inline]
            fn select0(&self, c: $pos) -> Option<$pos> {
                (!self).select1(c)
            }
        }
    )*)
}
impl_Select!(usize, u64, u32, u16, u8);

const X01: u64 = 0x0101_0101_0101_0101;
const X02: u64 = 0x2020_2020_2020_2020;
const X33: u64 = 0x3333_3333_3333_3333;
const X22: u64 = 0x2222_2222_2222_2222;
const X80: u64 = 0x2010_0804_0201_0080;
const X81: u64 = 0x2010_0804_0201_0081;
const X0F: u64 = 0x0f0f_0f0f_0f0f_0f0f;
const X55: u64 = X22 + X33 + X22 + X33;
const X8X: u64 = X81 + X80 + X80 + X80;

fn le8(x: u64, y: u64) -> u64 {
    let x8 = X02 + X02 + X02 + X02;
    let xs = (y | x8) - (x & !x8);
    (xs ^ x ^ y) & x8
}

fn lt8(x: u64, y: u64) -> u64 {
    let x8 = X02 + X02 + X02 + X02;
    let xs = (x | x8) - (y & !x8);
    (xs ^ x ^ !y) & x8
}