
#[allow(dead_code)] 
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
    pub struct PublicLeaderBoard {
        #[serde(rename(serialize = "PublicLeaderBoard", deserialize = "PublicLeaderBoard"))]
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
    pub struct MD5HashCashInput {
        pub complexity: i32,
        pub message: String
    }
    impl MD5HashCashInput {
        pub fn new(complexity: i32, message: String) -> MD5HashCashInput {
            return MD5HashCashInput { complexity, message };
        }
        pub fn to_string(md5_hash_cash: &MD5HashCashInput) -> String {
            return serde_json::to_string(md5_hash_cash).unwrap();
        }
        pub fn from_string(s: &str) -> MD5HashCashInput {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MonstrousMazeInput {
        pub grid: String,
        pub endurance: u8,
    }
    impl MonstrousMazeInput {
        pub fn new(grid: String, endurance: u8) -> MonstrousMazeInput {
            return MonstrousMazeInput { grid, endurance };
        }
        pub fn to_string(monstrous_maze_input: &MonstrousMazeInput) -> String {
            return serde_json::to_string(monstrous_maze_input).unwrap();
        }
        pub fn from_string(s: &str) -> MonstrousMazeInput {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ChallengeType {
        MD5HashCash(MD5HashCashInput),
        MonstrousMaze(MonstrousMazeInput), 
    }
    impl ChallengeType {
        pub fn new_md5_hashcash(complexity: i32, message: String) -> ChallengeType {
            return ChallengeType::MD5HashCash(MD5HashCashInput::new(complexity, message));
        }
        pub fn new_maze(grid: String, endurance: u8) -> ChallengeType {
            return ChallengeType::MonstrousMaze(MonstrousMazeInput::new(grid, endurance));
        }
        pub fn to_string(challenge_type: &ChallengeType) -> String {
            match challenge_type {
                ChallengeType::MD5HashCash(md5_hash_cash) => return serde_json::to_string(md5_hash_cash).unwrap(),
                ChallengeType::MonstrousMaze(monstrous_maze_input) => return serde_json::to_string(monstrous_maze_input).unwrap(),
            }
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
    pub struct MD5HashCashAnswer {
        pub seed: u64,
        pub hashcode: String
    }
    impl MD5HashCashAnswer {
        pub fn new(seed : u64, hashcode: String) -> MD5HashCashAnswer {
            return MD5HashCashAnswer { seed: seed, hashcode: hashcode }
        }
        pub fn to_string(md5_cash_output: &MD5HashCashAnswer) -> String {
            return serde_json::to_string(md5_cash_output).unwrap();
        }
        pub fn from_string(s: &str) -> MD5HashCashAnswer {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MonstrousMazeAnswer {
        pub path: String
    }
    impl MonstrousMazeAnswer {
        pub fn new(path: String) -> MonstrousMazeAnswer {
            return MonstrousMazeAnswer { path: path }
        }
        pub fn to_string(monstrous_maze_output: &MonstrousMazeAnswer) -> String {
            return serde_json::to_string(monstrous_maze_output).unwrap();
        }
        pub fn from_string(s: &str) -> MonstrousMazeAnswer {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ChallengeAnswerType {
        MD5HashCash(MD5HashCashAnswer),
        MonstrousMaze(MonstrousMazeAnswer)
    }
    impl ChallengeAnswerType {
        pub fn new_md5_hashcash(seed: u64, hashcode: String) -> ChallengeAnswerType {
            return ChallengeAnswerType::MD5HashCash(MD5HashCashAnswer::new(seed, hashcode));
        }
        pub fn new_maze(path: String) -> ChallengeAnswerType {
            return ChallengeAnswerType::MonstrousMaze(MonstrousMazeAnswer::new(path));
        }
        pub fn to_string(challenge_type: &ChallengeAnswerType) -> String {
            match challenge_type {
                ChallengeAnswerType::MD5HashCash(md5_hash_cash) => return serde_json::to_string(md5_hash_cash).unwrap(),
                ChallengeAnswerType::MonstrousMaze(monstrous_maze_input) => return serde_json::to_string(monstrous_maze_input).unwrap(),
            }
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
    pub struct ChallengeResult {
        #[serde(rename(serialize = "ChallengeResult", deserialize = "ChallengeResult"))]
        pub challenge_answer: ChallengeAnswer,
    }
    impl ChallengeResult {
        pub fn new(answer: ChallengeAnswerType, next_target: String) -> ChallengeResult {
            return ChallengeResult { challenge_answer: ChallengeAnswer { answer: answer, next_target: next_target} };
        }
        pub fn to_string(challenge_result: &ChallengeResult) -> String {
            return serde_json::to_string(challenge_result).unwrap();
        }
        pub fn from_string(s: &str) -> ChallengeResult {
            return serde_json::from_str(s).unwrap();
        }
    }
    

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ChallengeValue {
        Unreachable,
        Timeout,
        BadResult {
            used_time: f64,
            next_target: String
        },
        Ok {
            used_time: f64,
            next_target: String
        }
    }
    impl ChallengeValue {
        pub fn new_unreachable() -> ChallengeValue {
            return ChallengeValue::Unreachable;
        }
        pub fn new_timeout() -> ChallengeValue {
            return ChallengeValue::Timeout;
        }
        pub fn new_bad_result(used_time: f64, next_target: String) -> ChallengeValue {
            return ChallengeValue::BadResult { used_time: used_time, next_target: next_target };
        }
        pub fn new_ok(used_time: f64, next_target: String) -> ChallengeValue {
            return ChallengeValue::Ok { used_time: used_time, next_target: next_target };
        }
        pub fn from_string(s: &str) -> ChallengeValue {
            return serde_json::from_str(s).unwrap();
        }
        pub fn to_string(challenge_value: &ChallengeValue) -> String {
            return serde_json::to_string(challenge_value).unwrap();
        }
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ReportedChallengeResult {
        pub name: String,
        pub value: ChallengeValue,
    }
    impl ReportedChallengeResult {
        pub fn new(name: String, value: ChallengeValue) -> ReportedChallengeResult {
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
    pub struct RoundSummaryAnswer {
        pub challenge: String,
        pub chain: Vec<ReportedChallengeResult>
    }
    impl RoundSummaryAnswer {
        pub fn new(challenge: String, chain: Vec<ReportedChallengeResult>) -> RoundSummaryAnswer {
            return RoundSummaryAnswer { challenge, chain };
        }
        pub fn to_string(round_summary_answer: &RoundSummaryAnswer) -> String {
            return serde_json::to_string(round_summary_answer).unwrap();
        }
        pub fn from_string(s: &str) -> RoundSummaryAnswer {
            return serde_json::from_str(s).unwrap();
        }
    }
        
    #[derive(Serialize, Deserialize, Debug)]
    pub struct RoundSummary {
        #[serde(rename(serialize = "RoundSummary", deserialize = "RoundSummary"))]
        pub answer: RoundSummaryAnswer
    }
    impl RoundSummary {
        pub fn new(challenge: String, chain: Vec<ReportedChallengeResult>) -> RoundSummary {
            return RoundSummary { answer: RoundSummaryAnswer { challenge, chain } };
        }
        pub fn to_string(round_summary: &RoundSummary) -> String {
            return serde_json::to_string(round_summary).unwrap();
        }
        pub fn from_string(s: &str) -> RoundSummary {
            return serde_json::from_str(s).unwrap();
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct EndOfGameAnswer {
        pub leader_board: Vec<PublicPlayer>
    }

    impl EndOfGameAnswer {
        pub fn new(leader_board: Vec<PublicPlayer>) -> EndOfGameAnswer {
            return EndOfGameAnswer{leader_board: leader_board};
        }
        pub fn to_string(end_of_game_answer: &EndOfGameAnswer) -> String {
            return serde_json::to_string(end_of_game_answer).unwrap();
        }
        pub fn from_string(s: &str) -> EndOfGameAnswer {
            return serde_json::from_str(s).unwrap();
        }
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct EndOfGame {
        #[serde(rename(serialize = "EndOfGame", deserialize = "EndOfGame"))]
        pub answer :  EndOfGameAnswer
    }
    impl EndOfGame {
        pub fn new(leader_board: Vec<PublicPlayer>) -> EndOfGame {
            return EndOfGame {answer: EndOfGameAnswer{leader_board: leader_board}};
        }
        pub fn to_string(end_of_game: &EndOfGame) -> String {
            return serde_json::to_string(end_of_game).unwrap();
        }
        pub fn from_string(s: &str) -> EndOfGame {
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
        assert_eq!(super::message::PublicLeaderBoard::to_string(&public_leader_board), "{\"PublicLeaderBoard\":[{\"name\":\"name\",\"stream_id\":\"stream_id\",\"score\":1,\"steps\":1,\"is_active\":true,\"total_used_time\":1.0}]}");
    }
    #[test]
    fn public_leaderboard_deserialize_test() {
        let public_leader_board = super::message::PublicLeaderBoard::from_string("{\"PublicLeaderBoard\":[{\"name\":\"name\",\"stream_id\":\"stream_id\",\"score\":1,\"steps\":1,\"is_active\":true,\"total_used_time\":1.0}]}");
        assert_eq!(public_leader_board.players[0].name, "name");
        assert_eq!(public_leader_board.players[0].stream_id, "stream_id");
        assert_eq!(public_leader_board.players[0].score, 1);
        assert_eq!(public_leader_board.players[0].steps, 1);
        assert_eq!(public_leader_board.players[0].is_active, true);
        assert_eq!(public_leader_board.players[0].total_used_time, 1.0);
    }
    // Challenge tests
    #[test]
    fn md5_challenge_serialize_test() {
        let challenge = super::message::Challenge::new(super::message::ChallengeType::new_md5_hashcash(1, "message".to_string()));
        assert_eq!(super::message::Challenge::to_string(&challenge), "{\"Challenge\":{\"MD5HashCash\":{\"complexity\":1,\"message\":\"message\"}}}");
    }
    #[test]
    fn md5_challenge_deserialize_test() {
        let challenge = super::message::Challenge::from_string("{\"Challenge\":{\"MD5HashCash\":{\"complexity\":1,\"message\":\"message\"}}}");
        match challenge.challenge {
            super::message::ChallengeType::MD5HashCash(md5_hashcash) => {
                assert_eq!(md5_hashcash.complexity, 1);
                assert_eq!(md5_hashcash.message, "message");
            }
            _ => panic!("Expected MD5HashCash challenge")
        }
    }
    #[test]
    fn maze_challenge_serialize_test() {
        let challenge = super::message::Challenge::new(super::message::ChallengeType::new_maze("||".to_string(), 1));
        assert_eq!(super::message::Challenge::to_string(&challenge), "{\"Challenge\":{\"MonstrousMaze\":{\"grid\":\"||\",\"endurance\":1}}}");
    }
    #[test]
    fn monstrous_deserialize_test() {
        let challenge = super::message::Challenge::from_string("{\"Challenge\":{\"MonstrousMaze\":{\"grid\":\"||\",\"endurance\":1}}}");
        match challenge.challenge {
            super::message::ChallengeType::MonstrousMaze(monstrous_maze) => {
                assert_eq!(monstrous_maze.grid, "||");
                assert_eq!(monstrous_maze.endurance, 1);
            }
            _ => panic!("Expected MonstrousMaze challenge")
        }
    }
    // Challenge result
    #[test]
    fn md5_hashcash_challenge_answer_serialize() {
        let seed : u64 = 1;
        let answer_type = super::message::ChallengeAnswerType::new_md5_hashcash(seed, "ckjzkrnkaj".to_string());
        let challenge_result = super::message::ChallengeResult::new(answer_type, "answer".to_string());
        assert_eq!(super::message::ChallengeResult::to_string(&challenge_result), "{\"ChallengeResult\":{\"answer\":{\"MD5HashCash\":{\"seed\":1,\"hashcode\":\"ckjzkrnkaj\"}},\"next_target\":\"answer\"}}");
    }
    #[test]
    fn monstrous_maze_challenge_answer_serialize() {
        let answer_type = super::message::ChallengeAnswerType::new_maze("||".to_string());
        let challenge_result = super::message::ChallengeResult::new(answer_type, "answer".to_string());
        assert_eq!(super::message::ChallengeResult::to_string(&challenge_result), "{\"ChallengeResult\":{\"answer\":{\"MonstrousMaze\":{\"path\":\"||\"}},\"next_target\":\"answer\"}}");
    }
    #[test]
    fn md5_hashcash_challenge_answer_deserialize() {
        let challenge_result = super::message::ChallengeResult::from_string("{\"ChallengeResult\":{\"answer\":{\"MD5HashCash\":{\"seed\":1,\"hashcode\":\"ckjzkrnkaj\"}},\"next_target\":\"answer\"}}");
        match challenge_result.challenge_answer.answer {
            super::message::ChallengeAnswerType::MD5HashCash(md5_hashcash) => {
                assert_eq!(md5_hashcash.seed, 1);
                assert_eq!(md5_hashcash.hashcode, "ckjzkrnkaj");
                assert_eq!(challenge_result.challenge_answer.next_target, "answer");
            }
            _ => panic!("Expected MD5HashCash challenge")
        }
    }
    #[test]
    fn monstrous_maze_challenge_answer_deserialize() {
        let challenge_result = super::message::ChallengeResult::from_string("{\"ChallengeResult\":{\"answer\":{\"MonstrousMaze\":{\"path\":\"||\"}},\"next_target\":\"answer\"}}");
        match challenge_result.challenge_answer.answer {
            super::message::ChallengeAnswerType::MonstrousMaze(monstrous_maze) => {
                assert_eq!(monstrous_maze.path, "||");
                assert_eq!(challenge_result.challenge_answer.next_target, "answer");
            }
            _ => panic!("Expected MonstrousMaze challenge")
        }
    }

    // RoundSummary test
    #[test]
    fn round_summary_serialize() {
        let ok = super::message::ChallengeValue::new_ok(1.3443_f64, "toto".to_string());
        let unreachable = super::message::ChallengeValue::new_unreachable();
        let vec_chain = vec![super::message::ReportedChallengeResult::new("md5".to_string(), ok), super::message::ReportedChallengeResult::new("monstrous".to_string(), unreachable)];
        let round_summary = super::message::RoundSummary::new("challenge".to_string(), vec_chain);
        assert_eq!(super::message::RoundSummary::to_string(&round_summary), "{\"RoundSummary\":{\"challenge\":\"challenge\",\"chain\":[{\"name\":\"md5\",\"value\":{\"Ok\":{\"used_time\":1.3443,\"next_target\":\"toto\"}}},{\"name\":\"monstrous\",\"value\":\"Unreachable\"}]}}");
    }
    #[test]
    fn round_summary_deserialize() {
        let round_summary = super::message::RoundSummary::from_string("{\"RoundSummary\":{\"challenge\":\"challenge\",\"chain\":[{\"name\":\"md5\",\"value\":{\"Ok\":{\"used_time\":1.3443,\"next_target\":\"toto\"}}},{\"name\":\"monstrous\",\"value\":\"Unreachable\"}]}}");
        assert_eq!(round_summary.answer.challenge, "challenge");
        assert_eq!(round_summary.answer.chain.len(), 2);
        assert_eq!(round_summary.answer.chain[0].name, "md5");
        match round_summary.answer.chain[1].value {
            super::message::ChallengeValue::Unreachable => assert_eq!(0,0),
            _ => ()
            
        }
        assert_eq!(round_summary.answer.chain[1].name, "monstrous");
    }

    // End of game
    #[test]
    fn end_of_game_serialize() {
        let player1 = super::message::PublicPlayer::new("jean".to_string(), "123".to_string(), 5_i32, 67_u32, true, 562.67_f64);
        let player2 = super::message::PublicPlayer::new("adem".to_string(), "124".to_string(), 7_i32, 67_u32, true, 63.67_f64);

        let end_of_game = super::message::EndOfGame::new(vec![player1, player2]);
        assert_eq!(super::message::EndOfGame::to_string(&end_of_game), 
        "{\"EndOfGame\":{\"leader_board\":[{\"name\":\"jean\",\"stream_id\":\"123\",\"score\":5,\"steps\":67,\"is_active\":true,\"total_used_time\":562.67},{\"name\":\"adem\",\"stream_id\":\"124\",\"score\":7,\"steps\":67,\"is_active\":true,\"total_used_time\":63.67}]}}"
        );

    }

}