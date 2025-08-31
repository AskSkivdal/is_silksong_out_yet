use chrono::Utc;

const SILKSONG_TS: i64 = 1756994400;

/// Checks if Hollow Knight: Silksong released.
/// This function will return true if silksong is released, and return false if silksong is not
/// released yet.
pub fn is_silksong_out_yet() -> bool {
    let now = Utc::now().timestamp();

    return now > SILKSONG_TS;
}

/// Checks if Hollow Knight: Silksong dlc is released yet.
/// This function will currently always return false because we do not know when the dlc will be
/// released. This will be updated in a future version of this crate.
pub fn is_silksong_dlc_out_yet() -> bool {
    return false;
}
