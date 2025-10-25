pub fn get_client_id() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get UNIX timestamp because UNIX_EPOCH is later than now")
        .as_millis() as u64
}
