#[derive(Debug)]
pub struct ExecutorOptions {
    pub assume_yes: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for ExecutorOptions {
    fn default() -> Self {
        Self { assume_yes: false }
    }
}
