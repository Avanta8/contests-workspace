//{"name":"D. Slime Escape","group":"Codeforces - Codeforces Round #822 (Div. 2)","url":"https://codeforces.com/contest/1734/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n7 4\n-1 -2 -3 6 -2 -3 -1\n3 1\n232 -500 -700\n7 4\n-1 -2 -4 6 -2 -4 -1\n8 4\n-100 10 -7 6 -2 -3 6 -10\n8 2\n-999 0 -2 3 4 5 6 7\n7 3\n7 3 3 4 2 1 1\n","output":"YES\nYES\nNO\nYES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSlimeEscape"}}}

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

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k = input.read::<usize>() - 1;

    let arr = input.read_vec::<i64>(n);

    let (mut ben_right, goal_right) = {
        let mut ben = vec![];

        let mut current = 0;
        let mut lowest = 0;
        for i in k + 1..n {
            current += arr[i];
            lowest = min(lowest, current);
            if current > 0 {
                ben.push((lowest, current));
                current = 0;
                lowest = 0;
            }
        }
        ben.reverse();
        (ben, (lowest))
    };
    let (mut ben_left, goal_left) = {
        let mut ben = vec![];

        let mut current = 0;
        let mut lowest = 0;
        for i in (0..k).rev() {
            current += arr[i];
            lowest = min(lowest, current);
            if current > 0 {
                ben.push((lowest, current));
                current = 0;
                lowest = 0;
            }
        }
        ben.reverse();
        (ben, (lowest))
    };

    dbg!(arr);
    dbg!(ben_right, goal_right);
    dbg!(ben_left, goal_left);

    let mut changed = true;
    let mut current = arr[k];
    while changed {
        changed = false;
        if let Some(&(req, diff)) = ben_right.last() {
            if current + req >= 0 {
                current += diff;
                ben_right.pop();
                changed = true;
            }
        } else if current + goal_right >= 0 {
            out_line!(true);
            return;
        }
        if let Some(&(req, diff)) = ben_left.last() {
            if current + req >= 0 {
                current += diff;
                ben_left.pop();
                changed = true;
            }
        } else if current + goal_left >= 0 {
            out_line!(true);
            return;
        }
    }

    out_line!(false);
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
