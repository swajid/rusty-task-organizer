# rusty-task-organizer
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)

A rust task organizer for the terminal
                _               _            _                                     _              
 _ __ _   _ ___| |_ _   _      | |_ __ _ ___| | __      ___  _ __ __ _  __ _ _ __ (_)_______ _ __ 
| '__| | | / __| __| | | |_____| __/ _` / __| |/ /____ / _ \| '__/ _` |/ _` | '_ \| |_  / _ \ '__|
| |  | |_| \__ \ |_| |_| |_____| || (_| \__ \   <_____| (_) | | | (_| | (_| | | | | |/ /  __/ |   
|_|   \__,_|___/\__|\__, |      \__\__,_|___/_|\_\     \___/|_|  \__, |\__,_|_| |_|_/___\___|_|   
                    |___/                                        |___/                           


There is a certain satisfaction in seeing your completed tasks displayed in the terminal. In order to achieve this, first create a text file with your todo list with a new task on each new line. Then, add the following letters to the beginning of each task signifying their status, followed by a space:


```
s = started
d = done
p = planned
n = not started
```

To run: `cargo run`

# Example Input (.txt file)
````
p workout today
p call office about plates
n go out and get some coffee
d eat breakfast
d eat lunch
n go out for dinner
p sleep early
```

# Example Output
```
There are 3 planned tasks. 
All planned tasks are:
- [ ] workout today
- [ ] call office about plates
- [ ] sleep early

There are 2 done tasks. 
All done tasks are:
- [x] eat breakfast
- [x] eat lunch

There are 0 started tasks. 
All started tasks are:

There are 2 not started tasks. 
All not started tasks are:
- [ ] go out and get some coffee
- [ ] go out for dinner

```

# Future Updates
- [ ] Read in tasks file as a csv in Polars
- [ ] Unit tests
- [ ] Some graphics why not
