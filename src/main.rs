use std::io;
use std::cmp;
use rand::Rng;
use rand::seq::SliceRandom;
use std::convert::TryInto;

type Heap = [u32; 3];

enum Turn {
    Player,
    Ai,
}

fn main() {
    let mut heap = init();
    let mut turn = Turn::Player;

    loop {
        if let Turn::Player = turn {
            println!();
        }
        println!("heaps: {:?}", heap);
        take(&mut heap, &turn);

        if exhaused(&heap) {
            println!("{} win", to_str(&turn));
            return;
        }
        turn = match turn {
            Turn::Player => Turn::Ai,
            Turn::Ai => Turn::Player,
        };
    }
}

fn init() -> Heap {
    let mut buffer: Vec<u32> = (1..16).collect();
    loop {
        buffer.shuffle(&mut rand::thread_rng());
        let heap: Heap = buffer[0..3].try_into().expect("slice with incorrect size");
        if xorsum(&heap) != 0 {
            return heap;
        }
    }
}

fn take(heap: &mut Heap, turn: &Turn) {
    let (h, a) = match turn {
        Turn::Player => input(&heap),
        Turn::Ai => ai(&heap),
    };
    heap[h] -= cmp::min(a, heap[h]);
    println!("{} took {} from {}", to_str(&turn), a, h);
}

fn input(heap: &Heap) -> (usize, u32) {
    loop {
        println!("select heap: ");
        let mut h = String::new();
        io::stdin().read_line(&mut h).expect("failed to read");
        let h = h.trim().parse::<usize>().unwrap();
        if h > 2 {
            println!("index out of range");
            continue; 
        }
        if heap[h] == 0 {
            println!("selected heap is empty");
            continue;
        }
        println!("amount you take: ");
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("failed to read");
        let a = a.trim().parse::<u32>().unwrap();
        if a == 0 {
            println!("take at least one");
            continue;
        }
        break (h, a)
    }
}

fn ai(heap: &Heap) -> (usize, u32) {
    if xorsum(&heap) == 0 {
        let h: usize = loop {
            let r = rand::thread_rng().gen_range(0..3);
            if heap[r] != 0 {
                break r
            }
        };
        let a: u32 = if heap[h] == 1 { 1 } else {
            rand::thread_rng().gen_range(1..heap[h])
        };
        (h, a)
    } else {
        let mut bit = xorsum(&heap);
        for i in (0..5).map(|x| 1 << x) {
            bit = bit | (bit >> i);
        }
        let bit = bit ^ (bit >> 1);
        let h = heap.iter().position(|x| x & bit == bit);
        if h == None {
            panic!("can't be");
        }
        let h = h.unwrap();
        let a = heap[h] - (heap[h] ^ xorsum(&heap));
        (h, a)
    }
}

fn xorsum(heap: &Heap) -> u32 {
    heap.iter().fold(0, |acc, i| acc ^ i)
}

fn exhaused(heap: &Heap) -> bool {
    let rest: u32 = heap.iter().sum();
    rest == 0
}

fn to_str(turn: &Turn) -> &str {
    match turn {
        Turn::Player => "you",
        Turn::Ai => "ai",
    }
}