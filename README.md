# Sending data at 1 kbps over audio

*Pictures near the end*

This repository is a demonstration of sending bits over audio, then receiving them, demodulating the signals, and getting
back the original message.

The modulation scheme is quadrature phase shift keying (QPSK). I use a variant
called [pi/4 QPSK](https://en.wikipedia.org/wiki/Phase-shift_keying#%CF%80/4-QPSK).

## What's QPSK?

To transmit information over audio (or radio), we have to encode our bits into a wave. There's various ways of doing
this. The easiest way would be turning the wave off/on depending on a 0 or 1 bit (on/off keying). Another way is to
change the frequency of the wave depending on the bit (frequency keying). Here, I choose to change the phase of the
wave.

We send two bits at a time. The phase-difference of the transmitted wave with respect to a local, reference wave
represents the bits being sent. A *00* is a 0-radian phase difference, a *01* is a pi/2 radian phase difference, and so
on.

At the receiving end, we simply multiply the sent wave with a copy of the local oscillator, do some filtering, and *
magically* recover the original data.

## Clock Recovery

The hardest problem is clock recovery. At the receiving end, we have no idea what the phase of the local oscillator is.
We also don't know when the symbols "start" and "stop." The most common way to do clock recovery is to send a stream of
0's at the beginning. The receiving end then searches the signal for something that looks like a bunch of zeros, then
samples the message from there.

## Improving performance - 10 streams at 50 baud

The Fourier transform allows us to separate and isolate different frequencies. Therefore, we can "overlay", or
multiplex, multiple data streams at once by having each data stream be a different frequency. If each stream can send 50
symbols per second, then sending 10 streams at once lets us send 500 symbols per second.

This is also called Orthogonal Frequency Division Multiplexing (OFDM), and is also used by Wifi and the new 5G standard.

#### Sent data

To demonstrate multiplexing, I send ten streams of data of random base-4 numbers. Each stream consists of 125 symbols
sent at a rate of 50 baud, which means the transmission lasts for 2.5 seconds. In total, this means 100 bits per second
for each stream. When sending ten streams, this results in a total 1 kilobit per second transmission for this audio
scheme. The frequencies of each data stream are different.

```text
Data:  0 0 2 0 1 3 2 1 2 1 0 0 3 2 2 3 0 2 0 1 3 1 1 0 1 0 2 0 2 1 0 0 0 0 3 0 0 2 3 2 3 0 1 3 3 1 2 2 0 3 2 2 3 3 0 3 3 0 3 1 1 0 0 1 0 1 1 1 0 1 3 2 1 1 3 1 2 3 2 2 2 2 3 3 0 0 0 3 0 0 2 2 3 2 1 2 0 3 2 0 3 3 1 3 0 3 1 2 1 1 0 1 2 1 3 3 0 0 1 3 2 0 1 3 1
Data:  1 1 1 1 0 3 0 1 2 3 2 0 2 2 1 0 2 3 0 2 1 1 3 2 1 2 1 1 3 3 2 3 0 0 3 0 3 1 3 2 0 2 2 3 2 3 2 2 0 1 2 3 3 3 0 1 1 2 0 2 1 2 3 3 3 0 0 1 0 1 3 3 3 0 2 2 3 3 0 3 3 3 3 0 3 2 2 0 2 2 3 2 2 0 2 2 3 0 2 1 0 1 2 1 1 1 2 1 2 0 3 0 1 1 1 0 0 3 0 2 0 2 3 3 0
Data:  0 0 3 0 3 0 2 1 1 3 2 0 0 1 1 0 2 2 2 1 3 0 3 0 1 0 2 0 2 3 2 1 1 0 0 3 2 0 0 0 3 3 3 3 1 3 1 1 2 0 2 0 1 3 3 0 1 0 3 2 1 0 0 2 2 0 0 0 1 3 0 2 0 2 3 0 0 0 0 1 1 2 3 1 0 3 3 1 1 3 1 0 0 2 1 0 3 2 3 0 2 3 0 3 0 3 2 1 3 0 0 3 3 2 2 2 0 1 0 3 1 3 0 3 3
Data:  1 1 2 0 3 1 1 1 2 0 2 2 1 1 2 3 2 3 0 3 1 0 3 1 2 2 1 3 1 3 1 0 3 2 1 3 3 0 1 1 0 2 1 2 0 3 1 2 0 1 2 1 1 1 0 3 2 1 2 0 2 3 0 3 1 0 3 2 3 1 3 3 2 2 2 0 3 2 1 2 2 2 1 0 3 2 1 1 2 2 0 0 0 1 0 1 0 2 3 0 1 3 1 1 3 3 2 3 3 1 0 3 3 1 0 0 0 1 2 2 3 1 2 0 2
Data:  0 0 3 3 1 2 2 0 1 2 2 2 1 2 0 2 2 0 1 3 2 0 2 2 3 3 3 1 2 1 1 1 1 2 0 2 2 1 0 2 2 1 2 2 2 0 0 2 3 1 1 3 3 3 3 2 1 0 3 3 2 0 0 1 0 1 2 0 2 2 0 2 2 2 0 2 1 3 0 0 1 3 3 3 1 1 1 3 3 3 1 3 2 2 2 2 1 3 1 1 3 2 0 0 3 3 2 2 2 2 2 0 3 0 2 3 1 0 2 2 1 3 1 2 0
Data:  1 1 1 2 1 2 2 1 1 3 3 2 1 0 1 1 1 0 3 1 3 2 2 3 0 1 1 2 3 0 3 3 2 0 0 1 2 1 3 1 3 1 2 1 3 2 1 2 3 2 2 3 3 0 2 2 1 2 3 2 3 0 2 0 0 2 3 0 2 2 2 1 2 1 0 2 3 2 3 3 3 3 0 0 2 2 2 0 1 2 1 0 3 2 2 1 1 0 0 0 1 0 3 1 1 0 0 0 2 3 2 3 3 1 3 3 1 3 0 2 2 2 3 3 1
Data:  0 0 0 2 1 3 1 2 3 0 1 0 2 0 0 0 1 1 2 2 0 0 3 0 2 0 0 2 3 0 1 1 1 3 2 2 2 2 3 2 2 2 0 1 2 2 0 1 3 1 0 2 2 1 1 1 2 1 2 2 1 3 2 2 2 0 0 1 2 3 3 1 3 2 1 3 1 0 1 2 2 2 2 2 2 2 1 3 2 2 1 3 2 0 0 1 3 1 3 3 0 1 1 0 0 3 1 3 2 1 3 3 2 3 1 0 3 0 3 0 2 3 1 2 1
Data:  1 1 3 3 1 1 2 2 0 2 2 3 3 3 1 3 0 2 2 2 3 3 0 3 0 0 3 3 3 0 1 1 0 0 3 3 2 0 2 2 2 3 0 0 3 3 3 0 3 1 3 2 3 2 1 1 3 1 2 3 2 3 1 0 1 3 2 3 0 0 3 0 1 1 1 2 3 3 1 2 1 1 0 3 3 0 3 3 1 1 1 2 2 2 1 2 0 2 2 0 0 0 0 0 0 1 0 3 2 1 2 1 2 0 2 1 2 2 1 2 0 0 3 2 3
Data:  0 0 2 2 0 2 2 1 3 1 2 1 3 3 1 2 0 2 0 2 0 2 1 0 0 0 1 0 1 3 1 1 1 2 2 2 3 2 2 2 2 3 2 3 3 1 1 0 1 2 2 3 2 3 3 0 3 2 2 2 0 2 0 0 2 3 2 3 2 1 3 0 0 0 3 1 2 2 2 0 1 2 1 3 0 1 0 2 1 0 1 2 0 2 3 3 2 1 0 2 2 3 2 0 3 3 0 2 1 0 1 2 1 0 1 3 2 0 0 3 3 3 3 1 1
Data:  1 1 1 0 1 0 0 0 1 1 2 0 3 3 3 1 1 3 1 0 3 0 2 1 3 1 1 0 3 3 0 2 0 0 2 0 3 0 3 3 2 1 2 1 2 2 3 3 2 0 3 1 0 2 3 2 0 0 1 1 3 3 3 0 1 0 2 0 3 2 0 1 1 2 1 2 3 1 3 2 1 2 3 0 3 0 1 0 3 0 2 1 2 1 1 3 0 0 2 3 1 0 0 1 3 2 0 2 0 0 2 3 1 3 0 1 3 2 2 2 3 1 3 0 2
```

### Received data

At the receiving end, we filter the audio for each specific frequency. Then, we demodulate and decode the data. The data
is mostly correct, but there are some bit errors that is inevitable of any audio based data transmission. The errors are
bolded and highlighted here.

<pre>
Data: 0 0 2 0 1 3 2 1 2 1 0 0 3 2 2 3 0 2 0 1 3 1 1 0 1 0 2 0 2 1 0 0 0 0 3 0 0 2 3 2 3 0 1 3 3 1 2 2 0 3 2 2 3 3 0 3 3 0 3 1 1 0 0 1 0 1 1 1 0 1 3 2 1 1 3 1 2 3 2 2 2 2 3 3 0 0 0 3 0 0 2 2 3 2 1 2 0 3 2 0 3 3 1 3 0 3 1 2 1 1 0 1 2 1 3 3 0 0 1 3 2 0 1 3 1
Data: 1 1 1 1 0 3 0 1 2 3 2 0 2 2 1 0 2 3 0 2 1 1 3 2 1 2 1 1 3 3 2 3 0 0 3 0 3 1 3 2 0 2 2 3 2 3 2 2 0 1 2 3 3 3 0 1 1 2 0 2 1 2 3 3 3 0 0 1 0 1 3 3 3 0 2 2 3 3 0 3 3 3 3 <b><i>3</i></b> 3 2 2 0 2 2 3 2 2 0 2 2 3 0 2 1 0 1 2 1 1 1 2 1 2 0 3 0 1 1 1 0 0 3 0 2 0 2 3 3 0
Data: 0 0 3 0 3 0 2 1 1 3 2 0 0 1 1 0 <b><i>1</i></b> 2 <b><i>1</i></b> 1 3 0 3 0 1 0 2 0 2 3 2 1 1 0 0 3 2 0 0 0 3 3 3 3 1 3 1 <b><i>0</i></b> 2 0 2 0 1 3 3 0 1 0 3 2 1 0 0 2 2 0 0 0 1 3 0 2 0 2 3 0 0 0 0 1 1 2 3 1 0 3 3 1 1 3 1 0 0 2 1 0 3 2 3 0 2 3 0 3 0 3 2 1 3 0 0 3 3 2 2 2 0 1 0 3 1 3 0 3 3
Data: 1 1 2 0 3 1 1 1 2 0 2 2 1 1 2 3 2 <b><i>2</i></b> 0 3 1 0 3 1 2 2 1 3 1 3 1 0 3 2 1 3 3 0 1 1 0 2 1 2 0 3 1 2 0 1 2 1 1 1 0 3 2 1 2 0 2 3 0 3 1 0 3 2 3 1 3 3 2 2 2 0 3 2 <b><i>0</i></b> 2 2 2 1 0 3 2 1 1 2 2 0 0 0 1 0 1 0 2 3 0 1 3 1 1 3 3 2 3 3 1 0 3 3 1 0 0 0 1 2 2 3 1 2 0 2
Data: 0 0 3 3 1 2 2 0 1 2 2 2 1 2 0 2 2 0 1 3 2 0 2 <b><i>1</i></b> 3 <b><i>2</i></b> 3 1 2 1 1 1 1 2 0 2 <b><i>1</i></b> 1 0 2 <b><i>1</i></b> 1 2 2 2 0 0 2 3 1 1 3 3 3 3 <b><i>1</i></b> 1 0 3 3 2 0 0 <b><i>0</i></b> 0 1 2 0 2 2 0 2 2 2 0 2 1 3 0 0 1 3 3 3 1 1 1 3 3 3 1 <b><i>2</i></b> 2 2 2 2 1 3 1 1 3 2 <b><i>3</i></b> 0 3 3 2 2 2 <b><i>1</i></b> 2 0 3 <b><i>3</i></b> 2 3 1 <b><i>3</i></b> 2 <b><i>1</i></b> 1 <b><i>2</i></b> 1 2 0
Data: <b><i>0</i></b> 1 <b><i>0</i></b> 2 <b><i>0</i></b> 2 2 1 1 3 3 2 <b><i>0</i></b> 0 1 1 <b><i>0</i></b> <b><i>3</i></b> 3 <b><i>0</i></b> 3 2 2 3 0 1 <b><i>0</i></b> 2 3 0 3 <b><i>2</i></b> <b><i>1</i></b> 0 <b><i>3</i></b> 1 2 1 <b><i>2</i></b> 1 3 1 <b><i>1</i></b> <b><i>0</i></b> 3 2 1 2 3 2 2 <b><i>2</i></b> 3 0 <b><i>1</i></b> 2 1 2 3 2 3 0 <b><i>1</i></b> 0 0 2 3 <b><i>3</i></b> 2 2 <b><i>1</i></b> 1 2 1 0 <b><i>1</i></b> 3 2 3 <b><i>2</i></b> 3 3 0 <b><i>3</i></b> 2 2 2 0 1 2 1 0 <b><i>2</i></b> 2 2 1 1 0 0 0 <b><i>0</i></b> 0 3 1 1 0 0 <b><i>3</i></b> 2 3 2 3 3 1 <b><i>2</i></b> 3 <b><i>0</i></b> 3 0 2 2 2 3 3 <b><i>0</i></b>
Data: 0 0 0 2 1 3 1 2 3 0 1 0 2 0 0 0 1 1 2 2 0 0 3 <b><i>3</i></b> 2 0 0 2 3 0 <b><i>0</i></b> 1 1 3 2 2 <b><i>1</i></b> 2 3 2 2 <b><i>1</i></b> 0 1 2 <b><i>1</i></b> 0 1 3 1 0 2 2 1 1 1 2 1 2 2 1 3 2 2 2 0 0 1 2 <b><i>2</i></b> 3 1 <b><i>2</i></b> <b><i>1</i></b> 1 3 <b><i>0</i></b> 0 1 2 <b><i>1</i></b> 2 2 2 2 2 1 3 <b><i>1</i></b> 2 1 3 2 <b><i>3</i></b> <b><i>3</i></b> <b><i>0</i></b> 3 <b><i>0</i></b> 3 3 0 1 1 0 <b><i>3</i></b> 3 1 3 <b><i>1</i></b> 1 <b><i>2</i></b> 3 <b><i>1</i></b> <b><i>2</i></b> <b><i>0</i></b> <b><i>3</i></b> 3 0 3 0 2 3 1 <b><i>1</i></b> 1
Data: 1 1 3 3 1 1 2 2 0 2 2 3 3 3 1 3 0 2 2 2 3 3 0 3 0 0 3 3 3 <b><i>1</i></b> 1 1 0 0 3 3 2 0 2 2 2 3 0 0 3 3 3 0 3 1 3 2 3 2 1 1 3 1 2 3 2 3 1 0 1 3 2 3 0 0 3 0 1 1 1 2 3 3 1 2 1 1 0 3 3 0 3 3 1 1 1 2 2 2 1 2 0 2 2 0 0 0 0 0 0 1 0 3 2 1 2 1 2 0 2 1 2 2 1 2 0 0 3 2 3
Data: 0 0 2 2 0 2 2 1 3 1 2 1 3 3 1 2 0 2 0 2 0 2 1 0 0 0 1 0 1 3 1 1 1 2 2 2 3 2 2 2 2 3 2 3 3 1 1 0 1 2 2 3 2 3 3 0 3 2 2 2 0 2 0 0 2 3 2 3 2 1 3 0 0 0 3 1 2 2 2 0 1 2 1 3 0 1 0 2 1 0 1 2 0 2 3 3 2 1 0 2 2 3 2 0 3 3 0 2 1 0 1 2 1 0 1 3 2 0 0 3 3 3 3 1 1
Data: 1 1 1 0 1 0 0 0 1 1 2 0 3 3 3 1 1 3 1 0 3 0 2 1 3 1 1 0 3 3 0 2 0 0 2 0 3 0 3 3 2 1 2 1 2 2 3 3 2 0 3 1 0 2 3 2 0 0 1 1 3 3 3 0 1 0 2 0 3 2 0 1 1 2 1 2 3 1 3 2 1 2 3 0 3 0 1 0 3 0 2 1 2 1 1 3 0 0 2 3 1 0 0 1 3 2 0 2 0 0 2 3 1 3 0 1 3 2 2 2 3 1 3 0 2

</pre>

# Pictures

These are some pictures and graphs demonstrating the whole concept. I played audio through my earbuds, and recorded
audio through the built-in microphones. The only communication between the demodulator and the modulator is this audio
physical layer.

**The original transmitted audio (blue) vs the received audio (orange)**:

<img src="https://github.com/econaxis/beeeeeep/blob/main/transmitted-received.png?raw=true" width="533px" height="400px">

The FFT visualization of the received signal:

<img srhttps://github.com/econaxis/beeeeeep/blob/mainc="/fft.png?raw=true" width=533px" height=400px>

As you can see, each peak corresponds to a different data stream. This image was saved when I sent 11 data streams (
instead of 10, like the previous example), hence why there are 11 peaks evenly spaced apart.

**Filtering out a specific frequency (stream)**:

<img src="https://github.com/econaxis/beeeeeep/blob/main/bpfed.png?raw=true" width="533px" height=400px">

We extract a specific range of frequencies corresponding to one data stream for demodulation.

**After demodulating the signal into its baseband part and plotting the phase offset angle**:

<img src="https://github.com/econaxis/beeeeeep/blob/main/timing_recovery.png?raw=true" width="533px" height="400px">

The header of eight zero bits is evident near the beginning (just to the left of the black X). But why does zero not
have a phase offset of zero? This is because we haven't recovered the clock signal yet. We don't know what the phase of
the local oscillator at the other end. Thus, when we try to demodulate this signal, we end up with a "rotated" version.
Our "zero" point is not at 0, but at, according to the graph, around -2.3.

Knowing this "error", we can reverse it.

This image also shows us detecting the presence of a header. The orange points are potential header locations, and the
black X is the chosen header location. From this black X, we have determined the zero point (-2.3) as well as when to
sample for each symbol.

**Clock recovery in depth**:

<img src="https://github.com/econaxis/beeeeeep/blob/main/timing_recovery1.png?raw=true" width="533px" height="400px">

Through a bunch of heuristics and trial-error, we choose the X to maximize our chances of sampling at the right time and
getting the correct bit.

**Choosing points to evaluate symbol decisions**:

<img src="https://github.com/econaxis/beeeeeep/blob/main/evaluating.png?raw=true" width="533px" height="400px">

The red dots are where we will sample for each symbol. Note how each red dot is conveniently located on a local min/max,
thus maximizing our chances of guessing the correct bit.

**Fixing zero offsets**:

<img src="https://github.com/econaxis/beeeeeep/blob/main/evaluating_zero_offset.png?raw=true" width="533px" height="400px">

As mentioned before, the zero point was actually at -2.23, not 0. We fix this, as well as fix some other issues and do
more processing. All phase offsets now lie between 0 and 2pi.

**Clustering**:

<img src="https://github.com/econaxis/beeeeeep/blob/main/evaluating_clustered.png?raw=true" width="533px" height="400px">

All the phase offsets are now clustered around multiples of pi/2 radians. We now interpret the bits from the offset,
yielding the final message, which is typed above.

### All other streams

Of course, if we're sending 11 streams multiplexed, there are ten other streams that we have to do this same process.
However, the clock recovery is already done, so we can just sample and interpret symbols without doing clock recovery.

Here are the rest of the data streams, as well as their symbol locations.

<p float="left">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/2-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/3-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/4-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/5-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/6-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/7-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/8-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/9-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/10-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/11-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
<img src = "https://github.com/econaxis/beeeeeep/blob/main/12-ofdm.png?raw=true" width = "200px" margin = "50%" height = "150px">
</p>