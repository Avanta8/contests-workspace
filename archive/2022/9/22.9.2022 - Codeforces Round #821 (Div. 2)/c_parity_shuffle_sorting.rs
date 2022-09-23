//{"name":"C. Parity Shuffle Sorting","group":"Codeforces - Codeforces Round #821 (Div. 2)","url":"https://codeforces.com/contest/1733/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n7 8\n5\n1 1000000000 3 0 5\n1\n0\n","output":"0\n2\n3 4\n1 2\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CParityShuffleSorting"}}}

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
    let n = input.read();
    let arr = input.read_vec::<i64>(n);

    if arr.len() == 1 {
        out_line!(0);
        return;
    }

    let mut swaps = vec![];
    if arr[0] % 2 == 0 {
        let mut value = None;
        for (i, &x) in arr.iter().enumerate().rev() {
            if x % 2 == 0 {
                value = Some(i);
                break;
            }
        }
        let last_even_idx = value.expect("value");

        for (i, &x) in arr.iter().enumerate() {
            if x % 2 == 0 && i != last_even_idx {
                swaps.push((i, last_even_idx));
            }
        }

        for (i, &x) in arr.iter().enumerate() {
            if x % 2 == 1 {
                swaps.push((0, i));
            }
        }
    } else {
        swaps.push((0, n - 1));

        for (i, &x) in arr.iter().enumerate().take(n - 1) {
            if x % 2 == 1 {
                swaps.push((i, n - 1));
            } else {
                swaps.push((0, i));
            }
        }
    }

    assert!(swaps.len() <= n);
    out_line!(swaps.len());
    for s in swaps {
        out_line!(s.0 + 1, s.1 + 1);
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
