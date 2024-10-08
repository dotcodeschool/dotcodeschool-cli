use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use crate::monitor::StateMachine;

use self::v1::RunnerV1;

pub mod v1;

pub enum RunnerVersion {
    V1(RunnerV1),
}

impl StateMachine for RunnerVersion {
    fn run(self) -> Self {
        match self {
            RunnerVersion::V1(runner) => Self::V1(runner.run()),
        }
    }

    fn is_finished(&self) -> bool {
        match self {
            RunnerVersion::V1(runner) => runner.is_finished(),
        }
    }
}

/// Formats tests `stderr` and `stdout` output.
///
/// Format is as follows:
///
/// ```bash
/// ╭─[ output ]
/// │ {output}
/// ╰─[ {msg} ]
/// ```
///
/// * `output`: test output.
/// * `msg`: custom message to display after the output.
fn format_output(output: &str, msg: &str) -> String {
    let output = output.replace("\n", "\n    │");
    format!("    ╭─[ output ]{output}\n    ╰─[ {msg} ]")
}

#[allow(dead_code)]
fn format_spinner(progress: &ProgressBar) {
    progress.set_style(
        ProgressStyle::with_template("\n{spinner} {msg} {elapsed_precise}")
            .unwrap(),
    );
    progress.enable_steady_tick(Duration::from_millis(50));
}

fn format_bar(progress: &ProgressBar) {
    progress.set_style(
        ProgressStyle::with_template("{wide_bar} {message} {elapsed_precise}")
            .unwrap(),
    );
}
