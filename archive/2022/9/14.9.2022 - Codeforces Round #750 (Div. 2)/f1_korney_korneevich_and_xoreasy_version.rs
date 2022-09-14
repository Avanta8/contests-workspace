//{"name":"F1. Korney Korneevich and XOR (easy version)","group":"Codeforces - Codeforces Round #750 (Div. 2)","url":"https://codeforces.com/contest/1582/problem/F1","interactive":false,"timeLimit":1500,"tests":[{"input":"4\n4 2 2 4\n","output":"4\n0 2 4 6\n"},{"input":"8\n1 0 1 7 12 5 3 2\n","output":"12\n0 1 2 3 4 5 6 7 10 11 12 13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1KorneyKorneevichAndXOREasyVersion"}}}

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

use std::cmp::min;

use algo_lib::{
    collections::iter_ext::IterExt,
    io::{
        input::Input,
        output::output,
        task_io_settings::{TaskIoSettings, TaskIoType},
        task_runner::run_task,
    },
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

const N: usize = 512;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let arr = input.read_vec::<usize>(n);

    let mut table = vec![None; N];
    table[0] = Some(0);

    for &item in arr.iter() {
        for can_make in 0..N {
            if let Some(min_use) = table[can_make] {
                if min_use < item {
                    let new_make = item ^ can_make;
                    table[new_make] = Some(table[new_make].map_or(item, |r| min(r, item)));
                }
            }
        }
    }

    let ans = table
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|_| i))
        .collect_vec();

    out_line!(ans.len());
    out_line!(ans);
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
