//{"name":"D. Road to Post Office","group":"Codeforces - Educational Codeforces Round 15","url":"https://codeforces.com/contest/702/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5 2 1 4 10\n","output":"14\n"},{"input":"5 2 1 4 5\n","output":"13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRoadToPostOffice"}}}

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
    let (mut d, k, a, b, t): (i64, i64, i64, i64, i64) = input.read();

    if k >= d {
        out_line!(d * a);
        return;
    }

    let mut total = 0;

    total += k * a;
    d -= k;

    if t + k * a >= k * b {
        out_line!(total + d * b);
        return;
    }

    // let mut switch = 0;
    // for r in (0..k).rev() {
    //     if t + r * a >= r * b {
    //         switch = r;
    //         break;
    //     }
    // }

    let mut low = 0; // walk <= car
    let mut high = k; // walk > car

    let switch = loop {
        let mid = (low + high) / 2;

        if low == mid {
            break mid;
        }

        let car = t + mid * a;
        let walk = mid * b;

        if walk <= car {
            low = mid;
        } else {
            high = mid;
        }
    };

    let stops = d / k;
    let car_dist = stops * k;

    let rem = d - car_dist;
    total += stops * t + car_dist * a;

    if rem <= switch {
        total += rem * b;
    } else {
        total += t + a * rem;
    }

    out_line!(total);
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
