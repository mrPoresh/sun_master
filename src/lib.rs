use std::thread;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {

    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,

}

pub struct Worker {

    id: usize,
    thread: Option<thread::JoinHandle<()>>,

}

trait FnBox {

    fn call_box(self: Box<Self>);

}

impl<F: FnOnce()> FnBox for F {

    fn call_box(self: Box<F>) {

        (*self)()

    }

}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    
    // avalible Job
    NewJob(Job),
    // or kill thread
    Terminate,

}

impl ThreadPool {

    // fn for creating new pool for threads
    pub fn new(size: usize) -> ThreadPool {
        // fn will be panicked if thread counts less than 1
        assert!(size > 0);

        //--------------------------------------------//
        // *** Several manufactures, one consumer *** //
        //--------------------------------------------//

        // making new chanel -> pool is sender
        let (sender, receiver) = mpsc::channel();

        //              *** For safe use a single receiving end: ***         //
        //                                                                   //
        // - Arc allow the worker to clone the pointer to the Mutex, //
        //   which allows the workers to use in turn receiver                //
        //                                                                   //
        let receiver = Arc::new(Mutex::new(receiver));

        // creating Vec with the size capacity
        let mut workers = Vec::with_capacity(size);

        // use counter for adding in Vec our workers amount with id
        for id in 0..size {

            // adding in workers vec new Worker with id and the pointer to the reciver
            workers.push(Worker::new(id, Arc::clone(&receiver)));

        }

        ThreadPool {

            workers,
            sender

        }

    }

    //------------------------------------------------------------------------------------------------//
    //                      Execution method than takes a closure as an argument:                     //
    //                                                                                                //
    // *    Parameter F has a boundary of the Send type, FnOnce() and 'static life cycle where:     * //     
    //                                                                                                //
    // *  - FnOnce() is the trait we need because the threard will execute                            //
    //      the req clousure only once. Im use FnOnce with (), because the FnOnce trait is a clousure //
    //      that takes no parameters and returns no value                                           * //
    // *  - Send is need for pass closure from ona threard to another                               * //
    // *  - 'static necessary because we don't know how long it will take to execute the threard    * //
    //                                                                                                //
    //------------------------------------------------------------------------------------------------//

    pub fn execute<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static 

    {

        //make job with the closure
        let job = Box::new(f);
        //send job
        self.sender.send(Message::NewJob(job)).unwrap();

    }

}

impl Drop for ThreadPool {

    fn drop(&mut self) {

        println!("---* All Workers are deactivated --> job is complite *---");

        for _ in &mut self.workers {

            self.sender.send(Message::Terminate).unwrap();

        }

        for worker in &mut self.workers {

            println!("---* Deactivate Worker id: {} --> job is complite *---", worker.id);

            if let Some(thread) = worker.thread.take() {

                thread.join().unwrap();

            }

        }

    }

}

impl Worker {

    //fn for creating new thread in Worker 
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {

        // creating a new threard
        let thread = thread::spawn(move || {

            loop  {
                // - lock() for mutex: if panic it's mean that some worker can't return lock
                // - recv() for next Jobfrom chanel
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {

                    Message::NewJob(job) => {

                        println!("---* Worker id: {} get a job --> job is running *---", id);

                        job.call_box();

                    },

                    Message::Terminate => {

                        println!("---* Worker id: {} stop job --> job is stop *---", id);

                        break;

                    }
                }

            }

        });

        Worker {

            id,
            thread: Some(thread),

        }

    }

}