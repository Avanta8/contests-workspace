//{"name":"A. Consecutive Sum","group":"Codeforces - Codeforces Round #821 (Div. 2)","url":"https://codeforces.com/contest/1733/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 2\n5 6 0\n1 1\n7\n5 3\n7 0 4 0 4\n4 2\n2 7 3 4\n3 3\n1000000000 1000000000 999999997\n","output":"11\n7\n15\n10\n2999999997\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AConsecutiveSum"}}}

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

use std::cmp::max;

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
    let (n, k) = input.read::<(usize, i64)>();

    let arr = input.read_vec::<i64>(n);

    let mut best = 0;
    for i in 0..n + 1 - k as usize {
        let j = i + k as usize;
        let mut vec = arr.clone();

        for p in i..j {
            for q in p + 1..n {
                if vec[q] > vec[p] && p as i64 % k == q as i64 % k {
                    vec.swap(q, p);
                }
            }
        }

        // dbg!(i, j, vec);

        best = max(best, vec[i..j].iter().copied().sum::<i64>());
    }

    out_line!(best);
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
