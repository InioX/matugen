// pub mod alpha;
// pub mod camel;
// pub mod grayscale;
// pub mod hue;
// pub mod invert;
// pub mod lightness;

pub mod darken;
pub mod lighten;
pub mod replace;
pub(crate) use darken::darken;
pub(crate) use lighten::lighten;

pub mod set;
pub(crate) use replace::replace;
pub(crate) use set::*;

pub mod other_color;
pub(crate) use other_color::*;
