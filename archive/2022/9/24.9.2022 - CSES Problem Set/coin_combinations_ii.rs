//{"name":"Coin Combinations II","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1636","interactive":false,"timeLimit":1000,"tests":[{"input":"3 9\n2 3 5\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CoinCombinationsII"}}}

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
    numbers::mod_int::ModInt7,
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, x) = input.read::<(usize, usize)>();
    let arr = input.read_vec::<usize>(n);

    let mut dp = vec![ModInt7::new(0); x + 1];
    dp[0] = ModInt7::new(1);

    for &c in arr.iter() {
        for i in 0..x {
            if dp[i].val() == 0 {
                continue;
            }

            let r = c + i;

            if r <= x {
                dp[r] = dp[r] + dp[i];
            }
        }
    }

    out_line!(dp[x]);
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
