use std::arch::aarch64::{int16x4_t, vmla_s16};
use std::cmp::min;
use std::mem::{transmute, zeroed};

fn main() {
    let veca = [1i16, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let vecb = [1i16, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut vecr: [i16; 10] = Default::default();

    unsafe {
        let a: int16x4_t = zeroed();
        for i in (0..veca.len()).step_by(4) {
            let maski = min(veca.len() - i, 4);
            let b: int16x4_t = transmute::<[i16; 4], int16x4_t>(
                veca[i..(i+maski)].try_into().unwrap_or_else(|_| {
                    // Padding slow-path
                    let mut vecas_vec = veca[i..(i+maski)].to_vec();
                    vecas_vec.resize(4, 0);
                    vecas_vec.try_into().unwrap()
                }));
            let c: int16x4_t = transmute::<[i16; 4], int16x4_t>(
                vecb[i..(i+maski)].try_into().unwrap_or_else(|_| {
                    // Padding slow-path
                    let mut vecbs_vec = vecb[i..(i+maski)].to_vec();
                    vecbs_vec.resize(4, 0);
                    vecbs_vec.try_into().unwrap()
                }));
            vecr[i..(i+maski)].copy_from_slice(&transmute::<int16x4_t, [i16; 4]>(vmla_s16(a, b, c))[..maski]);
        }
        for elem in vecr {
            println!("{elem}");
        }
    }
}