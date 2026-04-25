#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec, format};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnchorHash(pub [u8; 32]);

impl AnchorHash {
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn short(&self) -> String {
        self.0[..4].iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StateLabel {
    Stationary,
    Transitioning,
    Degraded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InvariantId {
    Coupling,
    ScaleSeparation,
    ThermodynamicMeaning,
    FailClosed,
    StationaryFloor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InvariantResult {
    pub id: InvariantId,
    pub passed: bool,
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StateSeal {
    pub anchor: AnchorHash,
    pub label: StateLabel,
    pub sequence: u64,
    pub hash: [u8; 32],
}

/// PROVISIONAL: Wire format for cross-domain replay prevention.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WireSeal {
    pub seal: StateSeal,
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub signature: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor_short() {
        let mut bytes = [0u8; 32];
        bytes[0] = 0xde; bytes[1] = 0xad; bytes[2] = 0xbe; bytes[3] = 0xef;
        let anchor = AnchorHash(bytes);
        assert_eq!(anchor.short(), "deadbeef");
    }
}
