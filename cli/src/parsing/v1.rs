use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::db::TestState;

use super::{JsonCourse, JsonTest, TestResult};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JsonTestV1 {
    pub name: String,
    pub optional: bool,
    pub cmd: String,
    pub message_on_fail: String,
    pub message_on_success: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JsonTestSuiteV1 {
    pub name: String,
    pub optional: bool,
    pub tests: Vec<JsonTestV1>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JsonCourseV1 {
    pub version: String,
    #[serde(rename = "course")]
    pub name: String,
    pub instructor: String,
    pub course_id: u64,
    pub suites: Vec<JsonTestSuiteV1>,
}

impl JsonTest for JsonTestV1 {
    fn run(&self) -> TestResult {
        log::debug!("Running test: '{}'", self.cmd);

        let command: Vec<&str> = self.cmd.split_whitespace().collect();

        let output = std::process::Command::new(command[0])
            .args(command[1..].iter())
            .output();
        let output = match output {
            Ok(output) => output,
            Err(_) => {
                return TestResult::Fail("could not execute test".to_string())
            }
        };

        log::debug!("Test executed successfully!");

        match output.status.success() {
            true => TestResult::Pass(String::from_utf8(output.stdout).unwrap()),
            false => {
                TestResult::Fail(String::from_utf8(output.stderr).unwrap())
            }
        }
    }
}

impl<'a> JsonCourse<'a> for JsonCourseV1 {
    fn name(&'a self) -> &'a str {
        &self.name
    }

    fn author(&'a self) -> &'a str {
        &self.instructor
    }

    fn list_tests(&self) -> IndexMap<String, TestState> {
        todo!()
    }
}
