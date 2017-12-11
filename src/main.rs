extern crate time;

use time::PreciseTime;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let mut first_number_tryhard_number: Vec<u32> = Vec::new();
    let shared_first_number_tryhard = Arc::new(Mutex::new(first_number_tryhard_number));

    let mut threads = Vec::new();


    let mut first_number_checked: Vec<u32> = Vec::new();
    let shared_first_number_collect_vec_handle = Arc::new(Mutex::new(first_number_checked)); //TODO sent this to threads

    //TODO make this fill vector untill all first number are finded
    for i in (3..1000) {
        shared_first_number_tryhard.lock().unwrap().push(i);
    }

    //TODO make same number of threads as number of cpus
    for i in (0..9) {
        let shared_first_handle = shared_first_number_tryhard.clone();
        let shared_checked_number_vector  = shared_first_number_collect_vec_handle.clone();
        let handle = thread::spawn( move || {
            println!("Spawned thread nr {}", i);

            loop {
                let mut number;
                {
                  let mut  number_to_check_vector_handle = shared_first_handle.lock().unwrap();
                  number = number_to_check_vector_handle.pop();
                }
                match number {
                    Some(number) => {
                        println!("Checking number {}", number);
                        //TODO send to function to check if its first
                    },
                    None => {
                        break;
                    }
                }
            }
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap(); //w8 for threads to finish
    }


    let mut i = 3;
    let f = 1;

    let mut primes = Vec::new();
    let mut sqrt_of_i = 0.0;
    let mut is_prime = true;

    primes.push(2);
    primes.push(3);

    let start = PreciseTime::now();

    let mut counter = 0;
    while counter < 9999999 {
        i += 2;
        sqrt_of_i = (i as f64).sqrt();
        for item in &primes {
            if item.clone() > sqrt_of_i as i32 {
                break ;
            }
            if ((i % item) == 0) {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(i);
            is_prime = true;
            counter += 1;
        }
        else {
            is_prime = true;
        }

    }

    let end = PreciseTime::now();
    println!("{} seconds for whatever you did", start.to(end));
    println!("Bigest first number {}", primes.last().unwrap());
    println!("{:?}", primes.len());

}
