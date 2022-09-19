use std::fmt::format;
use std::io::Cursor;
use std::path::Path;
use std::time::Instant;

use algo_lib::io::input::Input;
use algo_lib::io::output::{set_global_output_to_stdout, Output, OUTPUT};

const EPS: f64 = 1e-9;

fn is_equal_floats(f_actual: f64, f_expected: f64) -> bool {
    let abs_diff = (f_actual - f_expected).abs();
    abs_diff <= EPS || abs_diff <= f_expected.abs() * EPS
}

fn is_equal_float_tokens(token_actual: Vec<u8>, token_expected: Vec<u8>) -> bool {
    if let Ok(f_actual) = String::from_utf8(token_actual).unwrap().parse::<f64>() {
        if let Ok(f_expected) = String::from_utf8(token_expected).unwrap().parse::<f64>() {
            return is_equal_floats(f_actual, f_expected);
        }
    }
    false
}

fn get_tests_path() -> &'static str {
    let a = "./sum_of_two_values/tests/";
    let b = "./tests/";
    if Path::new(a).is_dir() {
        a
    } else if Path::new(b).is_dir() {
        b
    } else {
        panic!("Cannot find tests directory.");
    }
}

fn check(expected: &mut &[u8], actual: &mut &[u8]) -> Result<(), String> {
    let mut expected_cursor = Cursor::new(expected.to_vec());
    let mut expected = Input::new(&mut expected_cursor);

    let mut actual_cursor = Cursor::new(actual.to_vec());
    let mut actual = Input::new(&mut actual_cursor);

    let mut token_num = 0usize;
    loop {
        let expected_token = expected.next_token();
        let actual_token = actual.next_token();
        if expected_token != actual_token {
            if expected_token.is_none() {
                return Err(format!("Expected has only {} tokens", token_num));
            } else if actual_token.is_none() {
                return Err(format!("Actual has only {} tokens", token_num));
            } else if !is_equal_float_tokens(
                actual_token.clone().unwrap(),
                expected_token.clone().unwrap(),
            ) {
                return Err(format!(
                    "Token #{} differs, expected {}, actual {}",
                    token_num,
                    String::from_utf8(expected_token.unwrap()).unwrap(),
                    String::from_utf8(actual_token.unwrap()).unwrap()
                ));
            }
        }
        token_num += 1;
        if actual_token.is_none() {
            break;
        }
    }
    Ok(())
}

static mut OUT: Vec<u8> = Vec::new();

struct WriteDelegate {}

impl std::io::Write for WriteDelegate {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            OUT.append(&mut Vec::from(buf));
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/**

Returns [true] in case of successes

 */
fn run_single(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    let name = path.file_name().unwrap().to_str().unwrap();
    let name = &name[..name.len() - 3];
    let time_limit = std::time::Duration::from_millis(1000);
    let out_path = path.parent().unwrap().join(format!("{}.out", name));
    println!("{}Test {}{}", BLUE, name, DEF);
    println!("{}Input:{}", BLUE, DEF);
    println!(
        "{}",
        std::fs::read_to_string(&path).unwrap_or_else(|_| panic!(
            "Can't open file with test input: {}",
            path.to_string_lossy()
        ))
    );
    let expected = match std::fs::read_to_string(out_path) {
        Ok(res) => Some(res),
        Err(_) => None,
    };
    println!("{}Expected:{}", BLUE, DEF);
    match &expected {
        None => {
            println!("{}Not provided{}", YELLOW, DEF);
        }
        Some(expected) => {
            println!("{}", expected);
        }
    }
    println!("{}Output:{}", BLUE, DEF);
    match std::panic::catch_unwind(|| {
        unsafe {
            OUT.clear();
        }
        let mut file =
            std::fs::File::open(&path).unwrap_or_else(|_| panic!("Can't open file: {:?}", path));

        let input = Input::new(&mut file);
        let started = std::time::Instant::now();
        unsafe {
            OUTPUT = Some(Output::new(Box::new(WriteDelegate {})));
        }
        let is_exhausted = crate::run(input);
        let res = started.elapsed();
        let output;
        unsafe {
            output = OUT.clone();
        }
        println!("{}", String::from_utf8_lossy(&output));
        (output, res, is_exhausted)
    }) {
        Ok((output, duration, is_exhausted)) => {
            println!(
                "{}Time elapsed: {:.3}s{}",
                BLUE,
                (duration.as_millis() as f64) / 1000.,
                DEF,
            );
            if !is_exhausted {
                println!("{}Input not exhausted{}", RED, DEF);
            }
            if let Some(expected) = expected {
                let mut expected_bytes = expected.as_bytes();
                match check(&mut expected_bytes, &mut &output[..]) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("{}Verdict: {}Wrong Answer ({}){}", BLUE, RED, err, DEF);
                        return false;
                    }
                }
            }
            if duration > time_limit {
                println!("{}Verdict: {}Time Limit{}", BLUE, RED, DEF);
                return false;
            } else {
                println!("{}Verdict: {}OK{}", BLUE, GREEN, DEF)
            }
        }
        Err(err) => {
            match err.downcast::<&str>() {
                Ok(as_string) => println!(
                    "{}Verdict: {}RuntimeError ({:?}){}",
                    BLUE, RED, as_string, DEF
                ),
                Err(err) => println!("{}Verdict: {}RuntimeError ({:?}){}", BLUE, RED, err, DEF),
            }
            return false;
        }
    }
    true
}

const BLUE: &str = "\x1B[34m";
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const YELLOW: &str = "\x1B[33m";
const DEF: &str = "\x1B[0m";

#[allow(unused)]
pub(crate) fn run_tests() -> bool {
    let mut paths = std::fs::read_dir(get_tests_path())
        .expect("Unable to read tests.")
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();

    paths.sort_by_key(|a| a.path());
    let mut test_failed = 0usize;
    let mut test_total = 0usize;
    for path in paths {
        let sub_path = path;
        if sub_path.file_type().unwrap().is_file() {
            let path = sub_path.path();
            match path.extension() {
                None => {}
                Some(extension) => {
                    if extension.to_str() == Some("in") {
                        println!("=====================================================");
                        test_total += 1;
                        if !run_single(path) {
                            test_failed += 1;
                        }
                    }
                }
            }
        }
    }
    if test_failed == 0 {
        println!(
            "{}All {}{}{} tests passed{}",
            BLUE, GREEN, test_total, BLUE, DEF
        );
    } else {
        println!(
            "{}{}/{}{} tests failed{}",
            RED, test_failed, test_total, BLUE, DEF
        );
    }
    test_failed == 0
}

#[allow(unused)]
pub(crate) fn run_single_test(name: &str) -> bool {
    let tests_path = get_tests_path();
    let path = format!("{}{}.in", tests_path, name);
    run_single(path)
}

#[allow(unused)]
pub(crate) fn run_locally() {
    let mut sin = std::io::stdin();
    let input = Input::new(&mut sin);
    set_global_output_to_stdout();
    crate::run(input);
}

#[allow(unused)]
pub(crate) fn run_stress(stress: fn() -> ()) {
    set_global_output_to_stdout();
    let start = Instant::now();
    stress();
    eprintln!("Finished in {}ms", start.elapsed().as_millis());
}
