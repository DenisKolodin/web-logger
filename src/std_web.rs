pub mod console {
    pub fn trace(message: &str) {
        js! { @(no_return) console.log(@{message}); }
    }

    pub fn debug(message: &str) {
        js! { @(no_return) console.debug(@{message}); }
    }

    pub fn info(message: &str) {
        js! { @(no_return) console.info(@{message}); }
    }

    pub fn warn(message: &str) {
        js! { @(no_return) console.warn(@{message}); }
    }

    pub fn error(message: &str) {
        js! { @(no_return) console.error(@{message}); }
    }
}
