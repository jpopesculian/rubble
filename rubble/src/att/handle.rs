//! Attribute handles.

use crate::att::{AttError, ErrorCode};
use crate::{bytes::*, Error};
use core::{fmt, ops::RangeInclusive};

/// A 16-bit handle uniquely identifying an attribute on an ATT server.
///
/// The `0x0000` handle (`NULL`) is invalid and must not be used.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Handle(u16);

impl Handle {
    /// The `0x0000` handle is not used for actual attributes, but as a special placeholder when no
    /// attribute handle is valid (eg. in error responses).
    pub const NULL: Self = Handle(0x0000);

    /// Returns the raw 16-bit integer representing this handle.
    pub fn as_u16(&self) -> u16 {
        self.0
    }

    /// Create an attribute handle from a raw u16
    pub const fn from_raw(raw: u16) -> Self {
        Handle(raw)
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06X}", self.0)
    }
}

impl defmt::Format for Handle {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{:#06X}", self.0)
    }
}

impl FromBytes<'_> for Handle {
    fn from_bytes(bytes: &mut ByteReader<'_>) -> Result<Self, Error> {
        Ok(Handle(bytes.read_u16_le()?))
    }
}

impl ToBytes for Handle {
    fn to_bytes(&self, writer: &mut ByteWriter<'_>) -> Result<(), Error> {
        writer.write_u16_le(self.as_u16())?;
        Ok(())
    }
}

/// A (de)serializable handle range that isn't checked for validity.
#[derive(Debug, Copy, Clone)]
pub struct RawHandleRange {
    start: Handle,
    end: Handle,
}

impl RawHandleRange {
    /// Checks that this handle range is valid according to the Bluetooth spec.
    ///
    /// Returns an `AttError` that should be sent as a response if the range is invalid.
    pub fn check(&self) -> Result<HandleRange, AttError> {
        if self.start.0 > self.end.0 || self.start.0 == 0 {
            Err(AttError::new(ErrorCode::InvalidHandle, self.start))
        } else {
            Ok(HandleRange(self.start..=self.end))
        }
    }
}

impl FromBytes<'_> for RawHandleRange {
    fn from_bytes(bytes: &mut ByteReader<'_>) -> Result<Self, Error> {
        Ok(Self {
            start: Handle::from_bytes(bytes)?,
            end: Handle::from_bytes(bytes)?,
        })
    }
}

impl ToBytes for RawHandleRange {
    fn to_bytes(&self, writer: &mut ByteWriter<'_>) -> Result<(), Error> {
        writer.write_u16_le(self.start.as_u16())?;
        writer.write_u16_le(self.end.as_u16())?;
        Ok(())
    }
}

/// A (de)serializable handle range that has been checked for validity.
#[derive(Debug)]
pub struct HandleRange(RangeInclusive<Handle>);

impl HandleRange {
    pub fn new(from: Handle, to: Handle) -> Self {
        HandleRange(from..=to)
    }

    /// Checks if an Handle is in a HandleRange
    pub fn contains(&self, handle: Handle) -> bool {
        self.0.start().0 <= handle.as_u16() && self.0.end().0 >= handle.as_u16()
    }

    /// Returns the lowest attribute handle value included in the range.
    pub fn start(&self) -> Handle {
        *self.0.start()
    }

    /// Returns the last (highest) attribute handle value included in the range.
    pub fn end(&self) -> Handle {
        *self.0.end()
    }
}
