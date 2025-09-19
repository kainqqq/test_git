use std::collections::VecDeque;
use std::fs;
use std::env;
use std::process;

type Point = (usize, usize);

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_file(filename: &str) -> Result<Self, String> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;
        
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err("File is empty".to_string());
        }
        
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let mut grid = vec![vec![' '; width]; height];
        
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                grid[y][x] = ch;
            }
        }
        
        Ok(Map { grid, width, height })
    }

    fn find(&self, target: char) -> Option<Point> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == target {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn set(&mut self, x: usize, y: usize, value: char) {
        self.grid[y][x] = value;
    }

    fn normalize(&self, x: isize, y: isize) -> (usize, usize) {
        let x_norm = ((x % self.width as isize + self.width as isize) % self.width as isize) as usize;
        let y_norm = ((y % self.height as isize + self.height as isize) % self.height as isize) as usize;
        (x_norm, y_norm)
    }

    fn bfs(&self, start: Point, end: Point) -> Option<Vec<Point>> {
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut parent = vec![vec![None; self.width]; self.height];
        let mut queue = VecDeque::new();
        
        visited[start.1][start.0] = true;
        queue.push_back(start);
        
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == end {
                let mut path = Vec::new();
                let mut current = end;
                while current != start {
                    path.push(current);
                    current = parent[current.1][current.0].unwrap();
                }
                path.reverse();
                return Some(path);
            }
            
            for &(dx, dy) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                let (nx_norm, ny_norm) = self.normalize(nx, ny);
                
                if self.get(nx_norm, ny_norm) != '#' && !visited[ny_norm][nx_norm] {
                    visited[ny_norm][nx_norm] = true;
                    parent[ny_norm][nx_norm] = Some((x, y));
                    queue.push_back((nx_norm, ny_norm));
                }
            }
        }
        None
    }

    fn mark_path(&mut self, path: &[Point]) {
        for &(x, y) in path {
            if self.get(x, y) != 'i' && self.get(x, y) != 'O' {
                self.set(x, y, '.');
            }
        }
    }

    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <map_file>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    let mut map = match Map::from_file(filename) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
    
    let start = map.find('i');
    let end = map.find('O');

    if let (Some(start_pos), Some(end_pos)) = (start, end) {
        if let Some(path) = map.bfs(start_pos, end_pos) {
            map.mark_path(&path);
        }
    }
    
    println!("{}", map.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn create_test_map() -> Map {
        let grid: Vec<Vec<char>> = vec![
            "##    #".chars().collect::<Vec<char>>(),
            "#  #i #".chars().collect::<Vec<char>>(),
            "#  O## ".chars().collect::<Vec<char>>(),
            "   #   ".chars().collect::<Vec<char>>(),
        ];
        let width = grid[0].len();
        let height = grid.len();
        Map { grid, width, height }
    }

    #[test]
    fn test_find_start_end() {
        let map = create_test_map();
        assert_eq!(map.find('i'), Some((4, 1)));
        assert_eq!(map.find('O'), Some((3, 2)));
    }

    #[test]
    fn test_normalize() {
        let map = create_test_map();
        assert_eq!(map.normalize(7, 0), (0, 0));
        assert_eq!(map.normalize(-1, 0), (6, 0));
    }

    #[test]
    fn test_bfs() {
        let map = create_test_map();
        let path = map.bfs((4, 1), (3, 2)).unwrap();
        assert!(path.len() > 0);
    }

    #[test]
    fn test_mark_path() {
        let mut map = create_test_map();
        let path = map.bfs((4, 1), (3, 2)).unwrap();
        map.mark_path(&path);
        assert_eq!(map.get(4, 1), 'i');
        assert_eq!(map.get(3, 2), 'O');
        assert_eq!(map.get(4, 0), '.');
    }

    #[test]
    fn test_from_file() {
        // Create a temporary test file
        let test_content = "##    #\n#  #i #\n#  O## \n   #   ";
        let filename = "test_map.txt";
        
        let mut file = File::create(filename).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();
        
        let map = Map::from_file(filename).unwrap();
        assert_eq!(map.height, 4);
        assert_eq!(map.width, 7);
        assert_eq!(map.find('i'), Some((4, 1)));
        
        // Clean up
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_from_file_error() {
        let result = Map::from_file("non_existent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_path() {
        // Test case where there's truly no path due to complete isolation
        // This map has 'i' and 'O' completely surrounded by walls with no gaps
        let grid: Vec<Vec<char>> = vec![
            "#######".chars().collect::<Vec<char>>(),
            "#i    #".chars().collect::<Vec<char>>(),
            "# #### #".chars().collect::<Vec<char>>(),
            "# #  # #".chars().collect::<Vec<char>>(),
            "# #### #".chars().collect::<Vec<char>>(),
            "#    O#".chars().collect::<Vec<char>>(),
            "#######".chars().collect::<Vec<char>>(),
        ];
        let map = Map {
            grid,
            width: 7,
            height: 7,
        };
        
        let start = map.find('i').unwrap();
        let end = map.find('O').unwrap();
        let path = map.bfs(start, end);
        
        // With this configuration, there should be no path
        assert!(path.is_none(), "Expected no path, but found one");
    }

    #[test]
    fn test_toroidal_path() {
        // Test case that specifically uses toroidal topology
        // 'i' on left edge, 'O' on right edge - should find path through wrapping
        let grid: Vec<Vec<char>> = vec![
            "##   ##".chars().collect::<Vec<char>>(),
            "i     O".chars().collect::<Vec<char>>(),
            "##   ##".chars().collect::<Vec<char>>(),
        ];
        let map = Map {
            grid,
            width: 7,
            height: 3,
        };
        
        let start = map.find('i').unwrap();
        let end = map.find('O').unwrap();
        let path = map.bfs(start, end);
        
        // With toroidal topology, there should be a path
        assert!(path.is_some(), "Expected a path with toroidal topology");
    }
}