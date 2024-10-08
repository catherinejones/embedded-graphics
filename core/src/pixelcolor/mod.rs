//! Pixel color types.
//!
//! This module contains structs for commonly used pixel color formats and
//! conversions between them. The [`raw` module] provides additional functions
//! to convert colors to and from raw data for use with images and displays.
//!
//! # Implementing custom color types
//!
//! Custom color types can be added by implementing the [`PixelColor`] trait.
//! The following example shows how to implement a new color type for a
//! 3 color EPD display.
//!
//! ```
//! use embedded_graphics::{
//!     geometry::Size, prelude::*, primitives::{Rectangle, PrimitiveStyle},
//!     pixelcolor::raw::RawU2,
//! };
//!
//! /// Color with 3 states.
//! #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
//! pub enum EpdColor {
//!     White,
//!     Black,
//!     Red,
//! }
//!
//! /// Implement the `PixelColor` trait to mark this type as an embedded-graphics
//! /// color and associated binary storage format.
//! impl PixelColor for EpdColor {
//!     // 2 bits per pixel are required to store the 3 colors.
//!     type Raw = RawU2;
//! }
//!
//! /// Implement conversion from `RawU2` to `EpdColor` to make the type usable
//! /// for raw images.
//! impl From<RawU2> for EpdColor {
//!     fn from(data: RawU2) -> Self {
//!         match data.into_inner() {
//!             0 => Self::White,
//!             1 => Self::Black,
//!             2 => Self::Red,
//!             // Interpret the invalid encoding 0b11 as white:
//!             _ => Self::White,
//!         }
//!     }
//! }
//!
//! /// Implement conversion from `EpdColor` to `RawU2` to make the type usable
//! /// in framebuffers.
//! impl From<EpdColor> for RawU2 {
//!     fn from(color: EpdColor) -> RawU2 {
//!         RawU2::new(color as u8)
//!     }
//! }
//!
//! /// Mock EPD display.
//! pub struct EpdDisplay {}
//!
//! impl DrawTarget for EpdDisplay {
//!     type Color = EpdColor;
//!     type Error = core::convert::Infallible;
//!
//!     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
//!     where
//!         I: IntoIterator<Item = Pixel<Self::Color>>,
//!     {
//!         for Pixel(point, color) in pixels.into_iter() {
//!             match color {
//!                 EpdColor::White => {} // draw white pixel at `point`
//!                 EpdColor::Black => {} // draw black pixel at `point`
//!                 EpdColor::Red => {}   // draw red pixel at `point`
//!             }
//!         }
//!
//!         Ok(())
//!     }
//! }
//!
//! impl OriginDimensions for EpdDisplay {
//!     fn size(&self) -> Size {
//!         Size::new(300, 300)
//!     }
//! }
//!
//! let mut display = EpdDisplay {};
//!
//! Rectangle::new(Point::new(0, 0), Size::new(100, 100))
//!     .into_styled(PrimitiveStyle::with_fill(EpdColor::White))
//!     .draw(&mut display)?;
//!
//! Rectangle::new(Point::new(100, 0), Size::new(100, 100))
//!     .into_styled(PrimitiveStyle::with_fill(EpdColor::Black))
//!     .draw(&mut display)?;
//!
//! Rectangle::new(Point::new(200, 0), Size::new(100, 100))
//!     .into_styled(PrimitiveStyle::with_fill(EpdColor::Red))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! The implementation of the [`DrawTarget`] trait uses a `match` statement to
//! draw the specified color and doesn't depend on the raw data conversions,
//! see the [`raw` module] documentation for an example that uses this feature.
//!
//! [`DrawTarget`]: super::draw_target::DrawTarget
//! [`Raw`]: PixelColor::Raw
//! [`raw` module]: raw

mod binary_color;
mod conversion;
mod gray_color;
pub mod raw;
mod rgb_color;
mod web_colors;

pub use binary_color::*;
pub use gray_color::*;
use raw::RawData;
pub use rgb_color::*;
pub use web_colors::WebColors;

/// Pixel color trait.
///
/// This trait is used to represent types that can be used in drawing methods.
///
/// See the [module-level documentation] for more details.
///
/// [module-level documentation]: self
pub trait PixelColor: Copy + PartialEq + From<Self::Raw> + Into<Self::Raw> {
    /// Raw data type.
    ///
    /// Specifies the raw storage type that can be used to represent this color.
    ///
    /// See the [`raw` module documentation] for more details.
    ///
    /// [`raw` module documentation]: raw
    type Raw: RawData;
}

/// Convert a [`PixelColor`] into its underlying storage type
///
/// This trait provides the `into_storage` method for implementors of [`PixelColor`]. This method
/// exposes the underlying storage value of a pixel color type.
///
/// # Examples
///
/// ## Get the `u16` representing an `Rgb565` color
///
/// This example converts an [`Rgb565`] color into its underlying `u16` representation.
///
/// ```rust
/// use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
///
/// let color = Rgb565::new(0x1f, 0x00, 0x0a);
///
/// let raw = color.into_storage();
///
/// assert_eq!(raw, 0b11111_000000_01010u16);
/// ```
///
pub trait IntoStorage {
    /// The underlying storage type for the pixel color
    type Storage;

    /// Convert the `PixelColor` into its raw storage form
    fn into_storage(self) -> Self::Storage;
}

impl<C: PixelColor> IntoStorage for C {
    type Storage = <<C as PixelColor>::Raw as RawData>::Storage;

    fn into_storage(self) -> Self::Storage {
        self.into().into_inner()
    }
}
