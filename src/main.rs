// use rand::seq::SliceRandom;
use rand::Rng;
use std::fs;

fn main() {
    let paths = fs::read_dir("./data").unwrap();
    let mut rng = rand::thread_rng();

    let vec_paths: Vec<_> = paths.map(|x| x).collect();
    let mut count = 0;

    let rand_file_index = rng.gen_range(0..vec_paths.len());


    for path in vec_paths {

        if count == rand_file_index {
            let contents = fs::read_to_string(&path.unwrap().path())
                .expect("Should have been able to read the file");

            let parts: Vec<_> = contents.split("%").skip_while(|&x| x.is_empty()).collect();
            let mut rng = rand::thread_rng();

            println!("{}", parts[rng.gen_range(0..parts.len())])
        }
        count += 1;

    }
}
