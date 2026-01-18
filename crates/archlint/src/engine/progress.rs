#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::indicatif::{ProgressBar, ProgressStyle};
#[cfg(feature = "cli")]
use indicatif::{ProgressBar, ProgressStyle};

#[must_use]
pub fn create_progress_bar(len: usize, template: &str, chars: &str) -> ProgressBar {
    let pb = ProgressBar::new(len as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(template)
            .expect("Invalid progress bar template")
            .progress_chars(chars),
    );
    pb
}

#[must_use]
pub const fn default_spinner_template() -> &'static str {
    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}"
}

#[must_use]
pub const fn detector_progress_template() -> &'static str {
    "{spinner:.green} [{elapsed_precise}] [{bar:40.green/white}] {pos}/{len} {msg}"
}

#[must_use]
pub const fn default_progress_chars() -> &'static str {
    "█▉▊▋▌▍▎▏  "
}
