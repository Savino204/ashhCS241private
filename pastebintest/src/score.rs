use std::fs;

#[derive(Debug)]
pub struct Score {
    pub name: String,
    pub score: i32,
}

#[derive(Debug)]
pub struct Board {
    pub list: Vec<Score>,
    pub length: usize,
}

impl Board {
    pub fn new() -> Self {
        
        let mut unsorted_list: Vec<Score> = Vec::new();
        let mut length: usize = 0;

        let paths = fs::read_dir("./uploads/").unwrap(); //reads from /uploads
        for item in paths { 

            //take a file, get first line (name) and second line (score)
            let raw_string = fs::read_to_string(item.unwrap().path()).unwrap();
            let mut iter_string = raw_string.lines();
            let name_ref = &iter_string.next().unwrap();
            let name = name_ref.to_string();
            let score: i32 = iter_string.next().unwrap().parse().unwrap();

            length += 1;

            let entry = Score {
                name,
                score,
            };
            unsorted_list.push(entry);
        }
        let list: Vec<Score> = Self::sort_scores(unsorted_list);
        let last = Board {
            list,
            length,
        };
        return last;
    }

    fn sort_scores(mut vec: Vec<Score>) -> Vec<Score> {
        vec.sort_by(|a, b| b.score.cmp(&a.score));
        return vec;
    }

}

/*
fn main() {
    let yay = Board::new();
    for item in yay.list {
        println!("name: {} score: {}", item.name, item.score);
    }
    println!("{}", yay.length);
}
*/