//{"name":"Grid Paths","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1638","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n....\n.*..\n...*\n*...\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GridPaths"}}}

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
    collections::{arr2d::Arr2d, iter_ext::IterExt},
    io::{
        input::Input,
        output::output,
        task_io_settings::{TaskIoSettings, TaskIoType},
        task_runner::run_task,
    },
    numbers::mod_int::ModInt7,
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let grid = (0..n)
        .map(|_| input.read_string().chars().map(|c| c == '.').collect_vec())
        .collect_vec();

    let mut dp = Arr2d::new(n, n, ModInt7::new(0));
    dp[0][0] = ModInt7::new(grid[0][0] as i32);
    for i in 0..n {
        for j in 0..n {
            if !grid[i][j] {
                continue;
            }
            if i > 0 {
                dp[(i, j)] = dp[(i, j)] + dp[(i - 1, j)];
            }
            if j > 0 {
                dp[(i, j)] = dp[(i, j)] + dp[(i, j - 1)];
            }
        }
    }

    out_line!(dp[(n - 1, n - 1)]);
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
