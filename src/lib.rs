pub fn sort(data: &mut Vec<u32>) {
    // let mut tmp = vec![0u32; data.len()];
    let mut tmp: Vec<Vec<u32>> = Vec::new();

    tmp.resize_with(256, || Vec::new());
    // for _ in 0..256 {
    //     tmp.push(Vec::new());
    // }

    for byte in 0..4 {
        for value in data.iter() {
            let b = value.to_le_bytes()[byte];

            tmp[b as usize].push(*value);
        }

        *data = tmp.iter().flatten().map(|v| *v).collect();

        for b in 0..256 {
            tmp[b].clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort;
    use std::time::Instant;

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

        let sort_time_1 = Instant::now();
        sort(&mut data);
        println!("sort time 1 : {}", sort_time_1.elapsed().as_micros());
        
        let sort_time_2 = Instant::now();
        data_copy.sort();
        println!("sort time 2 : {}", sort_time_2.elapsed().as_micros());

        assert_eq!(data, data_copy);
    }
}
