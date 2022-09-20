// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let vec0 = Vec::new(); // in the scope of main

    let mut vec1 = fill_vec(vec0); // vec1 owns vec0 now

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1); // vec1 is moved into the println! macro

    vec1.push(88); // vec1 is borrowed again

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1); // vec1 is moved into the println! macro again
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
