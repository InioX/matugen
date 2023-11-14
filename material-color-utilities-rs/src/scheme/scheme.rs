use crate::palettes::core::CorePalette;
#[cfg(feature = "serde")]
use crate::util::color::format_argb_as_rgb;
#[cfg(feature = "serde")]
use serde::{ser::SerializeStruct, Serialize};

/// Represents a Material color scheme, a mapping of color roles to colors.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Scheme {
    pub primary: [u8; 4],
    pub primary_fixed: [u8; 4],
    pub primary_fixed_dim: [u8; 4],
    pub on_primary: [u8; 4],
    pub on_primary_fixed: [u8; 4],
    pub on_primary_fixed_variant: [u8; 4],
    pub primary_container: [u8; 4],
    pub on_primary_container: [u8; 4],
    pub secondary: [u8; 4],
    pub secondary_fixed: [u8; 4],
    pub secondary_fixed_dim: [u8; 4],
    pub on_secondary: [u8; 4],
    pub on_secondary_fixed: [u8; 4],
    pub on_secondary_fixed_variant: [u8; 4],
    pub secondary_container: [u8; 4],
    pub on_secondary_container: [u8; 4],
    pub tertiary: [u8; 4],
    pub tertiary_fixed: [u8; 4],
    pub tertiary_fixed_dim: [u8; 4],
    pub on_tertiary: [u8; 4],
    pub on_tertiary_fixed: [u8; 4],
    pub on_tertiary_fixed_variant: [u8; 4],
    pub tertiary_container: [u8; 4],
    pub on_tertiary_container: [u8; 4],
    pub error: [u8; 4],
    pub on_error: [u8; 4],
    pub error_container: [u8; 4],
    pub on_error_container: [u8; 4],
    pub surface: [u8; 4],
    pub on_surface: [u8; 4],
    pub on_surface_variant: [u8; 4],
    pub outline: [u8; 4],
    pub outline_variant: [u8; 4],
    pub shadow: [u8; 4],
    pub scrim: [u8; 4],
    pub inverse_surface: [u8; 4],
    pub inverse_on_surface: [u8; 4],
    pub inverse_primary: [u8; 4],
    pub surface_dim: [u8; 4],
    pub surface_bright: [u8; 4],
    pub surface_container_lowest: [u8; 4],
    pub surface_container_low: [u8; 4],
    pub surface_container: [u8; 4],
    pub surface_container_high: [u8; 4],
    pub surface_container_highest: [u8; 4],
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
        ser!(primary_fixed);
        ser!(primary_fixed_dim);
        ser!(on_primary);
        ser!(on_primary_fixed);
        ser!(on_primary_fixed_variant);
        ser!(primary_container);
        ser!(on_primary_container);
        ser!(secondary);
        ser!(secondary_fixed);
        ser!(secondary_fixed_dim);
        ser!(on_secondary);
        ser!(on_secondary_fixed);
        ser!(on_secondary_fixed_variant);
        ser!(secondary_container);
        ser!(on_secondary_container);
        ser!(tertiary);
        ser!(tertiary_fixed);
        ser!(tertiary_fixed_dim);
        ser!(on_tertiary);
        ser!(on_tertiary_fixed);
        ser!(on_tertiary_fixed_variant);
        ser!(tertiary_container);
        ser!(on_tertiary_container);
        ser!(error);
        ser!(on_error);
        ser!(error_container);
        ser!(on_error_container);
        ser!(surface);
        ser!(on_surface);
        ser!(on_surface_variant);
        ser!(outline);
        ser!(outline_variant);
        ser!(shadow);
        ser!(scrim);
        ser!(inverse_surface);
        ser!(inverse_on_surface);
        ser!(inverse_primary);
        ser!(surface_dim);
        ser!(surface_bright);
        ser!(surface_container_lowest);
        ser!(surface_container_low);
        ser!(surface_container);
        ser!(surface_container_high);
        ser!(surface_container_highest);

        state.end()
    }
}

impl Scheme {
    pub fn light_from_core_palette(core: &mut CorePalette) -> Scheme {
        Scheme {
            primary: core.a1.tone(40),
            primary_fixed: core.a1.tone(90),
            primary_fixed_dim: core.a1.tone(80),
            on_primary: core.a1.tone(100),
            on_primary_fixed: core.a1.tone(10),
            on_primary_fixed_variant: core.a1.tone(30),
            primary_container: core.a1.tone(90),
            on_primary_container: core.a1.tone(10),
            secondary: core.a2.tone(40),
            secondary_fixed: core.a2.tone(90),
            secondary_fixed_dim: core.a2.tone(80),
            on_secondary: core.a2.tone(100),
            on_secondary_fixed: core.a2.tone(10),
            on_secondary_fixed_variant: core.a2.tone(30),
            secondary_container: core.a2.tone(90),
            on_secondary_container: core.a2.tone(10),
            tertiary: core.a3.tone(40),
            tertiary_fixed: core.a3.tone(90),
            tertiary_fixed_dim: core.a3.tone(80),
            on_tertiary: core.a3.tone(100),
            on_tertiary_fixed: core.a3.tone(10),
            on_tertiary_fixed_variant: core.a3.tone(30),
            tertiary_container: core.a3.tone(90),
            on_tertiary_container: core.a3.tone(10),
            error: core.error.tone(40),
            on_error: core.error.tone(100),
            error_container: core.error.tone(90),
            on_error_container: core.error.tone(10),
            surface: core.n1.tone(98),
            on_surface: core.n1.tone(10),
            on_surface_variant: core.n2.tone(30),
            outline: core.n2.tone(50),
            outline_variant: core.n2.tone(80),
            shadow: core.n1.tone(0),
            scrim: core.n1.tone(0),
            inverse_surface: core.n1.tone(20),
            inverse_on_surface: core.n1.tone(95),
            inverse_primary: core.a1.tone(80),
            surface_dim: core.n1.tone(87),
            surface_bright: core.n1.tone(98),
            surface_container_lowest: core.n1.tone(100),
            surface_container_low: core.n1.tone(96),
            surface_container: core.n1.tone(94),
            surface_container_high: core.n1.tone(92),
            surface_container_highest: core.n1.tone(90),
        }
    }

