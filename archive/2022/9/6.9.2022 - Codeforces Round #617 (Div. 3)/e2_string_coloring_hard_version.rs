//{"name":"E2. String Coloring (hard version)","group":"Codeforces - Codeforces Round #617 (Div. 3)","url":"https://codeforces.com/contest/1296/problem/E2","interactive":false,"timeLimit":1000,"tests":[{"input":"9\nabacbecfd\n","output":"2\n1 1 2 1 2 1 2 1 2\n"},{"input":"8\naaabbcbb\n","output":"2\n1 2 1 2 1 2 1 1\n"},{"input":"7\nabcdedc\n","output":"3\n1 1 1 1 1 2 3\n"},{"input":"5\nabcde\n","output":"1\n1 1 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2StringColoringHardVersion"}}}

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
    let n = input.read::<usize>();
    let s = input.read_string().bytes().map(|b| b - b'a').collect_vec();

    let mut sp = s.iter().copied().enumerate().collect_vec();

    let mut ans = vec![0; n];

    let mut count = 0;
    while !sp.is_empty() {
        count += 1;
        let mut removed = vec![false; sp.len()];
        let mut last = 0;
        for (idx, &(i, c)) in sp.iter().enumerate() {
            if c < last {
                continue;
            }
            last = c;
            ans[i] = count;
            removed[idx] = true;
        }
        sp = sp
            .into_iter()
            .enumerate()
            .filter_map(|(idx, e)| (!removed[idx]).then_some(e))
            .collect();
    }

    out_line!(count);
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
