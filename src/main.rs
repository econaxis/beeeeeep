use std::f32::consts::PI;




use realfft::num_complex::Complex32;

use array::fast_convolve;

mod array;
mod demod;
mod log;
mod pcm_manager;


pub const SAMPLRATE: usize = 44100;
pub const DURATION: f32 = 2.5;
pub const SAMPLES: usize = ((SAMPLRATE as f32) * DURATION) as usize;
const BAUD_RATE: f32 = 50.;
pub const SYMBOLS: u32 = (BAUD_RATE * DURATION) as u32;
pub const FREQUENCY: u32 = 175;
const GAIN: f32 = 2000.0;
const USE_PI4: bool = true;
const BLUMBLUMSEED: u64 = 5040;

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

fn main() {
    log::init("/tmp/debug");
    log::write_log(format_args!("{}", "{"));
    log::write_keyvalue("baud", BAUD_RATE);
    log::write_keyvalue("frequency", FREQUENCY);
    log::write_keyvalue("blumblumseed", BLUMBLUMSEED);
    let angle_data = array::blumblumshub(SYMBOLS, BLUMBLUMSEED, 4);
    // let angle_data = array::repeat(&[1, 2, 3, 0], BAUD_RATE as usize * 2);

    println!("Data1: {:?}", angle_data);
    let transmitted = generate_signal(&angle_data, FREQUENCY * 2, SAMPLES);

    let buf = send_chunks_audio(&[transmitted]);
    log::write_log(format_args!("{}: {:?},", r#""transmit""#, buf));
    let mut buf = buf.as_slice();
    let mut readbuf = [0i16; (SAMPLES as f32 * 1.05) as usize];
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
    const BETA: f32 = 2.0;
    const DISTANCE: f32 = T;
    match &RRCPTR {
        None => {
            RRCPTR = Some(array::generate(|a| {
                let val = if (a.abs() - T / 2. / BETA).abs() <= 0.05 {
                    0.1 * PI / (4. * T) * sinc(0.5 / BETA)
                } else {
                    0.1 * 1. / T * sinc(a / T) * ((PI * BETA * a) / T).cos() / (1. - (2. * BETA * a / T).powi(2))
                };
                if !val.is_normal() {
                    panic!("{} {} {}", val, a, T / 2. / BETA);
                }
                val
            }, -DISTANCE..DISTANCE, DISTANCE as usize));
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
    println!("Done processing");
    isignal
}

