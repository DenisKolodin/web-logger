pub mod console {
    use web_sys::console;

    pub fn trace(message: &str) {
        console::log_1(&message.into());
    }

    pub fn debug(message: &str) {
        console::debug_1(&message.into());
    }

    pub fn info(message: &str) {
        console::info_1(&message.into());
    }

    pub fn warn(message: &str) {
        console::warn_1(&message.into());
    }

    pub fn error(message: &str) {
        console::error_1(&message.into());
    }
}
