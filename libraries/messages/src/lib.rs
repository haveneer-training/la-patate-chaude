
pub mod message {
    use serde::{Deserialize, Serialize};
    use serde_unit_struct::{Deserialize_unit_struct, Serialize_unit_struct};

    #[derive(Serialize_unit_struct, Deserialize_unit_struct, Debug)]
    pub struct Hello;

    impl Hello {
        pub fn new() -> Hello {
            return Hello {};
        }
        pub fn to_string() -> String {
            return serde_json::to_string(&Hello).unwrap();
        }
        pub fn from_string(s: &str) -> Hello {
            if s != "Hello" {
                panic!("Expected 'Hello', got '{}'", s);
            }
            return Hello::new();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Welcome {
        pub version: u8,
    }
    impl Welcome {
        pub fn new(version: u8) -> Welcome {
            return Welcome { version };
        }
        pub fn to_string(welcome: &Welcome) -> String {
            return serde_json::to_string(welcome).unwrap();
        }
        pub fn from_string(s: &str) -> Welcome {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum SuscribeError {
        AlreadyRegistered,
        InvalidName,
        Null,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum SuscribeResult {
        Ok,
        Err(SuscribeError),
    }
    impl SuscribeResult {
        pub fn new(ok: bool, err: SuscribeError) -> SuscribeResult {
            if ok {
                return SuscribeResult::Ok;
            }
            return SuscribeResult::Err(err);
        }
        pub fn to_string(suscribe_result: &SuscribeResult) -> String {
            return serde_json::to_string(suscribe_result).unwrap();
        }
        pub fn from_string(s: &str) -> SuscribeResult {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PublicPlayer {
        pub name: String,
        pub stream_id: String,
        pub score: i32,
        pub steps: u32,
        pub is_active: bool,
        pub total_used_time: f64,
    }
    impl PublicPlayer {
        pub fn new(name: String, stream_id: String, score: i32, steps: u32, is_active: bool, total_used_time: f64) -> PublicPlayer {
            return PublicPlayer { name, stream_id, score, steps, is_active, total_used_time };
        }
        pub fn to_string(public_player: &PublicPlayer) -> String {
            return serde_json::to_string(public_player).unwrap();
        }
        pub fn from_string(s: &str) -> PublicPlayer {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(transparent)]
    pub struct PublicLeaderBoard {
        pub players: Vec<PublicPlayer>,
    }
    impl PublicLeaderBoard {
        pub fn new(players: Vec<PublicPlayer>) -> PublicLeaderBoard {
            return PublicLeaderBoard { players };
        }
        pub fn to_string(public_leader_board: &PublicLeaderBoard) -> String {
            return serde_json::to_string(public_leader_board).unwrap();
        }
        pub fn from_string(s: &str) -> PublicLeaderBoard {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ChallengeType {
        MD5HashCash {
            complexity: i32,
            message: String
        },
    }
    impl ChallengeType {
        pub fn new_md5_hashcash(complexity: i32, message: String) -> ChallengeType {
            return ChallengeType::MD5HashCash{ complexity, message };
        }
        pub fn to_string(challenge_type: &ChallengeType) -> String {
            return serde_json::to_string(challenge_type).unwrap();
        }
        pub fn from_string(s: &str) -> ChallengeType {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Challenge {
        #[serde(rename(serialize = "Challenge", deserialize = "Challenge"))]
        pub challenge: ChallengeType,
    }
    impl Challenge {
        pub fn new(challenge: ChallengeType) -> Challenge {
            return Challenge { challenge };
        }
        pub fn to_string(challenge: &Challenge) -> String {
            return serde_json::to_string(challenge).unwrap();
        }
        pub fn from_string(s: &str) -> Challenge {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ChallengeAnswerType {
        MD5HashCash {
            seed: String,
            hashcode: String
        },
    }
    impl ChallengeAnswerType {
        pub fn new_md5_hashcash(seed: String, hashcode: String) -> ChallengeAnswerType {
            return ChallengeAnswerType::MD5HashCash{ seed, hashcode };
        }
        pub fn to_string(challenge_answer_type: &ChallengeAnswerType) -> String {
            return serde_json::to_string(challenge_answer_type).unwrap();
        }
        pub fn from_string(s: &str) -> ChallengeAnswerType {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChallengeAnswer {
        pub answer: ChallengeAnswerType,
        pub next_target: String,
    }
    impl ChallengeAnswer {
        pub fn new(answer: ChallengeAnswerType, next_target: String) -> ChallengeAnswer {
            return ChallengeAnswer { answer, next_target };
        }
        pub fn to_string(challenge_answer: &ChallengeAnswer) -> String {
            return serde_json::to_string(challenge_answer).unwrap();
        }
        pub fn from_string(s: &str) -> ChallengeAnswer {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ReportedChallengeResult {
        name: String,
        value: String,
    }
    impl ReportedChallengeResult {
        pub fn new(name: String, value: String) -> ReportedChallengeResult {
            return ReportedChallengeResult { name, value };
        }
        pub fn to_string(repported_challenge_result: &ReportedChallengeResult) -> String {
            return serde_json::to_string(repported_challenge_result).unwrap();
        }
        pub fn from_string(s: &str) -> ReportedChallengeResult {
            return serde_json::from_str(s).unwrap();
        }
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct RoundSummary {
        challenge: String,
        chain: Vec<ReportedChallengeResult>
    }
    impl RoundSummary {
        pub fn new(challenge: String, chain: Vec<ReportedChallengeResult>) -> RoundSummary {
            return RoundSummary { challenge, chain };
        }
        pub fn to_string(round_summary: &RoundSummary) -> String {
            return serde_json::to_string(round_summary).unwrap();
        }
        pub fn from_string(s: &str) -> RoundSummary {
            return serde_json::from_str(s).unwrap();
        }
    }

}

#[cfg(test)]    
mod tests {
    // Hello tests
    #[test]
    fn hello_serialize_test() {
        assert_eq!(super::message::Hello::to_string(), "\"Hello\"");
    }
    #[test]
    #[should_panic(expected = "Expected 'Hello', got 'Goodbye'")]
    fn hello_deserialize_test() {
        super::message::Hello::from_string("Goodbye");   
    }

    // Welcome tests
    #[test]
    fn welcome_serialize_test() {
        let welcome = super::message::Welcome::new(1);
        assert_eq!(super::message::Welcome::to_string(&welcome), "{\"version\":1}");
    }
    #[test]
    fn welcome_deserialize_test() {
        let welcome = super::message::Welcome::from_string("{\"version\":1}");
        assert_eq!(welcome.version, 1);
    }

    // SuscribeResults tests
    #[test]
    fn suscribe_result_serialize_test_with_invalid_name() {
        let suscribe_result = super::message::SuscribeResult::new(false, super::message::SuscribeError::InvalidName);
        assert_eq!(super::message::SuscribeResult::to_string(&suscribe_result), "{\"Err\":\"InvalidName\"}");
    }
    #[test]
    fn suscribe_result_serialize_test_with_already_registered_user() {
        let suscribe_result = super::message::SuscribeResult::new(false, super::message::SuscribeError::AlreadyRegistered);
        assert_eq!(super::message::SuscribeResult::to_string(&suscribe_result), "{\"Err\":\"AlreadyRegistered\"}");
    }
    #[test]
    fn suscribe_result_serialize_test_with_message_ok() {
        let suscribe_result = super::message::SuscribeResult::new(true, super::message::SuscribeError::Null);
        assert_eq!(super::message::SuscribeResult::to_string(&suscribe_result), "\"Ok\"");
    }

    // PublicPlayer test
    #[test]
    fn public_player_serialize_test() {
        let public_player = super::message::PublicPlayer::new("name".to_string(), "stream_id".to_string(), 1, 1, true, 1.0);
        assert_eq!(super::message::PublicPlayer::to_string(&public_player), "{\"name\":\"name\",\"stream_id\":\"stream_id\",\"score\":1,\"steps\":1,\"is_active\":true,\"total_used_time\":1.0}");
    }
    #[test]
    fn public_player_deserialize_test() {
        let public_player = super::message::PublicPlayer::from_string("{\"name\":\"name\",\"stream_id\":\"stream_id\",\"score\":1,\"steps\":1,\"is_active\":true,\"total_used_time\":1.0}");
        assert_eq!(public_player.name, "name");
    }
    #[test]
    fn public_leaderboard_serialize_test() {
        let public_leader_board = super::message::PublicLeaderBoard::new(vec![super::message::PublicPlayer::new("name".to_string(), "stream_id".to_string(), 1, 1, true, 1.0)]);
        assert_eq!(super::message::PublicLeaderBoard::to_string(&public_leader_board), "[{\"name\":\"name\",\"stream_id\":\"stream_id\",\"score\":1,\"steps\":1,\"is_active\":true,\"total_used_time\":1.0}]");
    }
    #[test]
    fn public_leaderboard_deserialize_test() {
        let public_leader_board = super::message::PublicLeaderBoard::from_string("[{\"name\":\"name\",\"stream_id\":\"stream_id\",\"score\":1,\"steps\":1,\"is_active\":true,\"total_used_time\":1.0}]");
        assert_eq!(public_leader_board.players[0].name, "name");
        assert_eq!(public_leader_board.players[0].stream_id, "stream_id");
        assert_eq!(public_leader_board.players[0].score, 1);
        assert_eq!(public_leader_board.players[0].steps, 1);
        assert_eq!(public_leader_board.players[0].is_active, true);
        assert_eq!(public_leader_board.players[0].total_used_time, 1.0);
    }

    // Challenge tests
    #[test]
    fn challenge_serialize_test() {
        let challenge = super::message::Challenge::new(super::message::ChallengeType::new_md5_hashcash(1, "message".to_string()));
        assert_eq!(super::message::Challenge::to_string(&challenge), "{\"Challenge\":{\"MD5HashCash\":{\"complexity\":1,\"message\":\"message\"}}}");
    }
    // #[test]
    // fn challenge_deserialize_test() {
    //     let challenge = super::message::Challenge::from_string("{\"Challenge\":{\"MD5HashCash\":{\"complexity\":1,\"message\":\"message\"}}}");
    //     assert_eq!(challenge.challenge.MD5HashCash, super::message::ChallengeType::new_md5_hashcash(1, "message".to_string()));
    // }

    // ChallengeResult tests
    // TODO

    // RoundSummary tests
    // #[test]
    // fn round_summary_serialize_test() {
    //     let challenge = super::message::Challenge::new(super::message::ChallengeType::new_md5_hashcash(1, "message".to_string()));
    //     let round_summary = super::message::RoundSummary::new(challenge, vec![super::message::PublicPlayer::new("name".to_string(), "stream_id".to_string(), 1, 1, true, 1.0)]);
    //     assert_eq!(super::message::RoundSummary::to_string(&round_summary), "{\"Challenge\":{\"MD5HashCash\":{\"complexity\":1,\"message\":\"message\"}},\"LeaderBoard\":[{\"name\":\"name\",\"stream_id\":\"stream_id\",\"score\":1,\"steps\":1,\"is_active\":true,\"total_used_time\":1.0}]}");
    // }
    // #[test]
    // fn round_summary_deserialize_test() {
    //     let challenge = super::message::Challenge::new(super::message::ChallengeType::new_md5_hashcash(1, "message".to_string()));
    //     let round_summary = super::message::RoundSummary::from_string("{\"Challenge\":{\"MD5HashCash\":{\"complexity\":1,\"message\":\"message\"}},\"LeaderBoard\":[{\"name\":\"name\",\"stream_id\":\"stream_id\",\"score\":1,\"steps\":1,\"is_active\":true,\"total_used_time\":1.0}]}");
    //     assert_eq!(round_summary.challenge.MD5HashCash, super::message::ChallengeType::new_md5_hashcash(1, "message".to_string()));
    //     assert_eq!(round_summary.leader_board[0].name, "name");
    //     assert_eq!(round_summary.leader_board[0].stream_id, "stream_id");
    //     assert_eq!(round_summary.leader_board[0].score, 1);
    //     assert_eq!(round_summary.leader_board[0].steps, 1);
    //     assert_eq!(round_summary.leader_board[0].is_active, true);
    //     assert_eq!(round_summary.leader_board[0].total_used_time, 1.0);
    // }
}
