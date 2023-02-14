// rustc impl_trait_as_arg_type.rs && ./impl_trait_as_arg_type 

fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                    .collect() // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}

fn parse_csv_document2(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                    .collect() // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}

fn main() {
    let csv = "test_csv.csv";
    let file = std::fs::File::open(csv).unwrap();
    let mut reader = std::io::BufReader::new(file);
    
    let parsed = parse_csv_document(&mut reader).unwrap();
    println!("{:?}", parsed);
    
    let file = std::fs::File::open(csv).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let parsed2 = parse_csv_document2(&mut reader).unwrap();
    println!("{:?}", parsed2);
}

