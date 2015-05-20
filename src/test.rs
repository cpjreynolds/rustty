#[derive(Debug, Clone, Copy)]
struct Merp(i32);

impl Merp {
    fn default() -> Merp {
        Merp(1)
    }
}

fn main() {
    let x = vec![vec![Merp::default(); 5]; 5];
    println!("{:?}", x);
    println!("{:?}", x[1][1]);
}