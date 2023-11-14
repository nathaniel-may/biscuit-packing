use crate::{app, cli};

#[test]

fn simple_single_run() {
    let input = cli::Args {
        pan_width: 100.0,
        pan_length: 200.0,
        runs: 1000,
        command: cli::Commands::Single { biscuits: 17 },
    };

    let app = app::parse(input);
    // parsing scales by 10. todo move scaling logic? make scaling smarter? remove scaling?
    let expected_runs = vec![app::Run {
        biscuits: 17,
        width: 1000.0,
        length: 2000.0,
        iters: 1000,
        announce_end: None,
    }];

    assert_eq!(app.runs, expected_runs);
}
