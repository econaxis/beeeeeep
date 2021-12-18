
use std::ops::{Mul, Range};


use realfft::RealFftPlanner;

use realfft::num_complex::Complex32;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref PLANNER: Mutex<RealFftPlanner<f32>> = Mutex::new(RealFftPlanner::new());
}

fn fft(mut a: Vec<f32>) -> Vec<Complex32> {
    let mut planner = PLANNER.lock().unwrap();
    let fft = planner.plan_fft_forward(a.len());
    let mut output = fft.make_output_vec();
    fft.process(&mut a, &mut output).unwrap();
    output
}

fn ifft(mut a: Vec<Complex32>) -> Vec<f32> {
    let mut planner = PLANNER.lock().unwrap();
    let fft = planner.plan_fft_inverse(a.len() * 2 - 1);
    let mut output = fft.make_output_vec();
    fft.process(&mut a, &mut output).unwrap();
    output
}


pub fn multiply<T: Mul<Output=T> + Copy>(one: &mut [T], two: &[T]) {
    assert_eq!(one.len(), two.len());
    for (one, two) in one.iter_mut().zip(two.iter()) {
        *one = *one * *two;
    }
}

pub fn repeat<T: Copy>(v: &[T], desired_len: usize) -> Vec<T> {
    let mut out = Vec::with_capacity(desired_len);
    let per_elem = desired_len / v.len();
    let remainder = desired_len % v.len();

    for j in v {
        for _ in 0..per_elem {
            out.push(*j);
        }
    }

    for _ in 0..remainder {
        out.push(*v.last().unwrap());
    }

    out
}


pub fn fast_convolve(mut larger: Vec<f32>, mut smaller: Vec<f32>) -> Vec<f32> {
    let shift_length = smaller.len() / 2;
    let filt_sum: f32 = smaller.iter().sum();
    let size = larger.len() + smaller.len() - 1;

    // Zero pad vecs to size
    larger.resize(size, f32::default());
    smaller.resize(size, f32::default());

    let mut larger = fft(larger);
    let smaller = fft(smaller);

    multiply(&mut larger, &smaller);

    let mut result = ifft(larger);
    for i in 0..result.len() - shift_length {
        result[i] = result[i + shift_length] / filt_sum;
    }
    result
}

pub fn generate<T, F: Fn(f32) -> T>(func: F, range: Range<f32>, size: usize) -> Vec<T> {
    let range_size = range.end - range.start;
    let mut buf = Vec::with_capacity(size);
    for i in 0..size {
        let xval = i as f32 / size as f32 * range_size + range.start;
        buf.push(func(xval));
    };
    buf
}


const PQ: u64 = 1754338473;

pub fn blumblumshub(count: u32, seed: u64, max: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::with_capacity(count as usize);
    v.push(((seed * seed) % PQ) as u32);
    for i in 1..count as usize {
        v.push(((v[i - 1] as u64 * v[i - 1] as u64) % PQ) as u32);
    };
    for i in &mut v {
        *i %= max;
    }
    v
}