use rand::prelude::*;
use std::{
    fs,
    io::{self, Read, Result, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
};

type Task = Box<dyn FnOnce() + Send>;

struct ThreadPool {
    tx: Sender<Task>,
}

impl ThreadPool {
    fn new(threads: usize) -> Self {
        let (tx, rx) = channel();

        let rx = Arc::new(Mutex::new(rx));

        for _ in 0..threads {
            let worker = Worker::new(Arc::clone(&rx));
            std::thread::spawn(|| worker.run());
        }

        ThreadPool { tx }
    }
    fn execute<T>(&mut self, task: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let task = Box::new(task);

        self.tx.send(task).unwrap();
    }
}

struct Worker {
    rx: Arc<Mutex<Receiver<Task>>>,
}

impl Worker {
    fn new(rx: Arc<Mutex<Receiver<Task>>>) -> Self {
        Worker { rx }
    }
    fn run(self) {
        while let Ok(task) = self.rx.lock().unwrap().recv() {
            (task)();
        }
    }
}

fn main() {
    // bind listener to socket
    let listener = TcpListener::bind("0.0.0.0:80").expect("Can't bind to socket");

    // create thread pool
    let mut thread_pool = ThreadPool::new(12);

    // wait for connection and handle it
    for connection in listener.incoming() {
        let result = match connection {
            Ok(connection) => thread_pool.execute(|| {
                let result = handle_connection(connection);

                match result {
                    Err(err) => println!("Error during handling request: {err}"),
                    Ok(_) => {}
                }
            }),
            Err(err) => {
                println!("Error on incoming connection: {err}");
                continue;
            }
        };
    }
}

fn handle_connection(mut connection: TcpStream) -> Result<()> {
    let mut rng = rand::rng();

    // random delay for test
    //std::thread::sleep(std::time::Duration::from_millis(rng.random_range(10..=30)));

    // simulate cpu work
    for _ in 0..1_000_000 {
        let val = 5;
        std::hint::black_box(val);
    }

    // io work
    let file_content = fs::read(format!("files/{}", rng.random_range(0..300)))?;

    // specify the buffer for header
    let mut header = [0u8; 8];

    // read the header - how many bytes are going to be sent
    connection.read_exact(&mut header)?;

    // turn 8 bytes into usize
    let len = usize::from_le_bytes(header);

    // specify the buffer for message
    let mut buffer = vec![0; len];

    // read the message
    connection.read_exact(&mut buffer[0..len])?;

    // anti-optimization
    std::hint::black_box(&mut buffer);

    //send back some file
    let len = file_content.len();
    header = len.to_le_bytes();

    connection.write_all(&header)?;
    connection.write_all(&file_content)?;

    Ok(())
}
