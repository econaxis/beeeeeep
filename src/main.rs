use std::f32::consts::PI;
use std::ops::Add;


use realfft::num_complex::Complex32;

use array::fast_convolve;

mod array;
mod demod;
mod log;
mod pcm_manager;


pub const SAMPLRATE: usize = 44100;
pub const DURATION: f32 = 2.5;
pub const SAMPLES: usize = ((SAMPLRATE as f32) * DURATION) as usize;
const BAUD_RATE: f32 = 40.;
pub const SYMBOLS: u32 = (BAUD_RATE * DURATION) as u32;
pub const FREQUENCY: u32 = 250;
const GAIN: f32 = 7000.;
const USE_PI4: bool = true;
const BLUMBLUMSEED: u64 = 21841;

fn angle_to_iq(angle: &[u32]) -> (Vec<f32>, Vec<f32>) {
    let mut ivec = Vec::with_capacity(angle.len());
    let mut qvec = ivec.clone();
    for (index, j) in angle.iter().enumerate() {
        let mut angle = (*j % 4) as f32 * PI / 2.;
        if index % 2 == 0 && USE_PI4 {
            angle += PI / 4.;
        }
        ivec.push(angle.cos());
        qvec.push(angle.sin());
    };
    (ivec, qvec)
}


fn send_chunks_audio(chunks: &[Vec<f32>]) -> Vec<i16> {
    assert!(chunks.iter().all(|a| a.len() == chunks[0].len()));
    let mut buf = Vec::new();
    buf.resize(chunks[0].len(), 0i16);

    for ind in 0..buf.len() {
        let mut val = 0.;
        for c in chunks {
            val += c[ind];
        }
        // val /= chunks.len() as f32;
        buf[ind] = (val * GAIN) as i16;
    }
    buf
}

fn join(t: &[u32]) -> String {
    t.iter().map(ToString::to_string).fold(String::with_capacity(t.len()), |accum, item| accum + " " + &item)
}

fn generate_blumblumsignal(count: u32, seed: u64, frequency: u32, samples: usize) -> Vec<f32> {
    let angle_data = array::blumblumshub(count, seed, 4);
    println!("Data: {}", join(&angle_data));
    generate_signal(&angle_data, frequency, samples)
}

fn main() {
    log::init("/tmp/debug");
    log::write_log(format_args!("{}", "{"));
    log::write_keyvalue("baud", BAUD_RATE);
    log::write_keyvalue("frequency", FREQUENCY);
    log::write_keyvalue("blumblumseed", BLUMBLUMSEED);


    let buf = send_chunks_audio(&[
        generate_blumblumsignal(SYMBOLS, 192, FREQUENCY * 2, SAMPLES),
        generate_blumblumsignal(SYMBOLS, 193, FREQUENCY * 3, SAMPLES),
        generate_blumblumsignal(SYMBOLS, 194, FREQUENCY * 4, SAMPLES),
        generate_blumblumsignal(SYMBOLS, 195, FREQUENCY * 5, SAMPLES),
        generate_blumblumsignal(SYMBOLS, 196, FREQUENCY * 6, SAMPLES),
        generate_blumblumsignal(SYMBOLS, 197, FREQUENCY * 7, SAMPLES),
        generate_blumblumsignal(SYMBOLS, 198, FREQUENCY * 8, SAMPLES),
        generate_blumblumsignal(SYMBOLS, 199, FREQUENCY * 9, SAMPLES),
        generate_blumblumsignal(SYMBOLS, 200, FREQUENCY * 10, SAMPLES),
        generate_blumblumsignal(SYMBOLS, 201, FREQUENCY * 11, SAMPLES),
    ]);
    log::write_log(format_args!("{}: {:?},", r#""transmit""#, buf));
    let mut buf = buf.as_slice();
    let mut readbuf = [0i16; (SAMPLES as f32 * 1.15) as usize];
    let (io, io1) = pcm_manager::setup_audio();
    loop {
        let written = io.writei(buf).unwrap();
        buf = &buf[written..];
        if written == 0 { break; }
    }
    pcm_manager::get_play_pcm().drain().unwrap();
    let read = io1.readi(&mut readbuf).unwrap();
    log::write_keyvalue("li", &readbuf[..read]);
    log::finish();
}

static mut RRCPTR: Option<Vec<f32>> = None;

unsafe fn rrc_filter() -> &'static [f32] {
    fn sinc(x: f32) -> f32 {
        #[allow(illegal_floating_point_literal_pattern)]
        match x {
            -0.01..=0.01 => 1.0,
            _ => (PI * x).sin() / PI / x
        }
    }

    const T: f32 = SAMPLRATE as f32 / BAUD_RATE as f32;
    const BETA: f32 = 3.0;
    const DISTANCE: f32 = T + 50.;
    match &RRCPTR {
        None => {
            RRCPTR = Some(array::generate(|a| {
                let val = if (a.abs() - T / 2. / BETA).abs() <= 0.05 {
                    PI / (4. * T) * sinc(0.5 / BETA)
                } else {
                    1. / T * sinc(a / T) * ((PI * BETA * a) / T).cos() / (1. - (2. * BETA * a / T).powi(2))
                };
                if !val.is_normal() {
                    panic!("{} {} {}", val, a, T / 2. / BETA);
                }
                val
            }, -DISTANCE..DISTANCE, DISTANCE as usize));
            let sum: f32 = RRCPTR.as_ref().unwrap().iter().sum();
            RRCPTR.as_mut().unwrap().iter_mut().for_each(|a| *a = *a / sum);
            log::write_log(format_args!("{}: {:?},", r#""filter""#, RRCPTR.as_ref().unwrap()));
            RRCPTR.as_ref().unwrap()
        }
        Some(x) => x
    }
}


fn generate_signal(angle_data: &[u32], frequency: u32, num_samples: usize) -> Vec<f32> {
    const HEADER: &[u32] = &[0, 0, 0, 0, 0, 0, 0, 0];
    const HEADERSAMPLES: usize = (HEADER.len() as f32 / BAUD_RATE * SAMPLRATE as f32) as usize;
    let angle_data: Vec<_> = [HEADER, angle_data].concat();
    let (isignal, qsignal) = angle_to_iq(&angle_data);

    let isignal = array::repeat(&isignal, num_samples + HEADERSAMPLES);
    let qsignal = array::repeat(&qsignal, num_samples + HEADERSAMPLES);

    let gauss_filter: Vec<_> = Vec::from(unsafe { rrc_filter() });


    let mut isignal = fast_convolve(isignal, gauss_filter.clone());
    let qsignal = fast_convolve(qsignal, gauss_filter);


    // Modify array isignal to produce our result
    for index in 0..isignal.len() {
        let seconds = index as f32 / SAMPLRATE as f32;
        let angle = 2.0 * PI * (frequency as f32) * seconds;
        let oscillator = Complex32::from_polar(1.0, angle);

        let im = oscillator.im * isignal[index] as f32;
        let re = oscillator.re * qsignal[index] as f32;

        isignal[index] = im + re;
    }
    isignal
}

