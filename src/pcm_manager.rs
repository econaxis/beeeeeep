use alsa::{Direction, PCM, ValueOr};
use alsa::pcm::{Access, Format, HwParams, IO};

use crate::SAMPLRATE;

static mut RECORDER: Option<PCM> = None;
static mut PLAYER: Option<PCM> = None;

pub(crate) fn get_play_pcm() -> &'static PCM {
    if unsafe { PLAYER.is_none() } {
        let val = PCM::new("default", Direction::Playback, false).unwrap();
        unsafe {
            PLAYER = Some(val);
        }
    }
    unsafe { PLAYER.as_ref().unwrap() }
}

fn get_record_pcm() -> &'static PCM {
    if unsafe { RECORDER.is_none() } {
        let val = PCM::new("default", Direction::Capture, false).unwrap();
        unsafe { RECORDER = Some(val); }
    }
    unsafe {
        RECORDER.as_ref().unwrap()
    }
}

fn default_hw_params(pcm: &PCM, sampling_rate: u32) {
    let hwp = HwParams::any(pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(sampling_rate, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
}

pub(crate) fn setup_audio() -> (IO<'static, i16>, IO<'static, i16>) {
    let pcm = get_play_pcm();
    let recorder = get_record_pcm();
// Set hardware parameters: 44100 Hz / Mono / 16 bit
    default_hw_params(pcm, SAMPLRATE as u32);
    default_hw_params(recorder, SAMPLRATE as u32);

    let io = pcm.io_i16().unwrap();
    let io1 = recorder.io_i16().unwrap();
    // Make sure we don't start the stream too early
    let _hwp = pcm.hw_params_current().unwrap();
    let swp = pcm.sw_params_current().unwrap();
    pcm.sw_params(&swp).unwrap();
    pcm.resume().unwrap();
    recorder.resume().unwrap();
    (io, io1)
}