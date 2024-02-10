pub struct Fenwick {
    bits: Vec<i32>,
    size: usize,
}

// 1 => 001 =>         0 + 2⁰ (0,0) : on part de 0 et on en prend 1
// 2 => 010 =>         0 + 2¹ (0,1) : on part de 0 et on en prend 2¹
// 3 => 011 =>        2¹ + 2⁰ (2,2) : on part de 2¹ et on en prend 2⁰
// 4 => 100 =>         0 + 2² (0,3) : on part de 0 et on en prend 2²
// 5 => 101 =>        2² + 2⁰ (4,4) : on part de 2² et on en prend 2⁰
// 6 => 110 =>        2² + 2¹ (4,5) : on part de 2² et on en prend 2¹
// 7 => 111 => (2² + 2¹) + 2⁰ (6,6) : on part de (2² + 2¹) et on en prend 2⁰
// ...

fn init(arr: &Vec<i32>) -> Vec<i32> {
    let size = arr.len() + 1;
    let mut result = vec![0; size];

    for idx in 1..size {
        let idx_i32 = idx as i32;
        let count = idx_i32 & -idx_i32;
        let begin = idx_i32 & !count;

        for i in begin..(begin + count) {
            result[idx] += arr[i as usize];
        }
    }
    return result;
}

#[allow(dead_code, unused)]
impl Fenwick {
    pub fn new(arr: &Vec<i32>) -> Self {
        Self {
            bits: init(arr),
            size: arr.len() + 1,
        }
    }

    pub fn query(&self, idx: usize) -> i32 {
        let mut result = 0;
        let mut i = idx as i32 + 1;

        while i > 0 {
            result += self.bits[i as usize];
            i -= i & -i;
        }
        result
    }

    pub fn query_range(&self, idx1: usize, idx2: usize) -> i32 {
        self.query(idx2) - self.query(idx1 - 1)
    }

    pub fn update(&mut self, idx: usize, delta: i32) {
        let mut i = idx as i32 + 1;

        while i > 0 {
            self.bits[i as usize] += delta;
            i -= i & -i;
        }
    }

    pub fn print_bits(&self) {
        print!("[ ");
        for elt in &self.bits {
            print!("{} ", elt);
        }
        println!("]");
    }

    pub fn print(&self) {
        println!("0");
        self.print_aux(0, 1);
    }

    fn print_aux(&self, idx: usize, lvl: usize) {
        let mut i = 1;

        while (i | idx) < self.size && ((i | idx) != idx)  {
            for j in 0..lvl {
                print!("   ");
            }
            println!("{}", self.bits[idx | i]);
            self.print_aux(idx | i, lvl + 1);
            i <<= 1;
        }
    }
}
