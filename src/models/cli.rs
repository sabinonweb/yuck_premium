use std::{
    fmt::{self, Display},
    str::FromStr,
};

use clap::Subcommand;

// The number of bits proccessed over a certain period of time
#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Bitrate {
    Worst,
    Worse = 32,
    Poor = 96,
    Low = 128,
    Medium = 192,
    Good = 256,
    High = 320,
    Best,
}

impl FromStr for Bitrate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "worst" => Ok(Bitrate::Worst),
            "32" => Ok(Bitrate::Worse),
            "96" => Ok(Bitrate::Poor),
            "128" => Ok(Bitrate::Low),
            "192" => Ok(Bitrate::Medium),
            "256" => Ok(Bitrate::Good),
            "320" => Ok(Bitrate::High),
            "best" => Ok(Bitrate::Best),
            _ => Err(()),
        }
    }
}

impl Display for Bitrate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bitrate::Worst => write!(f, "worst"),
            Bitrate::Worse => write!(f, "32"),
            Bitrate::Poor => write!(f, "96"),
            Bitrate::Low => write!(f, "128"),
            Bitrate::Medium => write!(f, "192"),
            Bitrate::Good => write!(f, "256"),
            Bitrate::High => write!(f, "320"),
            Bitrate::Best => write!(f, "best"),
        }
    }
}

// Codec determines the compression rate and file siz.
// Lossy Codec formats compress the file and reduce size
// But formats that don't compress have high audio quality
#[derive(Debug, Clone, Copy)]
pub enum Codec {
    MP3,
    Flac,
    MPA,
    Opus,
}

impl FromStr for Codec {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mp3" => Ok(Codec::MP3),
            "flac" => Ok(Codec::Flac),
            "mpa" => Ok(Codec::MPA),
            "opus" => Ok(Codec::Opus),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Codec::MP3 => write!(f, "mp3"),
            Codec::Flac => write!(f, "flac"),
            Codec::MPA => write!(f, "mpa"),
            Codec::Opus => write!(f, "opus"),
        }
    }
}
