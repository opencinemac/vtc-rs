<h1 align="center">vtc-rs</h1>
<p align="center">
    <img height=150 class="heightSet" align="center" src="https://raw.githubusercontent.com/opencinemac/vtc-py/master/zdocs/source/_static/logo1.svg"/>
</p>
<p align="center">A SMPTE Timecode Library for Rust</p>
<p align="center">
    <a href="https://dev.azure.com/peake100/Open%20Cinema%20Collective/_build?definitionId=15"><img src="https://dev.azure.com/peake100/Open%20Cinema%20Collective/_apis/build/status/vtc-rs?repoName=opencinemac%2Fvtc-rs&branchName=dev" alt="click to see build pipeline"></a>
    <a href="https://dev.azure.com/peake100/Open%20Cinema%20Collective/_build?definitionId=15"><img src="https://img.shields.io/azure-devops/tests/peake100/Open%20Cinema%20Collective/15/dev?compact_message" alt="click to see build pipeline"></a>
    <a href="https://dev.azure.com/peake100/Open%20Cinema%20Collective/_build?definitionId=15"><img src="https://img.shields.io/azure-devops/coverage/peake100/Open%20Cinema%20Collective/15/dev?compact_message" alt="click to see build pipeline"></a>
</p>
<p align="center">
    <a href="https://crates.io/crates/vtc"><img src="https://img.shields.io/crates/v/vtc" alt="PyPI version" height="18"></a>
    <a href="https://docs.rs/vtc"><img src="https://docs.rs/vtc/badge.svg" alt="Documentation"></a>
</p>

# Overview

``vtc-rs`` is inspired by years of scripting workflow solutions in a Hollywood cutting
room. It aims to capture all the ways in which timecode is used throughout the industry so
users can spend more time on their workflow logic, and less time handling the
corner-cases of parsing and calculating timecode.

## Demo

Let's take a quick high-level look at what you can do with vtc-rs:

