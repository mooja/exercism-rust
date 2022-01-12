extern crate crossbeam;

use std::collections::HashMap;

use crossbeam::channel::unbounded;

type Tally = HashMap<char, usize>;

enum Task <'a> {
    Str(&'a str),
    End
}

enum TaskResult {
    Count(Tally),
    Finished
}

fn tally(s: &str) -> Tally {
    let mut rv = Tally::new();
    for ch in s.chars() {
        let lc_chars = ch.to_lowercase();
        for ch in lc_chars {
            if ch.is_alphabetic() {
                let e = rv.entry(ch).or_insert(0);
                *e += 1;
            }
        }
    }
    rv
}

pub fn frequency<'a> (input: &'a [&'a str], worker_count: usize) -> HashMap<char, usize> {
    let (queue_s, queue_r) = unbounded::<Task>();
    let (results_s, results_r) = unbounded::<TaskResult>();
    let mut rv = Tally::new();
    crossbeam::scope(|scope| {
        for _ in 0..worker_count {
            let queue_r = queue_r.clone();
            let results_s = results_s.clone();
            scope.spawn(move |_| {
                loop {
                    match queue_r.recv().unwrap() {
                        Task::Str(ref s) => {
                            results_s.send(TaskResult::Count(tally(s))).unwrap();
                        }

                        Task::End => {
                            results_s.send(TaskResult::Finished).unwrap();
                            break;
                        }
                    }
                }
            });
        }

        for s in input {
            queue_s.send(Task::Str(*s)).unwrap();
        }

        for _ in 0..worker_count {
            queue_s.send(Task::End).unwrap();
        }

        let mut finished = 0;
        while finished < worker_count {
            let result = results_r.recv().unwrap();
            match result {
                TaskResult::Count(cnt) => {
                    for (k, v) in cnt {
                        let e = rv.entry(k).or_insert(0);
                        *e += v;
                    }
                }
                TaskResult::Finished => finished += 1
            }
        }
    }).unwrap();
 
    rv
}
