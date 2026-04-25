use serde::{Deserialize, Serialize};

pub const GENESIS_ANCHOR: &str = "01a777";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub source: String,
    pub target: String,
    pub weight: f64, // The "Resonance" score
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationalMap {
    pub nodes: Vec<Node>,
    pub relations: Vec<Relation>,
}

// Existing TSH types below...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorHash(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvariantId {
    Coupling,
    ScaleSeparation,
    ThermodynamicMeaning,
    FailClosed,
    StationaryFloor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StateLabel {
    Stationary,
    Drift,
    Entropy,
    Resonance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantResult {
    pub id: InvariantId,
    pub label: StateLabel,
    pub message: String,
    pub passing: bool,
}

impl InvariantResult {
    pub fn passed(&self) -> bool { self.passing }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSeal {
    pub hash: AnchorHash,
    pub timestamp: u64,
    pub anchor: String,
    pub label: StateLabel,
    pub sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceCommand {
    pub id: String,
    pub verb: String,
    pub target: String,
    pub anchor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub id: String,
    pub success: bool,
    pub message: String,
    pub exit_code: i32,
}
