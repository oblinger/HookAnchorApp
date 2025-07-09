use hookanchor::command_matches_query;

fn main() {
    println!("Testing amap matches:");
    println!("amap -> Roadmap: {}", command_matches_query("Roadmap", "amap"));
    println!("amap -> ama page: {}", command_matches_query("ama page", "amap"));
    println!("amap -> AMA.Page: {}", command_matches_query("AMA.Page", "amap"));
}