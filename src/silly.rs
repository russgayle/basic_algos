pub fn print_int(a: i32) {
    println!("This is a: {}", a);
}

#[cfg(test)]
mod tests {
    use super::print_int;
    use test::Bencher;

    #[test]
    fn just_prints_unit() {
        print_int(5);
        assert!(true);
    }

    #[test]
    fn do_nothing_unit() {
        assert!(true);
    }

    #[bench]
    fn bench_nothing_useful(b: &mut Bencher) {
        b.iter(|| print_int(1));
    }
}