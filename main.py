import json
import math

import matplotlib.pyplot as plt
import numpy as np
import scipy.signal

FREQ: int
FREQRANGE: np.array
matched_filter: np.array
NORMAL_SAMPLES: int
ANSWER: np.array
SAMPLING_RATE = 44100
DURATION = 2.5


def blumblumshub(count, seed, maxelem):
    PQ = 1754338473
    for _ in range(0, count):
        seed = (seed * seed) % PQ
        yield seed % maxelem


def fft(array):
    yf = scipy.fft.fft(array)
    xf = scipy.fft.fftfreq(array.shape[0], 1 / SAMPLING_RATE)
    plt.plot(xf, np.abs(yf), alpha=0.3)
    return yf, xf


def lpf(array, totval, cutoff=15):
    cutoff = 2 * cutoff / totval
    sos = scipy.signal.butter(8, cutoff, output='sos')
    filtered_signal = scipy.signal.sosfilt(sos, array)
    return filtered_signal


def bpf(array, values):
    a = scipy.signal.butter(8, values, 'bandpass', output='sos')
    filtered_signal = scipy.signal.sosfilt(a, array)
    return filtered_signal * 2


def agc(array):
    level = np.convolve(array, np.ones(100), 'same') / 100
    level += (level == 0) * 1
    return np.divide(array, level)


def getangle(array):
    array = np.angle(array)
    cutoff = np.pi
    for index, a in np.ndenumerate(array[1:]):
        index = index[0]
        diff = array[index] - array[index - 1]

        if diff > cutoff:
            array[index] -= 2 * np.pi
        elif diff < -cutoff:
            array[index] += 2 * np.pi
    return array


def eval_gardner(angles, index):
    right = angles[index + NORMAL_SAMPLES // 2]
    left = angles[index - NORMAL_SAMPLES // 2]
    middle = angles[index]
    shift = (right + left) / 2

    value = (right - left) * (middle - shift)
    return value


def do_gardner_search(angles, index):
    value = eval_gardner(angles, index)

    if value >= -0.1:
        return None

    for _ in range(0, 100000):
        if value < -0.01:
            index += 1
            value = eval_gardner(angles, index)
        elif 0 < value < 0.05:
            return index - 1
        elif value > 0.05:
            return None
        else:
            return index


def demod(array):
    secs = array.shape[0] / SAMPLING_RATE
    oscrange = np.linspace(0, secs, array.shape[0]) * 2 * np.pi * FREQ
    oscillator = np.cos(oscrange) + 1j * np.sin(oscrange)
    array = bpf(array, FREQRANGE * 2 / 44100)

    array = lpf(np.multiply(array, oscillator), len(array), 100)
    array = np.convolve(array, matched_filter, 'same')
    array = getangle(array)
    plt.plot(array)

    i = NORMAL_SAMPLES // 2
    best_headers = []
    while i < array.shape[0] - NORMAL_SAMPLES // 2:
        temp_ind = i
        count = 0
        error = 0
        prevsign = False

        while temp_ind + NORMAL_SAMPLES // 2 < array.shape[0]:
            leftval = array[temp_ind - NORMAL_SAMPLES // 2]
            rightval = array[temp_ind + NORMAL_SAMPLES // 2]

            is_middle = (array[temp_ind] - (leftval + rightval) / 2) ** 2
            is_pi_4 = (abs(leftval - rightval) - np.pi / 4) ** 2
            sign = leftval > rightval
            cutoff_vals = 0.45

            if count >= 4:
                cutoff_vals /= 2

            if is_middle < cutoff_vals and is_pi_4 < cutoff_vals and prevsign is not sign:
                temp_ind += NORMAL_SAMPLES
                count += 1
                error += is_middle * count ** 2 + is_pi_4 * count ** 2
                prevsign = sign
            else:
                break
        error += 0.002 * math.sqrt(i)
        if count == 6:
            best_headers.append((i + 11 * NORMAL_SAMPLES // 2, error))
        elif count < 6 and len(best_headers) > 0:
            if best_headers[-1][1] <= 10 and abs(best_headers[-1][0] - i) > 1000:
                break

        i += 1

    best_headers = sorted(best_headers, key=lambda k: k[1])
    plt.plot(*zip(*best_headers[0:len(best_headers) // 5]), marker='1', markersize=5, linestyle='')
    if len(best_headers):
        working_index, error = best_headers[0]
        zero_point = array[working_index]

        plt.plot([working_index], [zero_point], marker='x', markersize=4, color='black')
        # Now i contains the working value
        samples = []
        should_pi4 = False
        for i in range(working_index, min(working_index + int(SAMPLING_RATE * DURATION), array.shape[0]),
                       NORMAL_SAMPLES):
            angle = array[i]
            plt.plot([i], [angle], marker='o', markersize=3, color='red')

            padding = np.pi / 4
            angle -= zero_point
            if should_pi4:
                angle -= np.pi / 4
            should_pi4 = not should_pi4
            angle *= -1
            while angle < -np.pi + padding:
                angle += np.pi * 2
            while angle > np.pi + padding:
                angle -= np.pi * 2

            if angle < -np.pi / 2 + padding:
                bit_decision = 3
            elif angle < 0 + padding:
                bit_decision = 0
            elif angle < np.pi / 2 + padding:
                bit_decision = 1
            elif angle < np.pi + padding:
                bit_decision = 2
            else:
                bit_decision = -1

            samples.append(bit_decision)
        large_len = min(len(samples), len(ANSWER))
        print("Sample: ", samples)
        print("Error:", samples[:large_len] - ANSWER[:large_len])
    return array


def load():
    global matched_filter, FREQ, FREQRANGE, NORMAL_SAMPLES, header_filter, ANSWER
    f = json.load(open("/tmp/debug", "r"))
    li = np.asarray(f["li"])
    matched_filter = np.asarray(f["filter"])
    FREQ = f["frequency"] * 2
    FREQRANGE = np.array([FREQ - 200, FREQ + 200])
    NORMAL_SAMPLES = int(44100 / f["baud"])
    header_filter = np.exp(-np.arange(-8, 8, 16 / NORMAL_SAMPLES) ** 2 * 0.3)

    ANSWER = np.array(
        list(blumblumshub(int(f["baud"] * DURATION), f["blumblumseed"], 4)))
    l1 = demod(li)
    plt.show()


load()
