use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::thread::{JoinHandle};
use std::time::Duration;


type TaskQueue = Mutex<Option<VecDeque<Box<dyn Fn()>>>>;

// varialbes to controll the thread
static mut TASK_QUEUE: TaskQueue = Mutex::new(None);
static mut EXECUTOR: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);

static mut THREAD_STOP: AtomicBool = AtomicBool::new(false);
static mut THREAD_STOPPED: Mutex<Option<Receiver<()>>> = Mutex::new(None);

// setup the thread to
pub fn setup() {
    unsafe {
        let (tx, rx) = mpsc::channel();
        let mut thread_stop_channel = THREAD_STOPPED.lock().expect("failed to lock");
        *thread_stop_channel = Some(rx);


        let mut tk = TASK_QUEUE.lock().expect("could not lock");
        *tk = Some(VecDeque::new());
        let mut executor = EXECUTOR.lock().expect("Failed to lock");
        // create a always running thread which checks if there are tasks left to execute
        *executor = Some(thread::spawn(move || loop {
            // check if there are any task to be executed if not pause the thread
            // and check if we need to stop
            if TASK_QUEUE.get_mut().unwrap().as_ref().unwrap().is_empty() {
                // check if we should stop
                if *THREAD_STOP.get_mut() {
                    println!("Terminating");
                    tx.send(()).expect("failed to send");
                    return;
                }

                thread::park();
                continue;
            }

            // execute the task
            TASK_QUEUE.get_mut().unwrap().as_mut().unwrap().pop_front().unwrap()();
        }));
    }
}


pub fn add_task<T: 'static>(task: Box<dyn Fn() -> T>) -> Result<Receiver<T>, &'static str> {
    unsafe {
        if *THREAD_STOP.get_mut() {
            return Err("thread is already shut down")
        }

        let (tx, rx) = mpsc::channel();

        TASK_QUEUE.get_mut().unwrap().as_mut().unwrap().push_back(Box::new(move || {
            let _ = tx.send(task());
        }));

        EXECUTOR.get_mut().unwrap().as_mut().unwrap().thread().unpark();

        Ok(rx)
    }
}



pub fn teardown() {
    // stop the thread when the queue is empty
    unsafe {
        *THREAD_STOP.get_mut() = true;
        EXECUTOR.get_mut().unwrap().as_mut().unwrap().thread().unpark();

        THREAD_STOPPED.get_mut().unwrap().as_mut().unwrap().recv_timeout(Duration::from_secs(1)).expect("failed to detect stop of thread")
    }
}
