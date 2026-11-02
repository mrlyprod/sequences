use super::Axis;
use mrly_core::error::{Result, value_error};

/// The cost class of a measure along an axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cost {
    /// A closed form: microseconds a term.
    Closed,
    /// A convolution over the diagonal profile: milliseconds a term.
    Convolved,
    /// A rendered grid of `n^(D L)` cells.
    Grid,
}

/// One integer reading of a design.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Measure {
    /// The filled cells.
    Fills,
    /// The empty cells.
    Voids,
    /// The exposed faces: the perimeter in the plane, the surface in space.
    Surface,
    /// The most filled cells on one diagonal plane.
    Peak,
    /// The diagonal planes holding a filled cell.
    Heights,
    /// The distinct corners the filled cells touch.
    Vertices,
    /// The distinct unit edges the filled cells carry.
    Edges,
    /// The distinct unit faces the filled cells carry.
    Faces,
    /// The Euler characteristic of the filled complex.
    Euler,
    /// The filled triangles of the diagonal slice of a cube.
    Triangles,
    /// The holes of the diagonal slice of a cube.
    Holes,
    /// The connected pieces of the diagonal slice of a cube.
    Pieces,
    /// The empty triangles of the diagonal slice of a cube.
    Gaps,
    /// The closed loops of the plane drawn in Truchet arcs, a filled cell's arcs round its corners `(i, j)` and `(i+1, j+1)`, an empty cell's the other two.
    Loops,
    /// The distinct `k x k` windows of the plane's renders, read along the window side `k`.
    Windows,
}

impl Measure {
    /// Every measure, in ledger order.
    pub const ALL: [Measure; 15] = [
        Measure::Fills,
        Measure::Voids,
        Measure::Surface,
        Measure::Peak,
        Measure::Heights,
        Measure::Vertices,
        Measure::Edges,
        Measure::Faces,
        Measure::Euler,
        Measure::Triangles,
        Measure::Holes,
        Measure::Pieces,
        Measure::Gaps,
        Measure::Loops,
        Measure::Windows,
    ];
    /// Returns the measure's one-word name.
    pub fn slug(self) -> &'static str {
        match self {
            Measure::Fills => "fills",
            Measure::Voids => "voids",
            Measure::Surface => "surface",
            Measure::Peak => "peak",
            Measure::Heights => "heights",
            Measure::Vertices => "vertices",
            Measure::Edges => "edges",
            Measure::Faces => "faces",
            Measure::Euler => "euler",
            Measure::Triangles => "triangles",
            Measure::Holes => "holes",
            Measure::Pieces => "pieces",
            Measure::Gaps => "gaps",
            Measure::Loops => "loops",
            Measure::Windows => "windows",
        }
    }
    /// Parses a one-word name back into its measure, or an error for any other word.
    pub fn parse(slug: &str) -> Result<Measure> {
        Measure::ALL
            .into_iter()
            .find(|measure| measure.slug() == slug)
            .map_or_else(|| value_error(format!("unknown measure {slug:?}.")), Ok)
    }
    /// Returns the measure's cost class along the axis: the face counts close along the level and render along the side.
    pub fn cost(self, axis: Axis) -> Cost {
        match self {
            Measure::Fills | Measure::Voids | Measure::Surface => Cost::Closed,
            Measure::Peak | Measure::Heights => Cost::Convolved,
            Measure::Vertices | Measure::Edges | Measure::Faces | Measure::Euler
                if axis == Axis::Level =>
            {
                Cost::Closed
            }
            _ => Cost::Grid,
        }
    }
    /// Returns whether the measure reads a design of the dimension and base.
    pub fn applies(self, dimension: usize, base: usize) -> bool {
        match self {
            Measure::Fills | Measure::Voids | Measure::Surface => dimension >= 1,
            Measure::Peak | Measure::Heights => dimension >= 2,
            Measure::Vertices | Measure::Edges | Measure::Euler => dimension == 2 || dimension == 3,
            Measure::Faces => dimension == 3,
            Measure::Triangles | Measure::Holes | Measure::Pieces | Measure::Gaps => {
                dimension == 3 && base == 2
            }
            Measure::Loops | Measure::Windows => dimension == 2,
        }
    }
    /// Returns the axes the measure runs along: the window side for the windows, the level and the side for the rest.
    pub fn axes(self) -> &'static [Axis] {
        match self {
            Measure::Windows => &[Axis::Window],
            _ => &[Axis::Level, Axis::Side],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_slug_parses_back() {
        for measure in Measure::ALL {
            assert_eq!(Measure::parse(measure.slug()).unwrap(), measure);
        }
        assert!(Measure::parse("area").is_err());
    }
    #[test]
    fn the_plane_and_the_cube_read_their_measures() {
        let plane: Vec<Measure> = Measure::ALL
            .into_iter()
            .filter(|m| m.applies(2, 2))
            .collect();
        assert_eq!(plane.len(), 10);
        let cube: Vec<Measure> = Measure::ALL
            .into_iter()
            .filter(|m| m.applies(3, 2))
            .collect();
        assert_eq!(cube.len(), 13);
        assert_eq!(Measure::ALL.iter().filter(|m| m.applies(1, 5)).count(), 3);
        assert!(!Measure::Triangles.applies(3, 3));
        assert_eq!(Measure::Euler.cost(Axis::Level), Cost::Closed);
        assert_eq!(Measure::Euler.cost(Axis::Side), Cost::Grid);
        assert_eq!(Measure::Holes.cost(Axis::Level), Cost::Grid);
        assert_eq!(Measure::Windows.axes(), [Axis::Window]);
        assert!(Measure::Loops.axes().contains(&Axis::Level));
    }
}
