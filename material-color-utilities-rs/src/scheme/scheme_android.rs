use crate::palettes::core::CorePalette;
#[cfg(feature = "serde")]
use crate::util::color::format_argb_as_rgb;
#[cfg(feature = "serde")]
use serde::{ser::SerializeStruct, Serialize};

/// Represents a Material color scheme, a mapping of color roles to colors.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct SchemeAndroid {
    pub color_accent_primary: [u8; 4],
    pub color_accent_primary_variant: [u8; 4],
    pub color_accent_secondary: [u8; 4],
    pub color_accent_secondary_variant: [u8; 4],
    pub color_accent_tertiary: [u8; 4],
    pub color_accent_tertiary_variant: [u8; 4],
    pub text_color_primary: [u8; 4],
    pub text_color_secondary: [u8; 4],
    pub text_color_tertiary: [u8; 4],
    pub text_color_primary_inverse: [u8; 4],
    pub text_color_secondary_inverse: [u8; 4],
    pub text_color_tertiary_inverse: [u8; 4],
    pub color_background: [u8; 4],
    pub color_background_floating: [u8; 4],
    pub color_surface: [u8; 4],
    pub color_surface_variant: [u8; 4],
    pub color_surface_highlight: [u8; 4],
    pub surface_header: [u8; 4],
    pub under_surface: [u8; 4],
    pub off_state: [u8; 4],
    pub accent_surface: [u8; 4],
    pub text_primary_on_accent: [u8; 4],
    pub text_secondary_on_accent: [u8; 4],
    pub volume_background: [u8; 4],
    pub scrim: [u8; 4],
}

#[cfg(feature = "serde")]
impl Serialize for SchemeAndroid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("SchemeAndroid", 25)?;

        // Macro to serialize an ARGB field to its RGB representation, and reduce
        // the risk of a typo between a field name and it's name in the output.
        macro_rules! ser {
            ($key:ident) => {
                state.serialize_field(stringify!($key), &format_argb_as_rgb(self.$key))?;
            };
        }

        ser!(color_accent_primary);
        ser!(color_accent_primary_variant);
        ser!(color_accent_secondary);
        ser!(color_accent_secondary_variant);
        ser!(color_accent_tertiary);
        ser!(color_accent_tertiary_variant);
        ser!(text_color_primary);
        ser!(text_color_secondary);
        ser!(text_color_tertiary);
        ser!(text_color_primary_inverse);
        ser!(text_color_secondary_inverse);
        ser!(text_color_tertiary_inverse);
        ser!(color_background);
        ser!(color_background_floating);
        ser!(color_surface);
        ser!(color_surface_variant);
        ser!(color_surface_highlight);
        ser!(surface_header);
        ser!(under_surface);
        ser!(off_state);
        ser!(accent_surface);
        ser!(text_primary_on_accent);
        ser!(text_secondary_on_accent);
        ser!(volume_background);
        ser!(scrim);

        state.end()
    }
}

impl SchemeAndroid {
    pub fn light_from_core_palette(core: &mut CorePalette) -> SchemeAndroid {
        SchemeAndroid {
            color_accent_primary: core.a1.tone(90),
            color_accent_primary_variant: core.a1.tone(40),
            color_accent_secondary: core.a2.tone(90),
            color_accent_secondary_variant: core.a2.tone(40),
            color_accent_tertiary: core.a3.tone(90),
            color_accent_tertiary_variant: core.a3.tone(40),
            text_color_primary: core.n1.tone(10),
            text_color_secondary: core.n2.tone(30),
            text_color_tertiary: core.n2.tone(50),
            text_color_primary_inverse: core.n1.tone(95),
            text_color_secondary_inverse: core.n1.tone(80),
            text_color_tertiary_inverse: core.n1.tone(60),
            color_background: core.n1.tone(95),
            color_background_floating: core.n1.tone(98),
            color_surface: core.n1.tone(98),
            color_surface_variant: core.n1.tone(90),
            color_surface_highlight: core.n1.tone(100),
            surface_header: core.n1.tone(90),
            under_surface: core.n1.tone(0),
            off_state: core.n1.tone(20),
            accent_surface: core.a2.tone(95),
            text_primary_on_accent: core.n1.tone(10),
            text_secondary_on_accent: core.n2.tone(30),
            volume_background: core.n1.tone(25),
            scrim: core.n1.tone(80),
        }
    }

    pub fn dark_from_core_palette(core: &mut CorePalette) -> SchemeAndroid {
        SchemeAndroid {
            color_accent_primary: core.a1.tone(90),
            color_accent_primary_variant: core.a1.tone(70),
            color_accent_secondary: core.a2.tone(90),
            color_accent_secondary_variant: core.a2.tone(70),
            color_accent_tertiary: core.a3.tone(90),
            color_accent_tertiary_variant: core.a3.tone(70),
            text_color_primary: core.n1.tone(95),
            text_color_secondary: core.n2.tone(80),
            text_color_tertiary: core.n2.tone(60),
            text_color_primary_inverse: core.n1.tone(10),
            text_color_secondary_inverse: core.n1.tone(30),
            text_color_tertiary_inverse: core.n1.tone(50),
            color_background: core.n1.tone(10),
            color_background_floating: core.n1.tone(10),
            color_surface: core.n1.tone(20),
            color_surface_variant: core.n1.tone(30),
            color_surface_highlight: core.n1.tone(35),
            surface_header: core.n1.tone(30),
            under_surface: core.n1.tone(0),
            off_state: core.n1.tone(20),
            accent_surface: core.a2.tone(95),
            text_primary_on_accent: core.n1.tone(10),
            text_secondary_on_accent: core.n2.tone(30),
            volume_background: core.n1.tone(25),
            scrim: core.n1.tone(80),
        }
    }

    // TODO: Make this look like amoled mode
    pub fn pure_dark_from_core_palette(core: &mut CorePalette) -> SchemeAndroid {
        SchemeAndroid {
            color_accent_primary: core.a1.tone(90),
            color_accent_primary_variant: core.a1.tone(70),
            color_accent_secondary: core.a2.tone(90),
            color_accent_secondary_variant: core.a2.tone(70),
            color_accent_tertiary: core.a3.tone(90),
            color_accent_tertiary_variant: core.a3.tone(70),
            text_color_primary: core.n1.tone(95),
            text_color_secondary: core.n2.tone(80),
            text_color_tertiary: core.n2.tone(60),
            text_color_primary_inverse: core.n1.tone(10),
            text_color_secondary_inverse: core.n1.tone(30),
            text_color_tertiary_inverse: core.n1.tone(50),
            color_background: core.n1.tone(0),
            color_background_floating: core.n1.tone(0),
            color_surface: core.n1.tone(5),
            color_surface_variant: core.n1.tone(15),
            color_surface_highlight: core.n1.tone(10),
            surface_header: core.n1.tone(10),
            under_surface: core.n1.tone(0),
            off_state: core.n1.tone(20),
            accent_surface: core.a2.tone(95),
            text_primary_on_accent: core.n1.tone(10),
            text_secondary_on_accent: core.n2.tone(30),
            volume_background: core.n1.tone(0),
            scrim: core.n1.tone(80),
        }
    }
}
