mod errors;
mod framerate;
mod framerate_parse;
#[cfg(test)]
mod framerate_test;

pub use errors::FramerateParseError;
pub use framerate::{Framerate, Ntsc};
pub use framerate_parse::{FramerateSource, FramerateSourceResult};
