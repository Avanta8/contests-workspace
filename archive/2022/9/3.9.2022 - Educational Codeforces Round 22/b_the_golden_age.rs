//{"name":"B. The Golden Age","group":"Codeforces - Educational Codeforces Round 22","url":"https://codeforces.com/contest/813/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"2 3 1 10\n","output":"1\n"},{"input":"3 5 10 22\n","output":"8\n"},{"input":"2 3 3 5\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTheGoldenAge"}}}

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
    collections::vec_ext::ConsecutiveIter,
    io::{
        input::Input,
        output::output,
        task_io_settings::{TaskIoSettings, TaskIoType},
        task_runner::run_task,
    },
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn get_powers(r: u128, x: u128) -> Vec<u128> {
    let mut xs = vec![];
    let mut cx = 1;
    while cx <= r {
        xs.push(cx);
        cx *= x;
    }
    xs
}

fn solve(input: &mut Input, _test_case: usize) {
    let (x, y, l, r) = input.read::<(u128, u128, u128, u128)>();

    let xs = get_powers(r, x);
    let ys = get_powers(r, y);

    let mut nums = vec![];
    for a in xs {
        for &b in ys.iter() {
            nums.push(a + b);
        }
    }
    nums.retain(|&n| n >= l && n <= r);
    nums.sort_unstable();
    nums.dedup();
    nums.insert(0, l - 1);
    nums.push(r + 1);

    let ans = nums
        .consecutive_iter()
        .map(|(&a, &b)| b - a - 1)
        .max()
        .unwrap_or_default();
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
}
//END MAIN
