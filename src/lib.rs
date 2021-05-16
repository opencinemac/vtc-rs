mod consts;
mod errors;

mod framerate;
mod framerate_parse;
#[cfg(test)]
mod framerate_test;

mod timecode;
mod timecode_parse;
#[cfg(test)]
mod timecode_test_ops;
#[cfg(test)]
mod timecode_test_parse;
#[cfg(test)]
mod timecode_test_table;

mod source_frames;
mod source_ppro_ticks;
mod source_seconds;

pub use errors::{FramerateParseError, TimecodeParseError};
pub use framerate::{rates, Framerate, FramerateParseResult, Ntsc};
pub use framerate_parse::{FramerateSource, FramerateSourceResult};
pub use source_frames::{FramesSource, FramesSourceResult};
pub use source_seconds::{SecondsSource, SecondsSourceResult};
pub use timecode::{Timecode, TimecodeParseResult, TimecodeSections};