```rust
use vtc::{Timecode, Framerate, Ntsc, rates};
use num::Rational64;

// It's easy to make a new 23.98 NTSC timecode. We use the with_frames constructor here since
// timecode is really a human-readable way to represent frame count.
let mut tc = Timecode::with_frames("17:23:13:02", rates::F23_98).unwrap();

// We can get all sorts of ways to represent the timecode.
assert_eq!(tc.timecode(), "17:23:13:02");
assert_eq!(tc.frames(), 1502234i64);
assert_eq!(tc.seconds(), Rational64::new(751868117, 12000));
assert_eq!(tc.runtime(3), "17:24:15.676");
assert_eq!(tc.premiere_ticks(), 15915544300656000i64);
assert_eq!(tc.feet_and_frames(), "93889+10");

// We can inspect the framerate.
assert_eq!(tc.rate().playback(), Rational64::new(24000, 1001));
assert_eq!(tc.rate().timebase(), Rational64::new(24, 1));
assert_eq!(tc.rate().ntsc(), Ntsc::NonDropFrame);

// Parsing is flexible

// Partial timecode:
let parsed = Timecode::with_frames("3:12", rates::F23_98).unwrap();
assert_eq!(parsed.timecode(), "00:00:03:12");

// Frame count:
let parsed = Timecode::with_frames(24, rates::F23_98).unwrap();
assert_eq!(parsed.timecode(), "00:00:01:00");

// Seconds:
let parsed = Timecode::with_seconds(1.5, rates::F23_98).unwrap();
assert_eq!(parsed.timecode(), "00:00:01:12");

// Premiere Ticks:
let parsed = Timecode::with_premiere_ticks(254016000000i64, rates::F23_98).unwrap();
assert_eq!(parsed.timecode(), "00:00:01:00");

// Feet + Frames:
let parsed = Timecode::with_frames("1+08", rates::F23_98).unwrap();
assert_eq!(parsed.timecode(), "00:00:01:00");

// We can add two timecodes
tc += Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
assert_eq!(tc.timecode(), "18:23:13:02");

// We can subtract too.
tc -= Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
assert_eq!(tc.timecode(), "17:23:13:02");

// It's easy to compare two timecodes:
assert!(tc > Timecode::with_frames("02:00:00:00", rates::F23_98).unwrap());

// And sort them:
let mut sorted = vec![tc, Timecode::with_frames("02:00:00:00", rates::F23_98).unwrap()];
sorted.sort();

assert_eq!(sorted[0].timecode(), "02:00:00:00");
assert_eq!(sorted[1].timecode(), "17:23:13:02");

// We can multiply:
tc *= 2;
assert_eq!(tc.timecode(), "34:46:26:04");

// ...divide... :
tc /= 2;
assert_eq!(tc.timecode(), "17:23:13:02");

// ... and even get the remainder of division!
let dividend = tc / 1.5;
let remainder = tc % 1.5;

assert_eq!(dividend.timecode(), "11:35:28:17");
assert_eq!(remainder.timecode(), "00:00:00:01");

// We can make a timecode negative:
tc = -tc;
assert_eq!(tc.timecode(), "-17:23:13:02");

// Or get it's absolute value.
tc = tc.abs();
assert_eq!(tc.timecode(), "17:23:13:02");

// We can make dropframe timecode for 29.97 or 59.94 using one of the pre-set framerates.
// We can use an int to parse 15000 frames.
let drop_frame = Timecode::with_frames(15000, rates::F29_97_DF).unwrap();
assert_eq!(drop_frame.timecode(), "00:08:20;18");
assert_eq!(drop_frame.rate().ntsc(), Ntsc::DropFrame);

// We can make new timecodes with arbitrary framerates if we want:
let arbitrary = Timecode::with_frames(
    "01:00:00:00",
    Framerate::with_playback(48, Ntsc::None).unwrap(),
).unwrap();
assert_eq!(arbitrary.frames(), 172800);

// We can make NTSC values for timebases and playback speeds that do not ship with this
// crate:
let mut ntsc = Timecode::with_frames(
    "01:00:00:00",
    Framerate::with_timebase(120, Ntsc::NonDropFrame).unwrap(),
).unwrap();
assert_eq!(ntsc.rate().playback(), Rational64::new(120000, 1001));
assert_eq!(ntsc.rate().timebase(), Rational64::new(120, 1));
assert_eq!(ntsc.rate().ntsc(), Ntsc::NonDropFrame);

// We can also rebase them using another framerate:
ntsc = ntsc.rebase(rates::F59_94_NDF);
assert_eq!(ntsc.timecode(), "02:00:00:00");
```

## Features

- SMPTE Conventions:
    - [X] NTSC
    - [X] Drop-Frame
    - [ ] Interlaced timecode
- Timecode Representations:
    - Timecode    | '01:00:00:00'
    - Frames      | 86400
    - Seconds     | 3600.0
    - Runtime     | '01:00:00.0'
    - Rational    | 18018/5
    - Feet+Frames | '5400+00'
        - [X] 35mm, 4-perf
        - [ ] 35mm, 3-perf
        - [ ] 35mm, 2-perf
        - [ ] 16mm
    - Premiere Ticks | 15240960000000
- Operations:
    - Comparisons (==, <, <=, >, >=)
    - Add
    - Subtract
    - Scale (multiply and divide)
    - Div/Rem
    - Modulo
    - Negative
    - Absolute
    - Rebase (recalculate frame count at new framerate)
- Flexible Parsing:
    - Partial timecodes      | '1:12'
    - Partial runtimes       | '1.5'
    - Negative string values | '-1:12', '-3+00'
    - Poorly formatted tc    | '1:13:4'
- Built-in consts for common framerates.

## Goals

- Parse and fetch all Timecode representations.
- A clean, rustic API.
- Support all operations that make sense for timecode.

## Non-Goals

- Real-time timecode generators.

## Attributions

<div>Drop-frame calculations adapted from <a href="https://www.davidheidelberger.com/2010/06/10/drop-frame-timecode/">David Heidelberger's blog.</a></div>
<div>Logo made by <a href="" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>
