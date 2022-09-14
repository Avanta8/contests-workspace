//{"name":"E1. String Coloring (easy version)","group":"Codeforces - Codeforces Round #617 (Div. 3)","url":"https://codeforces.com/contest/1296/problem/E1","interactive":false,"timeLimit":1000,"tests":[{"input":"9\nabacbecfd\n","output":"YES\n001010101\n"},{"input":"8\naaabbcbb\n","output":"YES\n01011011\n"},{"input":"7\nabcdedc\n","output":"NO\n"},{"input":"5\nabcde\n","output":"YES\n00000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1StringColoringEasyVersion"}}}

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
    collections::{iter_ext::IterExt, vec_ext::ConsecutiveIter},
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
    let n = input.read::<usize>();
    let s = input.read_string().bytes().map(|b| b - b'a').collect_vec();

    let mut removed = vec![false; n];
    let mut last = 0;
    for (i, &c) in s.iter().enumerate() {
        if c < last {
            continue;
        }
        last = c;
        removed[i] = true;
    }

    let rem = s
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, c)| (!removed[i]).then_some(c))
        .collect_vec();

    let able = rem.consecutive_iter().all(|(a, b)| a <= b);
    out_line!(able);
    if able {
        let col = removed
            .iter()
            .map(|&b| if b { '0' } else { '1' })
            .collect::<String>();
        out_line!(col);
    }
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
