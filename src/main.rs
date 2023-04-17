use std::{
    collections::{HashMap},
    time::Instant,
    sync::mpsc::channel,
    sync::mpsc::Sender,
    sync::mpsc::Receiver,
};

use threadpool::ThreadPool;

use task::{Task, TaskType, TaskResult};

fn main() {
    let (seed, starting_height, max_children) = get_args();

    eprintln!(
        "Using seed {}, starting height {}, max. children {}",
        seed, starting_height, max_children
    );
    let pool = ThreadPool::new(8);
    let mut count_map: HashMap<TaskType, usize> = HashMap::new();
    let mut output: u64 = 0;
    let (tx, rx): (Sender<TaskResult>, Receiver<TaskResult>) = channel();
    let mut counter: u64 = 0;
    
    let start = Instant::now();
    for task in Task::generate_initial(seed, starting_height, max_children) {
        counter += 1;
        let tx = tx.clone();
        *count_map.entry(task.typ).or_insert(0usize) += 1;
        pool.execute(move || {
            let result = task.execute();
            tx.send(result).unwrap();
        });
    }

    while counter != 0 {
        let result = rx.recv().unwrap();
        counter -= 1;
        output ^= result.0;
        for task in result.1 {
            counter += 1;
            let tx = tx.clone();
            *count_map.entry(task.typ).or_insert(0usize) += 1;
            pool.execute(move || {
                let result = task.execute();
                tx.send(result).unwrap();
            });
        }
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