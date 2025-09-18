use art_of_rally_leaderboard_api::Platform;

pub static WEBHOOK_URL: &str = "localhost";

pub fn users() -> (Platform, Vec<u64>, Vec<&'static str>, Vec<&'static str>) {
    return (Platform::Steam, vec![1], vec!["a"], vec!["b"]);
}
