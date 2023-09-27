use std::arch::aarch64::{int16x4_t, vmla_s16};
use std::cmp::min;
use std::fmt::Debug;
use std::mem::{transmute, zeroed};

// Requires allocation of a new length-4 vector on the stack consisting of
// v[i..(i + maski)] + [0] * (4 - maski)
fn padding_slow_path<T: Copy + Debug + Default>(in_vecs: &[T]) -> [T; 4] {
    let mut vecs_vec = in_vecs.to_vec();
    vecs_vec.resize(4, Default::default());
    vecs_vec.try_into().unwrap()
}

unsafe fn masked_slice_transmute(in_vecs: &[i16]) -> int16x4_t {
    transmute::<[i16; 4], int16x4_t>(
        <[i16; 4]>::try_from(in_vecs).unwrap_or_else(|_| {
            padding_slow_path(&in_vecs)
        })
    )
}

fn main() {
    let veca = [1i16, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let vecb = [1i16, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut vecr: [i16; 10] = Default::default();

    unsafe {
        let a: int16x4_t = zeroed();
        for i in (0..veca.len()).step_by(4) {
            let maski = min(veca.len() - i, 4);
            let b: int16x4_t = masked_slice_transmute(&veca[i..(i + maski)]);
            let c: int16x4_t = masked_slice_transmute(&vecb[i..(i + maski)]);
            vecr[i..(i+maski)].copy_from_slice(&transmute::<int16x4_t, [i16; 4]>(vmla_s16(a, b, c))[..maski]);
        }
        for elem in vecr {
            println!("{elem}");
        }
    }
}