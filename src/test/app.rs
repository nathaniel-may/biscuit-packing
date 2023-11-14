use crate::{app, cli, error::Error};

#[test]

fn simple_single_run() {
    let input = cli::Args {
        pan_width: 100.0,
        pan_length: 200.0,
        runs: 1000,
        command: cli::Commands::Single { biscuits: 17 },
    };

    let app = app::parse(input).unwrap();
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

#[test]
fn biscuits_below_one_single() {
    let input = cli::Args {
        pan_width: 100.0,
        pan_length: 200.0,
        runs: 1000,
        command: cli::Commands::Single { biscuits: 0 },
    };

    let app = app::parse(input);
    match app {
        Err(Error::BiscuitsBelowOne) => (),
        Err(x) => panic!("expected different error, got {x}"),
        Ok(x) => panic!("expected error, got {x:?}"),
    }
}

#[test]
fn biscuits_below_one_multi() {
    let input = cli::Args {
        pan_width: 100.0,
        pan_length: 200.0,
        runs: 1000,
        command: cli::Commands::Multi { start: 0, end: 1 },
    };

    let app = app::parse(input);
    match app {
        Err(Error::BiscuitsBelowOne) => (),
        Err(x) => panic!("expected different error, got {x}"),
        Ok(x) => panic!("expected error, got {x:?}"),
    }
}

#[test]
fn biscuits_start_above_end() {
    let input = cli::Args {
        pan_width: 100.0,
        pan_length: 200.0,
        runs: 1000,
        command: cli::Commands::Multi { start: 2, end: 1 },
    };

    let app = app::parse(input);
    match app {
        Err(Error::StartGreaterThanEnd) => (),
        Err(x) => panic!("expected different error, got {x}"),
        Ok(x) => panic!("expected error, got {x:?}"),
    }
}
