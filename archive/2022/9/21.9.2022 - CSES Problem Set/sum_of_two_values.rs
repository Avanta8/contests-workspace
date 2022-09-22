//{"name":"Sum of Two Values","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1640","interactive":false,"timeLimit":1000,"tests":[{"input":"4 8\n2 7 5 1\n","output":"2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SumOfTwoValues"}}}

#![allow(
    unused_imports,
    non_snake_case,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::if_same_then_else,
    clippy::if_not_else,
    clippy::ifs_same_cond,
    clippy::type_complexity,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::needless_range_loop
)]

use std::collections::HashMap;

use algo_lib::{
    collections::{fxhash::FxHashMap, iter_ext::IterExt},
    io::{
        input::Input,
        output::output,
        task_io_settings::{TaskIoSettings, TaskIoType},
        task_runner::run_task,
    },
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let target = input.read::<u32>();

    let arr = input.read_vec::<u32>(n);

    // let mut seen = FxHashMap::default();
    let mut seen = HashMap::new();
    for (i, &x) in arr.iter().enumerate() {
        if let Some(j) = seen.get(&x) {
            out_line!(i + 1, j + 1);
            return;
        }
        seen.insert(target - x, i);
    }

    out_line!("IMPOSSIBLE");
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // submit();
}
//END MAIN
