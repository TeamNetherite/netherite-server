pub enum State {
    None,
    Handshaking,
    Status,
    Login
}

impl TryFrom<i32> for State {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use State::*;
        match value {
            -1 => Ok(None),
            0 => Ok(Handshaking),
            1 => Ok(Status),
            2 => Ok(Login),
            _ => Err(())
        }
    }
}