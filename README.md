A Rust program that finds the shortest path between i (start) and O (goal) on a map using Breadth-First Search (BFS) with toroidal topology. The edges of the map wrap around, creating a continuous "donut-shaped" space where moving off one edge brings you to the opposite side.

🛠️ Installation
Install Rust (if not already installed)
Clone this repository:
    git clone https://github.com/your-username/pathfinder-toroidal.git
    cd pathfinder-toroidal

💻 Usage
Run the program with a map file as an argument:
    cargo run -- <map_file>

🌐 Example
Normal Pathfinding
    Input (example.map):
    ##    #
    #  #i #
    #  O## 
       #   