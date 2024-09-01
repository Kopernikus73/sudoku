# Sudoku - Generator

---
## How to use

1. Generate a sudoku with standard difficulty
    
    `$ ./sudoku-generator`
2. Generate a sudoku with changed difficulty [0..10]

   `$ ./sudoku-generator -d 3` (The difficulty is only relative because it is based on randomness)
3. Generate more sudokus at a time [1..10]
   
   `$ ./sudoku-generator -a 3`
4. The generated sudokus will be in the ***./sudoku/*** directory 
    
    There will be a completed version at the top and a version with missing numbers, based on the difficulty, on the bottom
    1. Remove the solved sudoku
   
       `$ ./sudoku-generator -s` (You can still change the difficulty)
---
## Installation
### Executable Download
1. Click on the Releases tab and download the system specific executable file
2. Create a folder named ***sudokus*** in the same directory the executable file is located
3. Now you can follow the instructions in the [How to Use](#how-to-use) section
### Manual Download (not recommended)
1. Install [cargo on the website for Windows](https://doc.rust-lang.org/cargo/getting-started/installation.html) or through your package manager on Linux
    
    Don't know which package manager you use?
   1. If [neofetch](https://github.com/dylanaraps/neofetch) is installed:
    
        `$ neofetch` 
   2. Else look [here](https://en.wikipedia.org/wiki/List_of_software_package_management_systems) and see if you recognize any of these names and try out to use it
   3. Look it up online
2. Clone the repository

    `$ git clone https://github.com/Kopernikus73/sudoku`

3. Navigate into the directory of the cloned repository
   
    `$ cd your/path/sudoku`
4. Build the executable
    `$ cargo build --release`
    
   The executable file will be in the ***./target/release/*** directory and is named ***sudoku-generator***
5. Now you can follow the instructions in the [How to Use](#how-to-use) section
---
## License
Licence is [GPL-3.0](https://github.com/Kopernikus73/sudoku/blob/main/LICENSE)