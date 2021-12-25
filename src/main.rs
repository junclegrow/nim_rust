use std::io;
use std::cmp;

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

fn ai(heap: [u32; 3]) -> [u32; 3] {
    heap
}

fn main() {
    let mut heap = [3, 8, 6];
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
        heap = ai(heap);
        if exhaused(heap) {
            println!("ai win");
            return;
        }
    }
}
