use crate::packet::{Packet, S2CPacket};
use crate::server::Server;
use serde::{Deserialize, Serialize};
use netherite_common::macros::EnumFields;

#[stdto::bytes(endian = "big")]
#[stdto::yaml]
#[stdto::toml]
#[stdto::json]
pub struct Chat {}

#[derive(Serialize, Deserialize)]
pub struct ChatStyle {
    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    obfuscated: bool,
    font: ChatFont,
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

pub enum ChatColor {
    Normal(ColorCode),
    WebColor(i32)
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