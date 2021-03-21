use std::thread;

// produces an infinite sequence of whole number integers starting from 2.
fn gen_sequence(sender: crossbeam_channel::Sender<usize>) {
    for i in 2.. {
        sender.send(i).expect("failure in gen_sequence");
    }
}

// filters values obtained by the input channel and send the result to the output channel.
fn sieve(
    input_chan: crossbeam_channel::Receiver<usize>,
    output_chan: crossbeam_channel::Sender<usize>,
    prime: usize,
) {
    loop {
        let i = input_chan.recv().unwrap();
        if i % prime != 0 {
            output_chan.send(i).expect("failure in sieve");
        }
    }
}

fn main() {
    let (sender, mut receiver) = crossbeam_channel::unbounded();

    // generates and sends an infinite sequence in separate thread.
    thread::spawn(|| {
        gen_sequence(sender);
    });

    let prime_to_find: usize = 127;

    // store will hold all the prime numbers.
    let mut store: Vec<usize> = Vec::with_capacity(prime_to_find);

    for i in 0..prime_to_find {
        // read from receiver channel
        let prime = receiver.recv().unwrap();

        // add the first prime number to the store
        store.push(prime);

        // make a new send and recieve channel
        let (new_send, new_recv) = crossbeam_channel::unbounded();

        // user builder to name the thread and run the sieve operation in new thread
        thread::Builder::new()
            .name(format!("thread {}", i))
            .spawn(move || {
                sieve(receiver, new_send, prime);
            })
            .unwrap();

        // reassign receiver for next loop iteration; capturing new list of filtered primes up to
        // iteration number.
        receiver = new_recv;
    }

    drop(receiver);

    println!("{:?}", store.pop().unwrap());
}
