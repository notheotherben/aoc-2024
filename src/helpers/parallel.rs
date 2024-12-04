use std::{sync::mpsc, thread, collections::VecDeque};

pub fn map_reduce<I, M, N, R, O>(input: I, mapper: M, initial: O, reducer: R) -> O
    where M: Fn(I::Item) -> N + Send + Clone + 'static,
          R: Fn(O, N) -> O + 'static,
          I: Iterator + Send,
          I::Item: Send + 'static,
          N: Send + 'static,
          O: Send,
{
    let (tx, rx) = mpsc::channel();

    const PARALLELISM: usize = 8;

    let mut work_queues = VecDeque::with_capacity(PARALLELISM);
    let mut threads = Vec::with_capacity(PARALLELISM);

    for _ in 0..PARALLELISM {
        let (wtx, wrx) = mpsc::channel();
        work_queues.push_back(wtx);

        let tx = tx.clone();
        let mapper = mapper.clone();
        threads.push(thread::spawn(move || {
            for work in wrx.iter() {
                tx.send(mapper(work)).unwrap();
            }
        }));
    }

    for work in input {
        let queue = work_queues.pop_front().unwrap();
        queue.send(work).unwrap();
        work_queues.push_back(queue);
    }

    drop(work_queues);
    drop(tx);

    threads.into_iter().for_each(|t| t.join().unwrap());

    rx.iter().fold(initial, reducer)
}
