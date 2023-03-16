#[stdto::bytes(endian = "big")]
pub struct Disconnect {
    reason: Chat
}