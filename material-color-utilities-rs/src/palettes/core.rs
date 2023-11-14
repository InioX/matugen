use crate::htc::Hct;
use crate::palettes::tonal::TonalPalette;

/// An intermediate concept between the key color for a UI theme, and a full color scheme. 5 sets of
/// tones are generated, all except one use the same hue as the key color, and all vary in chroma.
pub struct CorePalette {
    pub a1: TonalPalette,
    pub a2: TonalPalette,
    pub a3: TonalPalette,
    pub n1: TonalPalette,
    pub n2: TonalPalette,
    pub error: TonalPalette,
}

#[cfg_attr(feature = "clap", derive(Copy, Clone, Debug, clap::ValueEnum))]
pub enum ColorPalette {
    Default,
    Triadic,
    Adjacent,
}

impl CorePalette {
    pub fn new(argb: [u8; 4], is_content: bool, color_palette: &ColorPalette) -> CorePalette {
        let hct = Hct::from_int(argb);
        let hue = hct.hue();
        let chroma = hct.chroma();
        let error = TonalPalette::from_hue_and_chroma(25.0, 84.0);

        let angle: f64;

        if is_content {
            match &color_palette {
                ColorPalette::Default => CorePalette {
                    a1: TonalPalette::from_hue_and_chroma(hue, chroma),
                    a2: TonalPalette::from_hue_and_chroma(hue, chroma / 3.),
                    a3: TonalPalette::from_hue_and_chroma(hue + 60., chroma / 2.),
                    n1: TonalPalette::from_hue_and_chroma(hue, (chroma / 12.).min(6.0)),
                    n2: TonalPalette::from_hue_and_chroma(hue, (chroma / 6.).min(8.0)),
                    error,
                },
                ColorPalette::Triadic => {
                    angle = 90.;
                    CorePalette {
                        a1: TonalPalette::from_hue_and_chroma(hue, chroma),
                        a2: TonalPalette::from_hue_and_chroma(hue + angle, chroma / 3.),
                        a3: TonalPalette::from_hue_and_chroma(hue - angle, chroma / 2.),
                        n1: TonalPalette::from_hue_and_chroma(hue, (chroma / 12.).min(6.0)),
                        n2: TonalPalette::from_hue_and_chroma(hue, (chroma / 6.).min(8.0)),
                        error,
                    }
                }
                ColorPalette::Adjacent => {
                    angle = 30.;
                    CorePalette {
                        a1: TonalPalette::from_hue_and_chroma(hue, chroma),
                        a2: TonalPalette::from_hue_and_chroma(hue + angle, chroma / 3.),
                        a3: TonalPalette::from_hue_and_chroma(hue - angle, chroma / 2.),
                        n1: TonalPalette::from_hue_and_chroma(hue, (chroma / 12.).min(6.0)),
                        n2: TonalPalette::from_hue_and_chroma(hue, (chroma / 6.).min(8.0)),
                        error,
                    }
                }
            }
        } else {
            CorePalette {
                a1: TonalPalette::from_hue_and_chroma(hue, 48.0f64.max(chroma)),
                a2: TonalPalette::from_hue_and_chroma(hue, 16.),
                a3: TonalPalette::from_hue_and_chroma(hue + 60., 24.),
                n1: TonalPalette::from_hue_and_chroma(hue, 6.),
                n2: TonalPalette::from_hue_and_chroma(hue, 8.),
                error,
            }
        }
    }
}
