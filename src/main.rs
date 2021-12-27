use rand::seq::SliceRandom;
use rand::Rng;
use std::cmp;
use std::convert::TryInto;
use std::io;

type Heaps = [u32; 3];

enum State {
    PlayerChooseHeap,
    PlayerDecideAmount(usize),
    Take(usize, u32, Turn),
    Judge(Turn),
    Ai,
}

enum Turn {
    Player,
    Ai,
}

impl Turn {
    fn to_string(&self) -> &str {
        match self {
            Turn::Player => "you",
            Turn::Ai => "ai",
        }
    }
}

fn main() {
    let mut heaps = init();
    let mut state = State::PlayerChooseHeap;

    loop {
        state = match state {
            State::PlayerChooseHeap => {
                println!("\nheaps: {:?}", heaps);
                println!("choose one heap");

                let heap_index = choose_heap();
                if let None = heap_index { continue; }
                let heap_index = heap_index.unwrap();

                if heaps[heap_index] == 0 {
                    println!("selected heap is empty");
                    continue;
                }
                State::PlayerDecideAmount(heap_index)
            },
            State::PlayerDecideAmount(heap_index) => {
                println!("decide amount you take");
                
                let amount = decide_amount();
                if let None = amount { continue; }
                let amount = amount.unwrap();

                State::Take(heap_index, amount, Turn::Player)
            },
            State::Take(heap_index, amount, turn) => {
                heaps[heap_index] -= cmp::min(amount, heaps[heap_index]);
                println!("{} took {} from {}", turn.to_string(), amount, heap_index);

                State::Judge(turn)
            },
            State::Judge(turn) => {
                let rest: u32 = heaps.iter().sum();

                if rest == 0 {
                    println!("{} win", turn.to_string());
                    return;
                }
                
                match turn {
                    Turn::Player => State::Ai,
                    Turn::Ai => State::PlayerChooseHeap,
                }
            },
            State::Ai => {
                let (heap_index, amount) = ai(heaps);

                State::Take(heap_index, amount, Turn::Ai)
            },
        }
    }
}

fn init() -> Heaps {
    let mut buffer: Vec<u32> = (1..16).collect();
    loop {
        buffer.shuffle(&mut rand::thread_rng());
        let heap: Heaps = buffer[0..3].try_into().expect("slice with incorrect size");
        if xorsum(heap) != 0 {
            return heap
        }
    }
}

fn choose_heap() -> Option<usize> {
    let mut heap_index = String::new();
    io::stdin().read_line(&mut heap_index).unwrap();
    let heap_index = heap_index.trim().parse::<usize>().unwrap();
    if heap_index > 2 {
        println!("index out of range");
        return None
    }
    Some(heap_index)
}

fn decide_amount() -> Option<u32> {
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).unwrap();
    let amount = amount.trim().parse::<u32>().unwrap();
    
    if amount == 0 {
        println!("take at least one");
        return None
    }
    Some(amount)
}

fn ai(heap: Heaps) -> (usize, u32) {
    if xorsum(heap) == 0 {
        let h: usize = loop {
            let r = rand::thread_rng().gen_range(0..3);
            if heap[r] != 0 {
                break r;
            }
        };
        let a: u32 = if heap[h] == 1 {
            1
        } else {
            rand::thread_rng().gen_range(1..heap[h])
        };
        (h, a)
    } else {
        let mut bit = xorsum(heap);
        for i in (0..5).map(|x| 1 << x) {
            bit = bit | (bit >> i);
        }
        let bit = bit ^ (bit >> 1);
        let h = heap.iter().position(|x| x & bit == bit);
        if h == None {
            panic!("can't be");
        }
        let h = h.unwrap();
        let a = heap[h] - (heap[h] ^ xorsum(heap));
        (h, a)
    }
}

fn xorsum(heaps: Heaps) -> u32 {
    heaps.iter().fold(0, |acc, i| acc ^ i)
}