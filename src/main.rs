use std::io;
use std::time::Instant; 

fn main() {
    println!("Enter collatz number"); 
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Could not read line");
    let num: u32 = num.trim().parse().expect("Please enter an integer");

    let now = Instant::now(); 
    let threaded = threaded_collatz(num); 
    let elapsed = now.elapsed();
    println!("Threaded elapsed: {:.2?}", elapsed); 
    
    let now = Instant::now(); 
    let normal = verify_threaded(num); 
    let elapsed = now.elapsed();
    println!("Normal elapsed: {:.2?}", elapsed); 

    // println!("Threaded\tNormal"); 
    // for (it1, it2) in threaded.iter().zip(normal.iter()) {
    //     println!("{}\t{}", it1, it2); 
    // }
    for (it1, it2) in threaded.iter().zip(normal.iter()) {
        assert_eq!(it1, it2); 
    }
}

fn collatz(num: &mut u32) -> u32 {
    if *num == 0 {
        return 0
    }
    let mut count = 0;
    while *num != 1 {
        count += 1;
        if *num % 2 == 0 {
            *num /= 2;
        } else {
            *num = 3 * (*num) + 1;
        }
    }
    count
}

fn section_collatz(start: u32, end: u32) -> Vec<u32> {
    let mut ret = Vec::new(); 

    for mut i in start..end {
        ret.push(collatz(&mut i)); 
    }

    ret
}

const NUM_THREADS: u32 = 128; 
fn threaded_collatz(num: u32) -> Vec<u32> {
    
    // let pool = rayon::ThreadPoolBuilder::new().num_threads(NUM_THREADS).build().unwrap(); 
    // let n = pool.install()
    let mut threads = Vec::new(); 
    for i in 0..NUM_THREADS {
        let t = std::thread::spawn(move || 
            section_collatz(i * num / NUM_THREADS, (i+1) * num / NUM_THREADS)); 
        threads.push(t); 
        if i == NUM_THREADS - 1 && (i+1) * num / NUM_THREADS != num {
           let t = std::thread::spawn(move ||
                section_collatz((i+1) * num / NUM_THREADS, num)); 
            threads.push(t); 
        }
    }
        
    let mut ans = Vec::new(); 
    for t in threads {
        let mut tmp = t.join().unwrap(); 
        ans.append(&mut tmp); 
    }

    ans
}

fn verify_threaded(num: u32) -> Vec<u32> {
    let mut ans = Vec::new(); 

    for mut i in 0..num {
        ans.push(collatz(&mut i)); 
    }
    ans
}