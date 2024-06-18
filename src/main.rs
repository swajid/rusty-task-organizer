use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let my_file_path = "../files/todo-list.txt";
    let list = File::open(my_file_path)?;
    let buf_reader = BufReader::new(list);
    
    let mut lines = Vec::new();
    let mut started = Vec::new();
    let mut done = Vec::new();
    let mut planned = Vec::new();
    let mut not_started = Vec::new();
    for line in buf_reader.lines() {
        lines.push(line?);
    }
    for line in &lines {
        //println!("{}", line);
        if line.starts_with("s") {
            started.push(&line[2..]);
        }else if line.starts_with("d"){
            done.push(&line[2..]);
        }else if line.starts_with("p"){
            planned.push(&line[2..]);
        }else if line.starts_with("n"){
            not_started.push(&line[2..]);
        }
    }

    //println!("{:?}", planned);
    println!("There are {} planned tasks. \nAll planned tasks are:", planned.len());
    for planned_tasks in &planned {
        println!("- [ ] {}", planned_tasks);
    }

    println!("\nThere are {} done tasks. \nAll done tasks are:", done.len());
    for done_tasks in &done {
        println!("- [x] {}", done_tasks);
    }

    println!("\nThere are {} started tasks. \nAll started tasks are:", started.len());
    for started_tasks in &started {
        println!("- [ ] {}", started_tasks);
    }

    println!("\nThere are {} not started tasks. \nAll not started tasks are:", not_started.len());
    for not_started_tasks in &not_started {
        println!("- [ ] {}", not_started_tasks);
    }

    Ok(())
}