    pub fn dark_from_core_palette(core: &mut CorePalette) -> Scheme {
        Scheme {
            primary: core.a1.tone(80),
            primary_fixed: core.a1.tone(90),
            primary_fixed_dim: core.a1.tone(80),
            on_primary: core.a1.tone(20),
            on_primary_fixed: core.a1.tone(10),
            on_primary_fixed_variant: core.a1.tone(30),
            primary_container: core.a1.tone(30),
            on_primary_container: core.a1.tone(90),
            secondary: core.a2.tone(80),
            secondary_fixed: core.a2.tone(90),
            secondary_fixed_dim: core.a2.tone(80),
            on_secondary: core.a2.tone(20),
            on_secondary_fixed: core.a2.tone(10),
            on_secondary_fixed_variant: core.a2.tone(30),
            secondary_container: core.a2.tone(30),
            on_secondary_container: core.a2.tone(90),
            tertiary: core.a3.tone(80),
            tertiary_fixed: core.a3.tone(90),
            tertiary_fixed_dim: core.a3.tone(80),
            on_tertiary: core.a3.tone(20),
            on_tertiary_fixed: core.a3.tone(10),
            on_tertiary_fixed_variant: core.a3.tone(30),
            tertiary_container: core.a3.tone(30),
            on_tertiary_container: core.a3.tone(90),
            error: core.error.tone(80),
            on_error: core.error.tone(20),
            error_container: core.error.tone(30),
            on_error_container: core.error.tone(80),
            surface: core.n1.tone(6),
            on_surface: core.n1.tone(90),
            on_surface_variant: core.n2.tone(80),
            outline: core.n2.tone(60),
            outline_variant: core.n2.tone(30),
            shadow: core.n1.tone(0),
            scrim: core.n1.tone(0),
            inverse_surface: core.n1.tone(90),
            inverse_on_surface: core.n1.tone(20),
            inverse_primary: core.a1.tone(40),
            surface_dim: core.n1.tone(6),
            surface_bright: core.n1.tone(24),
            surface_container_lowest: core.n1.tone(4),
            surface_container_low: core.n1.tone(10),
            surface_container: core.n1.tone(12),
            surface_container_high: core.n1.tone(17),
            surface_container_highest: core.n1.tone(22),
        }
    }

    pub fn pure_dark_from_core_palette(core: &mut CorePalette) -> Scheme {
        Scheme {
            primary: core.a1.tone(80),
            primary_fixed: core.a1.tone(90),
            primary_fixed_dim: core.a1.tone(80),
            on_primary: core.a1.tone(20),
            on_primary_fixed: core.a1.tone(10),
            on_primary_fixed_variant: core.a1.tone(30),
            primary_container: core.a1.tone(30),
            on_primary_container: core.a1.tone(90),
            secondary: core.a2.tone(80),
            secondary_fixed: core.a2.tone(90),
            secondary_fixed_dim: core.a2.tone(80),
            on_secondary: core.a2.tone(20),
            on_secondary_fixed: core.a2.tone(10),
            on_secondary_fixed_variant: core.a2.tone(30),
            secondary_container: core.a2.tone(30),
            on_secondary_container: core.a2.tone(90),
            tertiary: core.a3.tone(80),
            tertiary_fixed: core.a3.tone(90),
            tertiary_fixed_dim: core.a3.tone(80),
            on_tertiary: core.a3.tone(20),
            on_tertiary_fixed: core.a3.tone(10),
            on_tertiary_fixed_variant: core.a3.tone(30),
            tertiary_container: core.a3.tone(30),
            on_tertiary_container: core.a3.tone(90),
            error: core.error.tone(80),
            on_error: core.error.tone(20),
            error_container: core.error.tone(30),
            on_error_container: core.error.tone(80),
            surface: core.n1.tone(0),
            on_surface: core.n1.tone(90),
            on_surface_variant: core.n2.tone(80),
            outline: core.n2.tone(60),
            outline_variant: core.n2.tone(30),
            shadow: core.n1.tone(0),
            scrim: core.n1.tone(0),
            inverse_surface: core.n1.tone(90),
            inverse_on_surface: core.n1.tone(20),
            inverse_primary: core.a1.tone(40),
            surface_dim: core.n1.tone(87),
            surface_bright: core.n1.tone(98),
            surface_container_lowest: core.n1.tone(100),
            surface_container_low: core.n1.tone(96),
            surface_container: core.n1.tone(94),
            surface_container_high: core.n1.tone(92),
            surface_container_highest: core.n1.tone(90),
        }
    }
}
