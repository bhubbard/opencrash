//! Binary single-file `.pack` header and section directory for `OpenCrash`.
//!
//! An `OpenCrash` Pack bundles NHTSA FARS, California SWITRS / state DOT collision
//! records, H3/R-tree spatial hazard indices, intersection geometries, and
//! reporting police agency (CHP / PD / Sheriff) jurisdiction boundaries into a
//! single memory-mapped file.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Four-byte magic identifier at the start of every `OpenCrash` `.pack` file.
pub const PACK_MAGIC: [u8; 4] = *b"OCRS";

/// Current binary format version of the `OpenCrash` `.pack` specification.
pub const PACK_VERSION: u32 = 1;

/// Fixed byte length of the serialized header prefix (`magic` + `version` + `collision_count` + `intersection_count`).
pub const HEADER_PREFIX_LEN: usize = 24;

/// Errors returned when validating or decoding an `OpenCrash` `.pack` header.
#[derive(Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum PackError {
    /// The input buffer is shorter than [`HEADER_PREFIX_LEN`].
    #[error("pack buffer too short: expected at least {HEADER_PREFIX_LEN} bytes, got {actual}")]
    BufferTooShort {
        /// Actual byte count provided.
        actual: usize,
    },
    /// The leading four bytes do not match [`PACK_MAGIC`].
    #[error("invalid pack magic bytes: {actual:?}")]
    InvalidMagic {
        /// The four bytes found at offset 0.
        actual: [u8; 4],
    },
    /// The pack version is unsupported by this binary.
    #[error("unsupported pack format version {found} (expected {PACK_VERSION})")]
    UnsupportedVersion {
        /// Version number decoded from the header.
        found: u32,
    },
}

/// Byte offset and length of a contiguous section inside a `.pack` file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SectionSpan {
    /// Byte offset from the start of the `.pack` file (`64-byte` aligned).
    pub offset: u64,
    /// Byte length of the section payload.
    pub length: u64,
}

/// Metadata and section directory stored in the header of an `OpenCrash` `.pack` file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackHeader {
    /// Format version (`PACK_VERSION`).
    pub version: u32,
    /// Total number of historical collisions indexed in this pack.
    pub collision_count: u64,
    /// Total number of named road intersections and highway postmiles indexed.
    pub intersection_count: u64,
    /// Jurisdiction or region identifier (e.g., `"us-ca"`, `"us-all"`).
    pub region: String,
    /// Spatial R-tree and H3 hex-cell hazard index section.
    pub spatial_section: SectionSpan,
    /// Columnar bit-packed collision records (timestamp, severity, PCF violation, weather, lighting).
    pub collisions_section: SectionSpan,
    /// Law enforcement agency (CHP Area Office, LAPD Traffic Division, Sheriff) beat routing section.
    pub agencies_section: SectionSpan,
}

impl PackHeader {
    /// Creates a new header for the given region and counts.
    pub fn new(region: impl Into<String>, collision_count: u64, intersection_count: u64) -> Self {
        Self {
            version: PACK_VERSION,
            collision_count,
            intersection_count,
            region: region.into(),
            spatial_section: SectionSpan::default(),
            collisions_section: SectionSpan::default(),
            agencies_section: SectionSpan::default(),
        }
    }

    /// Encodes the 24-byte fixed prefix (`magic`, `version`, `collision_count`, `intersection_count`).
    pub fn encode_prefix(&self) -> [u8; HEADER_PREFIX_LEN] {
        let mut buf = [0u8; HEADER_PREFIX_LEN];
        buf[0..4].copy_from_slice(&PACK_MAGIC);
        buf[4..8].copy_from_slice(&self.version.to_le_bytes());
        buf[8..16].copy_from_slice(&self.collision_count.to_le_bytes());
        buf[16..24].copy_from_slice(&self.intersection_count.to_le_bytes());
        buf
    }

    /// Validates and decodes the 24-byte fixed prefix from a byte slice.
    pub fn decode_prefix(bytes: &[u8]) -> Result<(u32, u64, u64), PackError> {
        if bytes.len() < HEADER_PREFIX_LEN {
            return Err(PackError::BufferTooShort {
                actual: bytes.len(),
            });
        }
        let mut magic = [0u8; 4];
        magic.copy_from_slice(&bytes[0..4]);
        if magic != PACK_MAGIC {
            return Err(PackError::InvalidMagic { actual: magic });
        }
        let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        if version != PACK_VERSION {
            return Err(PackError::UnsupportedVersion { found: version });
        }
        let collision_count = u64::from_le_bytes([
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ]);
        let intersection_count = u64::from_le_bytes([
            bytes[16], bytes[17], bytes[18], bytes[19], bytes[20], bytes[21], bytes[22], bytes[23],
        ]);
        Ok((version, collision_count, intersection_count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_roundtrip_succeeds() {
        let header = PackHeader::new("us-ca", 7_500_000, 1_120_000);
        let encoded = header.encode_prefix();
        let (version, crashes, intersections) =
            PackHeader::decode_prefix(&encoded).expect("valid prefix should decode");
        assert_eq!(version, PACK_VERSION);
        assert_eq!(crashes, 7_500_000);
        assert_eq!(intersections, 1_120_000);
    }
}
