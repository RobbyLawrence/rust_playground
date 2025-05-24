use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_graph_from_file(String file_path) {

}
fn main() -> io::Result<()> {
    let file_path = "k5.dim";
    let (num_vertices, adjmatrix) = read_graph_from_file(file_path)?;
    println!("Number of vertices: {}", num_vertices);
    println!("Adjacency Matrix:");
    Ok(())
}
