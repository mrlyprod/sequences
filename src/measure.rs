use super::Axis;
use mrly_core::error::{Result, value_error};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cost {
    Closed,
    Convolved,
    Grid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Measure {
    Fills,
    Voids,
    Surface,
    Peak,
    Heights,
    Vertices,
    Edges,
    Faces,
    Euler,
    Triangles,
    Holes,
    Pieces,
    Gaps,
    Loops,
    Windows,
}

impl Measure {
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
    pub fn parse(slug: &str) -> Result<Measure> {
        Measure::ALL
            .into_iter()
            .find(|measure| measure.slug() == slug)
            .map_or_else(|| value_error(format!("unknown measure {slug:?}.")), Ok)
    }
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
    pub fn axes(self) -> &'static [Axis] {
        match self {
            Measure::Windows => &[Axis::Window],
            _ => &[Axis::Level, Axis::Side],
        }
    }
}
