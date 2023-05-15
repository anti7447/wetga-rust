use std::fs::*;
use std::io::{Read};
use std::path::PathBuf;

#[cfg(target_os = "windows")]
const ENTER: &str = "\r\n";

#[cfg(target_os = "linux")]
const ENTER: &str = "\n";

#[derive(Debug)]
struct Block {
    name: String,
    position_x: String,
    position_y: String,
    rotate: String,
}

fn main() {
    let path = get_path_world();
    let blocks = create_blocks(&path.unwrap()[0]).unwrap();

    // A stub so that there is no warning "structure fields are never read"
    let _ = [&blocks[0].name, &blocks[0].position_x, &blocks[0].position_y, &blocks[0].rotate];

    println!("{:?}", blocks);
}

fn create_blocks(path: &PathBuf) -> std::io::Result<Vec<Block>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut lines: Vec<&str> = content.split(ENTER).collect();
    let mut blocks: Vec<Block> = Vec::new();

    for _ in 0..(lines.len()/4) { 
        blocks.push(Block {
            name: lines.remove(0).to_string(),
            position_x: lines.remove(0).to_string(),
            position_y: lines.remove(0).to_string(),
            rotate: lines.remove(0).to_string()
        });
    }

    return Ok(blocks);
}

fn get_path_world() -> std::io::Result<Vec<PathBuf>> {
    let dir = read_dir(".\\worlds")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    
    Ok(dir)
}