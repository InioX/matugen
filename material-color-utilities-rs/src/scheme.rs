use crate::palettes::core::CorePalette;
#[cfg(feature = "serde")]
use crate::util::color::format_argb_as_rgb;
#[cfg(feature = "serde")]
use serde::{ser::SerializeStruct, Serialize};

/// Represents a Material color scheme, a mapping of color roles to colors.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Scheme {
    pub primary: [u8; 4],
    pub on_primary: [u8; 4],
    pub primary_container: [u8; 4],
    pub on_primary_container: [u8; 4],
    pub secondary: [u8; 4],
    pub on_secondary: [u8; 4],
    pub secondary_container: [u8; 4],
    pub on_secondary_container: [u8; 4],
    pub tertiary: [u8; 4],
    pub on_tertiary: [u8; 4],
    pub tertiary_container: [u8; 4],
    pub on_tertiary_container: [u8; 4],
    pub error: [u8; 4],
    pub on_error: [u8; 4],
    pub error_container: [u8; 4],
    pub on_error_container: [u8; 4],
    pub background: [u8; 4],
    pub on_background: [u8; 4],
    pub surface: [u8; 4],
    pub on_surface: [u8; 4],
    pub surface_variant: [u8; 4],
    pub on_surface_variant: [u8; 4],
    pub outline: [u8; 4],
    pub outline_variant: [u8; 4],
    pub shadow: [u8; 4],
    pub scrim: [u8; 4],
    pub inverse_surface: [u8; 4],
    pub inverse_on_surface: [u8; 4],
    pub inverse_primary: [u8; 4],
}

#[cfg(feature = "serde")]
impl Serialize for Scheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Scheme", 29)?;

        // Macro to serialize an ARGB field to its RGB representation, and reduce
        // the risk of a typo between a field name and it's name in the output.
        macro_rules! ser {
            ($key:ident) => {
                state.serialize_field(stringify!($key), &format_argb_as_rgb(self.$key))?;
            };
        }

        ser!(primary);
        ser!(on_primary);
        ser!(primary_container);
        ser!(on_primary_container);
        ser!(secondary);
        ser!(on_secondary);
        ser!(secondary_container);
        ser!(on_secondary_container);
        ser!(tertiary);
        ser!(on_tertiary);
        ser!(tertiary_container);
        ser!(on_tertiary_container);
        ser!(error);
        ser!(on_error);
        ser!(error_container);
        ser!(on_error_container);
        ser!(background);
        ser!(on_background);
        ser!(surface);
        ser!(on_surface);
        ser!(surface_variant);
        ser!(on_surface_variant);
        ser!(outline);
        ser!(outline_variant);
        ser!(shadow);
        ser!(scrim);
        ser!(inverse_surface);
        ser!(inverse_on_surface);
        ser!(inverse_primary);

        state.end()
    }
}

impl Scheme {
    pub fn light_from_core_palette(core: &mut CorePalette) -> Scheme {
        Scheme {
            primary: core.a1.tone(40),
            on_primary: core.a1.tone(100),
            primary_container: core.a1.tone(90),
            on_primary_container: core.a1.tone(10),
            secondary: core.a2.tone(40),
            on_secondary: core.a2.tone(100),
            secondary_container: core.a2.tone(90),
            on_secondary_container: core.a2.tone(10),
            tertiary: core.a3.tone(40),
            on_tertiary: core.a3.tone(100),
            tertiary_container: core.a3.tone(90),
            on_tertiary_container: core.a3.tone(10),
            error: core.error.tone(40),
            on_error: core.error.tone(100),
            error_container: core.error.tone(90),
            on_error_container: core.error.tone(10),
            background: core.n1.tone(99),
            on_background: core.n1.tone(10),
            surface: core.n1.tone(99),
            on_surface: core.n1.tone(10),
            surface_variant: core.n2.tone(90),
            on_surface_variant: core.n2.tone(30),
            outline: core.n2.tone(50),
            outline_variant: core.n2.tone(80),
            shadow: core.n1.tone(0),
            scrim: core.n1.tone(0),
            inverse_surface: core.n1.tone(20),
            inverse_on_surface: core.n1.tone(95),
            inverse_primary: core.a1.tone(80),
        }
    }

    pub fn dark_from_core_palette(core: &mut CorePalette) -> Scheme {
        Scheme {
            primary: core.a1.tone(80),
            on_primary: core.a1.tone(20),
            primary_container: core.a1.tone(30),
            on_primary_container: core.a1.tone(90),
            secondary: core.a2.tone(80),
            on_secondary: core.a2.tone(20),
            secondary_container: core.a2.tone(30),
            on_secondary_container: core.a2.tone(90),
            tertiary: core.a3.tone(80),
            on_tertiary: core.a3.tone(20),
            tertiary_container: core.a3.tone(30),
            on_tertiary_container: core.a3.tone(90),
            error: core.error.tone(80),
            on_error: core.error.tone(20),
            error_container: core.error.tone(30),
            on_error_container: core.error.tone(80),
            background: core.n1.tone(10),
            on_background: core.n1.tone(90),
            surface: core.n1.tone(10),
            on_surface: core.n1.tone(90),
            surface_variant: core.n2.tone(30),
            on_surface_variant: core.n2.tone(80),
            outline: core.n2.tone(60),
            outline_variant: core.n2.tone(30),
            shadow: core.n1.tone(0),
            scrim: core.n1.tone(0),
            inverse_surface: core.n1.tone(90),
            inverse_on_surface: core.n1.tone(20),
            inverse_primary: core.a1.tone(40),
        }
    }
}
