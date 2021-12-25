use std::io;
use std::cmp;
use rand::Rng;

fn xorsum(heap: [u32; 3]) -> u32 {
    heap.iter().fold(0, |acc, i| acc ^ i)
}

fn exhaused(heap: [u32; 3]) -> bool {
    let rest: u32 = heap.iter().sum();
    rest == 0
}

fn input(heap: [u32; 3]) -> (usize, u32) {
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

fn ai(heap: [u32; 3]) -> (usize, u32) {
    if xorsum(heap) == 0 {
        let h: usize = loop {
            let r = rand::thread_rng().gen_range(0..3);
            if heap[r] != 0 {
                break r
            }
        };
        let a: u32 = rand::thread_rng().gen_range(1..heap[h]);
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

fn init() -> [u32; 3] {
    let mut heap = [0; 3];
    heap[0] = rand::thread_rng().gen_range(1..16);
    heap[1] = rand::thread_rng().gen_range(1..16);
    loop {
        heap[2] = rand::thread_rng().gen_range(1..16);
        if xorsum(heap) != 0 {
            return heap
        }
    }
}

fn main() {
    let mut heap = init();

    loop {
        println!("heaps: {:?}", heap);

        // player
        let (h, a) = input(heap);
        heap[h] -= cmp::min(a, heap[h]);
        if exhaused(heap) {
            println!("player win");
            return;
        }

        // ai
        let (h, a) = ai(heap);
        heap[h] -= cmp::min(a, heap[h]);
        println!("ai took {} from {}", a, h);
        if exhaused(heap) {
            println!("ai win");
            return;
        }
    }
}
