pub fn tutorial_capture_enabled() -> bool {
    matches!(
        std::env::var("BEVY_TUTORIAL_CAPTURE").as_deref(),
        Ok("1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON")
    )
}
