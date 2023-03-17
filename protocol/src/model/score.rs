use serde::{Serialize, Deserialize};
use super::chat::Selector;

#[derive(Serialize, Deserialize)]
pub struct ScoreObjective {
    name: Selector
}