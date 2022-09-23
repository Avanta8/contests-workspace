//{"name":"D1. Zero-One (Easy Version)","group":"Codeforces - Codeforces Round #821 (Div. 2)","url":"https://codeforces.com/contest/1733/problem/D1","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n5 8 7\n01001\n00101\n5 7 2\n01000\n11011\n7 8 3\n0111001\n0100001\n5 10 1\n01100\n01100\n","output":"8\n-1\n6\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1ZeroOneEasyVersion"}}}

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

use std::cmp::{max, min};

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

fn solve(input: &mut Input, _test_case: usize) {
    let (n, x, y) = input.read::<(usize, i64, i64)>();

    if x < y {
        std::process::exit(45);
    }

    let A = input.read_string().bytes().map(|b| b - b'0').collect_vec();
    let B = input.read_string().bytes().map(|b| b - b'0').collect_vec();

    // dbg!(n, A, B);

    let mut need = 0;
    let mut idxs = (None, None);
    for (i, (&a, &b)) in A.iter().zip(B.iter()).enumerate() {
        if a != b {
            need += 1;
            if idxs.0.is_none() {
                idxs.0 = Some(i);
            } else {
                idxs.1 = Some(i);
            }
        }
    }

    // dbg!(need);
    // dbg!(idxs);

    if need % 2 == 0 {
        if need == 2 {
            let mut a = idxs.0.unwrap();
            let mut b = idxs.1.unwrap();
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }

            // dbg!(a, b);

            if a + 1 == b {
                let diff = max(a, n - 1 - b);
                // dbg!(diff);
                if diff >= 2 {
                    out_line!(min(2 * y, x));
                } else {
                    out_line!(x);
                }
            } else {
                out_line!(y);
            }
        } else {
            out_line!(need / 2 * y);
        }
    } else {
        out_line!(-1);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
