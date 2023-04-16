use std::{
    collections::{HashMap},
    time::Instant, sync::Mutex,
};

use threadpool::ThreadPool;
use std::sync::Arc;

use task::{Task, TaskType};

struct Result {
    counter: HashMap<TaskType, usize>,
    res: u64
}

fn main() {
    let (seed, starting_height, max_children) = get_args();

    eprintln!(
        "Using seed {}, starting height {}, max. children {}",
        seed, starting_height, max_children
    );
    let pool = ThreadPool::new();
    let mut count_map = HashMap::new();
    let mut output: u64 = 0;
    let mut res = Arc::new(Mutex::new(Result {counter: count_map, res: output}));
    
    let start = Instant::now();
    for task in Task::generate_initial(seed, starting_height, max_children) {
        let res = res.clone();
        pool.execute(move || {
            let res = res.lock().expect("Locking errors");
            let count_map = res.counter;
            let output = res.res;
            *count_map.entry(task.typ).or_insert(0usize) += 1;
            let result = task.execute();
            output ^= result.0;
        });
    }

    
    let end = Instant::now();

    eprintln!("Completed in {} s", (end - start).as_secs_f64());

    println!(
        "{},{},{},{}",
        output,
        count_map.get(&TaskType::Hash).unwrap_or(&0),
        count_map.get(&TaskType::Derive).unwrap_or(&0),
        count_map.get(&TaskType::Random).unwrap_or(&0)
    );
}

// There should be no need to modify anything below

fn get_args() -> (u64, usize, usize) {
    let mut args = std::env::args().skip(1);
    (
        args.next()
            .map(|a| a.parse().expect("invalid u64 for seed"))
            .unwrap_or_else(|| rand::Rng::gen(&mut rand::thread_rng())),
        args.next()
            .map(|a| a.parse().expect("invalid usize for starting_height"))
            .unwrap_or(5),
        args.next()
            .map(|a| a.parse().expect("invalid u64 for seed"))
            .unwrap_or(5),
    )
}

mod task;
