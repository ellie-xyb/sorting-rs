pub fn sort(data: &mut Vec<u32>) {
    println!("hi");
}

#[cfg(test)]
mod tests {
    use crate::sort;
    fn get_data() -> Vec<u32> {
        let mut data: Vec<u32> = Vec::new();
        for i in 0..1_000_000u32{
            data.push(i);
        }
        data.reverse();
        data
    }
    #[test]
    fn it_works() {
        let mut data = get_data();
        let mut data_copy = data.clone();
        assert_eq!(data, data_copy);

        sort(&mut data);
        data_copy.sort();

        assert_eq!(data, data_copy);
    }
}
