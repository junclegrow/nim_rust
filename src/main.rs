use std::io;

fn valid_cmd(cmd: &Vec<&str>) -> bool {
    if cmd[0] < "a" || "c" < cmd[0] { return false }
    if cmd[1] < "1" { return false }
    true
}

fn main() {
    let mut heap = [3, 8, 6];

    loop {
        println!("\nHeaps");
        println!("a:{}  b:{}  c:{}", heap[0], heap[1], heap[2]);

        println!("Your turn\nformat:alphabet amount");
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("failed to read");
        let cmd: Vec<&str> = cmd.trim().split(' ').collect();
        if !valid_cmd(&cmd) { continue; }

        println!("{:?}", &cmd);
    }
}
