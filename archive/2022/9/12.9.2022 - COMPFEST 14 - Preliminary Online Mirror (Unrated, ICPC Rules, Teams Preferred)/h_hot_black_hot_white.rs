//{"name":"H. Hot Black Hot White","group":"Codeforces - COMPFEST 14 - Preliminary Online Mirror (Unrated, ICPC Rules, Teams Preferred)","url":"https://codeforces.com/contest/1725/problem/H?f0a28=1","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 10 9 14\n","output":"0\n1001\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HHotBlackHotWhite"}}}

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
    let n: usize = input.read();
    let arr = input.read_vec::<u32>(n);

    let mods = arr.iter().map(|&x| x % 3).map(|x| x * x % 3).collect_vec();

    let rem0 = mods
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| (x == 0).then_some(i))
        .collect_vec();
    let rem1 = mods
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| (x != 0).then_some(i))
        .collect_vec();

    let diff = rem0.len().abs_diff(rem1.len()) / 2;
    let (z, mut less, mut more) = if rem0.len() <= rem1.len() {
        (0, rem0, rem1)
    } else {
        (2, rem1, rem0)
    };

    for _ in 0..diff {
        less.push(more.pop().unwrap());
    }

    let mut colour = vec![false; n];
    for x in less {
        colour[x] = true;
    }

    let ans = colour
        .into_iter()
        .map(|x| if x { '1' } else { '0' })
        .collect::<String>();

    out_line!(z);
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
