fn main() {
    let input = include_str!("../input.txt");
    let (_, map) = input.split_once("\n\n").unwrap();

    let mut s = String::new();
    for (i, line) in map.lines().enumerate() {
        let mut x = line.split_ascii_whitespace();
        let left = x.next().unwrap();
        let op = x.next().unwrap();
        let right = x.next().unwrap();
        x.next().unwrap();
        let out = x.next().unwrap();
        s.push_str(format!("{left}[shape=point]\n").as_str());
        s.push_str(format!("{right}[shape=point]\n").as_str());
        if out.starts_with("z") {
            s.push_str(format!("{out}[style=\"filled\" fillcolor=\"red\"]\n").as_str());
        } else {
            s.push_str(format!("{out}[shape=point]\n").as_str());
        }
        match op {
            "AND" => s.push_str(format!("{i}[label=\"{line}\"][shape=box]\n").as_str()),
            "OR" => s.push_str(format!("{i}[label=\"{line}\"][shape=circle]\n").as_str()),
            "XOR" => s.push_str(format!("{i}[label=\"{line}\"][shape=diamond]\n").as_str()),
            _ => (),
        }
        s.push_str(format!("{left} -> {i}\n").as_str());
        s.push_str(format!("{right} -> {i}\n").as_str());
        s.push_str(format!("{i} -> {out}\n").as_str());
    }

    let ans = format!("Digraph G {{\n{s}}}");
    println!("{ans}");
    let filename = "visualize.gv";
    std::fs::write(filename, ans).unwrap();
    println!("Saved to {filename}");
}
