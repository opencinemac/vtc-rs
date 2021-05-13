mod errors;
mod framerate;
mod framerate_parse;
#[cfg(test)]
mod framerate_test;

pub use errors::FramerateParseError;
pub use framerate::{rates, Framerate, FramerateParseResult, Ntsc};
pub use framerate_parse::{FramerateSource, FramerateSourceResult};
