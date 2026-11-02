//! The sequence rows: a design sequence's address, its measure and axis, its terms and its status.

/// The measures and their cost classes.
pub mod measure;

pub use measure::{Cost, Measure};

use mrly_core::error::{Result, value_error};
use mrly_core::name::Named;
use mrly_math::counts::Closed;
use mrly_math::name::{Bang, Sequence as SequenceName};

/// The index a sequence runs along.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    /// The fractal level from 1, at side `max(base, 3)`.
    Level,
    /// The odd side `2k - 1` from `k = 2`, at level 1.
    Side,
    /// The window side `k` from 1, over every level of the design at side `max(base, 3)`.
    Window,
}

impl Axis {
    /// Every axis, level first.
    pub const ALL: [Axis; 3] = [Axis::Level, Axis::Side, Axis::Window];
    /// Returns the axis's one-word name.
    pub fn slug(self) -> &'static str {
        match self {
            Axis::Level => "level",
            Axis::Side => "side",
            Axis::Window => "window",
        }
    }
    /// Parses a one-word name back into its axis, or an error for any other word.
    pub fn parse(slug: &str) -> Result<Axis> {
        match slug {
            "level" => Ok(Axis::Level),
            "side" => Ok(Axis::Side),
            "window" => Ok(Axis::Window),
            other => value_error(format!("unknown axis {other:?}.")),
        }
    }
    /// Returns the index of the first term: the level 1, the `k` of side 3 or the window side 1.
    pub fn start(self) -> i32 {
        match self {
            Axis::Level | Axis::Window => 1,
            Axis::Side => 2,
        }
    }
    /// Returns the side number and level of the term at the index; on the window axis the level slot carries the window side.
    pub fn place(self, index: usize, number: usize) -> (usize, u32) {
        match self {
            Axis::Level | Axis::Window => (number, index as u32 + 1),
            Axis::Side => (2 * index + 3, 1),
        }
    }
}

/// The status a claim carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tag {
    /// Proved on this tree.
    Proved,
    /// Checked against a record or a second generator.
    Verified,
    /// Stated and not yet checked.
    Conjecture,
    /// Checked and found false.
    Refuted,
}

impl Tag {
    /// Returns the tag's capitalised word.
    pub fn text(self) -> &'static str {
        match self {
            Tag::Proved => "Proved",
            Tag::Verified => "Verified",
            Tag::Conjecture => "Conjecture",
            Tag::Refuted => "Refuted",
        }
    }
}

/// A design sequence's address: the design, the measure and the axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key {
    /// The design's code.
    pub code: u128,
    /// The design's dimension.
    pub dimension: usize,
    /// The numeral base of the corners.
    pub base: usize,
    /// The reading taken.
    pub measure: Measure,
    /// The index the reading runs along.
    pub axis: Axis,
}

impl Key {
    /// Pins a design sequence to its address.
    pub const fn new(
        code: u128,
        dimension: usize,
        base: usize,
        measure: Measure,
        axis: Axis,
    ) -> Key {
        Key {
            code,
            dimension,
            base,
            measure,
            axis,
        }
    }
    /// Returns the sequence name this key addresses.
    pub fn named(&self) -> SequenceName {
        SequenceName::new(
            self.code,
            self.dimension,
            self.base,
            self.measure.slug(),
            self.axis.slug(),
        )
    }
    /// Returns the sequence's file name.
    ///
    /// ```
    /// use mrly_sequences::{Axis, Key, Measure};
    /// let key = Key::new(23, 3, 2, Measure::Surface, Axis::Level);
    /// assert_eq!(key.name(), "sequence_dim=3_code=23_measure=surface_axis=level");
    /// assert_eq!(key.id(), "753b6b49");
    /// ```
    pub fn name(&self) -> String {
        self.named().to_file().unwrap_or_default()
    }
    /// Returns the first eight hex digits of the sha256 of the sequence's canonical JSON.
    pub fn id(&self) -> String {
        self.named().to_id()
    }
    /// Returns the design pinned to its dimension and base.
    pub fn design(&self) -> Bang {
        Bang::new(self.code, self.dimension, self.base)
    }
    /// Returns the side number the level axis runs at.
    pub fn number(&self) -> usize {
        self.base.max(3)
    }
}

/// One row: a design sequence with its terms, its closed form and its status.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sequence {
    /// The sequence's address.
    pub key: Key,
    /// The first terms.
    pub terms: Vec<i128>,
    /// Whether the cell budget or a u128 stopped the terms short.
    pub capped: bool,
    /// The closed form, when one is known.
    pub closed: Option<Closed>,
    /// The status of the row, when one is known.
    pub tag: Option<Tag>,
}

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct Readme;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_axes_place_their_terms() {
        assert_eq!(Axis::Side.place(0, 3), (3, 1));
        assert_eq!(Axis::Level.place(2, 3), (3, 3));
        for axis in Axis::ALL {
            assert_eq!(Axis::parse(axis.slug()).unwrap(), axis);
        }
        assert!(Axis::parse("depth").is_err());
    }
}
