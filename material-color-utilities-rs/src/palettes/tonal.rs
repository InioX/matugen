use crate::htc::Hct;
use ahash::AHashMap;

// 0 to 100
pub type Tone = u8;

pub struct TonalPalette {
    cache: AHashMap<Tone, [u8; 4]>,
    hue: f64,
    chroma: f64,
}

impl TonalPalette {
    pub fn from_int(argb: [u8; 4]) -> TonalPalette {
        let hct = Hct::from_int(argb);
        Self::from_hue_and_chroma(hct.hue(), hct.chroma())
    }

    pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> TonalPalette {
        TonalPalette {
            cache: Default::default(),
            hue,
            chroma,
        }
    }

    pub fn tone(&mut self, tone: Tone) -> [u8; 4] {
        if let Some(cached) = self.cache.get(&tone) {
            *cached
        } else {
            let color = Hct::from(self.hue, self.chroma, tone as f64).to_int();
            self.cache.insert(tone, color);
            color
        }
    }
}
