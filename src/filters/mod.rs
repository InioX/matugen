// pub mod alpha;
// pub mod camel;
// pub mod grayscale;
// pub mod hue;
// pub mod invert;
// pub mod lightness;

pub mod replace;

pub mod set;
pub(crate) use replace::replace;
pub(crate) use set::*;

pub mod colortransform;
pub(crate) use colortransform::*;
