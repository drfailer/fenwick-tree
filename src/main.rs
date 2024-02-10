mod fenwick;

use fenwick::Fenwick;

fn print_arr(arr: &Vec<i32>) {
        print!("[ ");
        for elt in arr {
            print!("{} ", elt);
        }
        println!("]");
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7];
    let f = Fenwick::new(&arr);

    f.print_bits();
    print_arr(&arr);
    f.print();
    println!("sum(0, 0): {}", f.query(0));
    println!("sum(0, 1): {}", f.query(1));
    println!("sum(0, 2): {}", f.query(2));
    println!("sum(0, 3): {}", f.query(3));
    println!("sum(3, 4): {}", f.query_range(3, 4));
}
