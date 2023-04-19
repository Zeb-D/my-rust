#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::{ self, BufRead, BufReader };

    fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
        // Open the file in read-only mode.
        let file = File::open(filename).unwrap(); 
        // Read the file line by line, and return an iterator of the lines of the file.
        return io::BufReader::new(file).lines(); 
    }

    #[test]
    fn main() {
        // Stores the iterator of lines of the file in lines variable.
        let lines = read_lines("lorem_ipsum.txt".to_string());
        // Iterate over the lines of the file, and in this case print them.
        for line in lines {
            println!("{}", line.unwrap());
        }
    }

}