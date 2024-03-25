fn main() {
    let txt = std::fs::read_to_string("1min.txt").unwrap();

    let mut counter = 0;
    let mut download: u64 = 0;
    let mut upload: u64 = 0;
    let mut both: u64 = 0;
    for line in txt.lines() {
        counter += 1;
        let mut words = line.split_whitespace();

        let date = words.next().unwrap();
        let time = words.next().unwrap();
        download += words.next().unwrap().parse::<u64>().unwrap();
        upload += words.next().unwrap().parse::<u64>().unwrap();
        both += words.next().unwrap().parse::<u64>().unwrap();
        if counter == 5 {
            println!("{date} {time} {download} {upload} {both}");
            counter = 0;
            download = 0;
            upload = 0;
            both = 0;
        }
    }
    println!("\nLast: {download} {upload} {both}");
}
