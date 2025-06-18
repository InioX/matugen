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
pub(crate) use set::set_blue;
pub(crate) use set::set_green;
pub(crate) use set::set_red;
