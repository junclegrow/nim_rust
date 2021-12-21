use std::io;
use std::cmp;

fn valid_cmd(cmd: &Vec<&str>) -> bool {
    if cmd[0] < "a" || "c" < cmd[0] { return false }
    if cmd[1] < "1" { return false }
    true
}

fn xorsum(heap: &Vec<i32>) -> i32 {
    heap.iter().fold(0, |acc, i| acc ^ i)
}

fn main() {
    let mut heap = [3, 8, 6];

    loop {
        // print
        println!("\nHeaps");
        println!("a:{}  b:{}  c:{}", heap[0], heap[1], heap[2]);

        // input
        println!("Your turn\nformat:alphabet amount");
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("failed to read");
        let cmd: Vec<&str> = cmd.trim().split(' ').collect();
        if !valid_cmd(&cmd) { continue; }

        // take
        let c = cmd[0].chars().nth(0).unwrap();
        let amount = cmp::min(heap[c as usize - 'a' as usize], cmd[1].parse().unwrap());
        heap[c as usize - 'a' as usize] -= amount;

        // check
        let rest: i32 = heap.iter().sum();
        if rest == 0 {
            println!("fin");
            return;
        }
    }
}
