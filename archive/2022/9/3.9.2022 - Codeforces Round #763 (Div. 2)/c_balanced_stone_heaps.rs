//{"name":"C. Balanced Stone Heaps","group":"Codeforces - Codeforces Round #763 (Div. 2)","url":"https://codeforces.com/contest/1623/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n1 2 10 100\n4\n100 100 100 1\n5\n5 1 1 1 8\n6\n1 2 3 4 5 6\n","output":"7\n1\n1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBalancedStoneHeaps"}}}

#![allow(
    unused_imports,
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

use algo_lib::io::{
    input::Input,
    output::output,
    task_io_settings::{TaskIoSettings, TaskIoType},
    task_runner::run_task,
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

use std::cmp::min;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let vec = input.read_vec::<i64>(n);

    let can_do = |target: i64| -> bool {
        let mut carry = (0, 0);
        for (i, stones) in vec.iter().copied().enumerate().rev() {
            let new_stones = stones + carry.0;
            if new_stones < target {
                return false;
            }
            carry.0 = carry.1;
            if i >= 2 {
                let diff = min(new_stones - target, stones) / 3;
                carry.0 += diff;
                carry.1 = diff * 2;
            } else {
                carry.1 = 0;
            }
        }
        true
    };

    // We should always be able to do `low`, and should never be able to do `high`.

    let mut low = vec.iter().copied().min().unwrap();
    let mut high = vec.iter().copied().sum::<i64>() / vec.len() as i64 + 1;

    loop {
        let mid = (low + high) / 2;

        if can_do(mid) {
            if mid == low {
                out_line!(mid);
                return;
            }
            low = mid;
        } else {
            high = mid;
        }
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
}
//END MAIN
