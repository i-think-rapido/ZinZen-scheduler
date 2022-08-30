extern crate scheduler;
mod common;
use scheduler::{Input, Output};
use std::path::Path;

#[test]
#[ignore]
fn basic_test_works() {
	let input_path = Path::new("./js-tests/basic-test/input.json");
	let output_path = Path::new("./js-tests/basic-test/output.json");
	let input: Input = common::get_input_from_json(input_path).unwrap();
	let desired_output: String = common::get_output_string_from_json(output_path).unwrap();

	let output: Vec<Output> = scheduler::run_scheduler(input);
	assert_eq!(
		serde_json::to_string(&output).unwrap(),
		desired_output
	);
}

#[test]
fn repetition_daily_works() {
    let input_path = Path::new("./js-tests/repetition-daily/input.json");
	let output_path = Path::new("./js-tests/repetition-daily/output.json");
	let input: Input = common::get_input_from_json(input_path).unwrap();
	let desired_output: String = common::get_output_string_from_json(output_path).unwrap();

	let output: Vec<Output> = scheduler::run_scheduler(input);
	assert_eq!(
		serde_json::to_string(&output).unwrap(),
		desired_output
	);
}

#[test]
fn repetition_daily_bounds_works() {
    let input_path = Path::new("./js-tests/repetition-daily-bounds/input.json");
	let output_path = Path::new("./js-tests/repetition-daily-bounds/output.json");
	let input: Input = common::get_input_from_json(input_path).unwrap();
	let desired_output: String = common::get_output_string_from_json(output_path).unwrap();

	let output: Vec<Output> = scheduler::run_scheduler(input);
	assert_eq!(
		serde_json::to_string(&output).unwrap(),
		desired_output
	);
}
