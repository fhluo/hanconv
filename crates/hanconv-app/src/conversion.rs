use gpui::Action;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, VariantArray};

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    EnumCount,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
    JsonSchema,
    Action,
)]
#[serde(rename_all = "lowercase")]
pub enum Conversion {
    S2T,
    T2S,
    S2TW,
    TW2S,
    T2TW,
    TW2T,
    S2HK,
    HK2S,
    T2HK,
    HK2T,
    T2JP,
    JP2T,
}

impl Conversion {
    pub fn name(&self) -> &'static str {
        match self {
            Conversion::S2T => "s2t",
            Conversion::T2S => "t2s",
            Conversion::S2TW => "s2tw",
            Conversion::TW2S => "tw2s",
            Conversion::T2TW => "t2tw",
            Conversion::TW2T => "tw2t",
            Conversion::S2HK => "s2hk",
            Conversion::HK2S => "hk2s",
            Conversion::T2HK => "t2hk",
            Conversion::HK2T => "hk2t",
            Conversion::T2JP => "t2jp",
            Conversion::JP2T => "jp2t",
        }
    }

    pub fn title(&self) -> String {
        format!(
            "{} → {}",
            t!(format!("{}.source", self.name())),
            t!(format!("{}.target", self.name()))
        )
    }

    pub fn run(&self, content: impl AsRef<str>) -> String {
        match self {
            Conversion::S2T => hanconv::s2t(content),
            Conversion::T2S => hanconv::t2s(content),
            Conversion::S2TW => hanconv::s2tw(content),
            Conversion::TW2S => hanconv::tw2s(content),
            Conversion::T2TW => hanconv::t2tw(content),
            Conversion::TW2T => hanconv::tw2t(content),
            Conversion::S2HK => hanconv::s2hk(content),
            Conversion::HK2S => hanconv::hk2s(content),
            Conversion::T2HK => hanconv::t2hk(content),
            Conversion::HK2T => hanconv::hk2t(content),
            Conversion::T2JP => hanconv::t2jp(content),
            Conversion::JP2T => hanconv::jp2t(content),
        }
    }
}
