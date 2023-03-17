use std::ops::Range;
use crate::{
    model::{
        score::ScoreObjective,
        pos::Vec3f
    },
    server::Server,
};
use netherite_common::macros::EnumFields;
use serde::{Deserialize, Serialize};
use crate::model::gamemode::Gamemode;
use crate::model::Invertable;

#[derive(Serialize, Deserialize)]
pub enum Selector {
    Nearest(Option<SelectorArguments>),
    Random(Option<SelectorArguments>),
    Everyone(Option<SelectorArguments>),
    Everything(Option<SelectorArguments>),
    EntitySelf,
}

#[derive(Serialize, Deserialize)]
pub struct SelectorArguments {
    #[serde(flatten)]
    pos: Option<Vec3f>,
    distance: Option<Range<f32>>,
    #[serde(flatten)]
    #[serde(with = "crate::model::pos::vec_diff3f")]
    diff: Option<Vec3f>,

    tag: Option<String>,
    team: Option<String>,

    limit: Option<i32>,
    sort: Option<SelectorSort>,
    level: Option<Range<i32>>,

    gamemode: Option<Invertable<Gamemode>>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SelectorSort {
    Nearest,
    Furthest,
    Random,
    Arbitrary
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Chat {
    StringComponent {
        text: String,
        #[serde(flatten)]
        chat: DefaultChatComponent,
    },
    TranslationComponent {
        translate: String,
        #[serde(flatten)]
        chat: DefaultChatComponent,
    },
    KeybindComponent {
        keybind: String,
        #[serde(flatten)]
        chat: DefaultChatComponent,
    },
    ScoreComponent {
        score: ScoreObjective,
        #[serde(flatten)]
        chat: DefaultChatComponent,
    },
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DefaultChatComponent {
    /// The style of this component.
    #[serde(flatten)]
    pub style: ChatStyle,
    /// The text to insert into the chat typing box, when clicking the text with Shift
    pub insertion: Option<String>,
    /// The siblings of this component
    pub extra: Vec<DefaultChatComponent>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatStyle {
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub strikethrough: bool,
    pub obfuscated: bool,
    pub font: ChatFont,
    pub color: Option<ChatColor>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatFont {
    #[serde(rename = "minecraft:uniform")]
    Unicode,
    #[serde(rename = "minecraft:alt")]
    Alien,
    #[serde(rename = "minecraft:default")]
    #[default]
    Default,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatColor {
    Normal(ColorCode),
    WebColor(i32),
}

#[derive(Clone, EnumFields, Serialize, Deserialize)]
#[enum_field(delegate code: char, hex: i32)]
#[serde(try_from = "IntoColorCode", into = "IntoColorCode")]
pub enum ColorCode {
    #[ef(code = '0', hex = 0x000000)]
    Black,
    #[ef(code = '1', hex = 0x0000aa)]
    DarkBlue,
    #[ef(code = '2', hex = 0x00aa00)]
    DarkGreen,
    #[ef(code = '3', hex = 0x00aaaa)]
    DarkCyan,
    #[ef(code = '4', hex = 0xaa0000)]
    DarkRed,
    #[ef(code = '5', hex = 0xaa00aa)]
    DarkPurple,
    #[ef(code = '6', hex = 0xffaa00)]
    Gold,
    #[ef(code = '7', hex = 0xaaaaaa)]
    Gray,
    #[ef(code = '8', hex = 0x555555)]
    DarkGray,
    #[ef(code = '9', hex = 0x5555ff)]
    Blue,
    #[ef(code = 'a', hex = 0x55ff55)]
    Green,
    #[ef(code = 'b', hex = 0x55ffff)]
    Aqua,
    #[ef(code = 'c', hex = 0xff5555)]
    Red,
    #[ef(code = 'd', hex = 0xff55ff)]
    LightPurple,
    #[ef(code = 'e', hex = 0xffff55)]
    Yellow,
    #[ef(code = 'f', hex = 0xffffff)]
    White,
}
