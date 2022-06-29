use serde::{Serialize, Deserialize};
use crate::models_md5_hash_cash::MD5HashCashInput;
use crate::models_md5_hash_cash::MD5HashCashOutput;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge {
    NONE,
    MD5HashCash(MD5HashCashInput),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    NONE,
    MD5HashCash(MD5HashCashOutput),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub name: ChallengeAnswer,
    pub next_target: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary {
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame {
    pub leader_board: Vec<PublicPlayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}