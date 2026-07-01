struct FibonacciIter {
    curr: u64,
    next: u64,
}

impl Iterator for FibonacciIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.curr;
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(result)
    }
}

fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter_mut() {
        *number += 1;
        println!("{}", number);
    }

    println!("===");

    let fib_iter = FibonacciIter { curr: 0, next: 1 };
    for number in fib_iter.take(10) {
        println!("{}", number);
    }
}
