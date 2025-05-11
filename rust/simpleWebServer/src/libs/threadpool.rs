use std::thread;
use crate::route::routemanager;
    
struct Worker<'rm> {
    id : usize,
    thread : thread::JoinHandle<()>,
    manager : &'rm routemanager::RouteManager
}

impl<'rm> Worker<'rm> {
    pub fn new(id : usize, rm : &routemanager::RouteManager) -> Worker {
        let thread = std::thread::spawn(|| {});
        Worker { id: id, thread: thread, manager: rm }
    }
}

pub struct ThreadPool<'rm> {
    workers : Vec<Worker<'rm>>
}

impl<'rm> ThreadPool<'rm> {
    pub fn new(poolsize : usize, rm : &routemanager::RouteManager) -> ThreadPool{
        if poolsize < 1 {
            panic!("Can't have less than 1 thread!");
        }

        let mut workers = Vec::with_capacity(poolsize);

        for i in 0..poolsize {
            workers.push(Worker::new(i, rm));
        }
        ThreadPool { workers }
    }
}