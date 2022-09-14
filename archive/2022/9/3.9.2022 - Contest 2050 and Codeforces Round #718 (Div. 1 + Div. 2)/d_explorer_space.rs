//{"name":"D. Explorer Space","group":"Codeforces - Contest 2050 and Codeforces Round #718 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1517/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3 10\n1 1\n1 1\n1 1\n1 1 1\n1 1 1\n","output":"10 10 10\n10 10 10\n10 10 10\n"},{"input":"2 2 4\n1\n3\n4 2\n","output":"4 4\n10 6\n"},{"input":"2 2 3\n1\n2\n3 4\n","output":"-1 -1\n-1 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DExplorerSpace"}}}

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

use algo_lib::{
    collections::arr2d::{Arr2d, Arr2dRead},
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
    let (n, m, k): (usize, usize, u32) = input.read();

    if k % 2 == 1 {
        out_line!(Arr2d::new(n, m, -1));
        return;
    }

    let mut grid = Arr2d::new(n, m, vec![]);

    let d2 = input.read_table::<i64>(n, m - 1);
    let d1 = input.read_table::<i64>(n - 1, m);

    for i in 0..n {
        for j in 0..m - 1 {
            let u = (i, j);
            let v = (i, j + 1);
            grid[u].push((v, d2[u]));
            grid[v].push((u, d2[u]));
        }
    }
    for i in 0..n - 1 {
        for j in 0..m {
            let u = (i, j);
            let v = (i + 1, j);
            grid[u].push((v, d1[u]));
            grid[v].push((u, d1[u]));
        }
    }

    let mut arr = Arr2d::generate(n, m, |i, j| {
        grid[(i, j)]
            .iter()
            .map(|&(_neigh, cost)| cost)
            .min()
            .unwrap()
    });

    for _ in 1..k / 2 {
        let mut new_arr = arr.clone();
        for i in 0..n {
            for j in 0..m {
                new_arr[(i, j)] = grid[(i, j)]
                    .iter()
                    .map(|&(neigh, cost)| arr[neigh] + cost)
                    .min()
                    .unwrap();
            }
        }
        arr = new_arr;
    }

    for i in 0..n {
        for j in 0..m {
            arr[(i, j)] *= 2;
        }
    }

    out_line!(arr);
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
