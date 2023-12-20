use std::{collections::HashMap, sync::Arc, time::Instant};

use osmpbfreader::OsmObj;
use rstar::{
    primitives::{GeomWithData, Line},
    RTree,
};

fn main() {
    let start = Instant::now();
    let mut pbf = osmpbfreader::OsmPbfReader::new(
        std::fs::File::open("/Users/giangminh/Downloads/vietnam-latest.osm.pbf").unwrap(),
    );
    println!("Loaded pbf in {}ms", start.elapsed().as_millis());

    let mut nodes = HashMap::new();
    let mut nodes_count = 0;
    let mut ways_count = 0;
    let mut lines_count = 0;

    let mut tree = RTree::new();
    for obj in pbf.iter() {
        match obj {
            Ok(OsmObj::Node(node)) => {
                nodes.insert(node.id.0, [node.lat() as f32, node.lon() as f32]);
                nodes_count += 1;
                if nodes_count % 10000 == 0 {
                    println!(
                        "Loaded {} ways {} nodes in {}ms",
                        ways_count,
                        nodes_count,
                        start.elapsed().as_millis()
                    );
                }
            }
            Ok(OsmObj::Way(way)) => {
                ways_count += 1;
                if ways_count % 10000 == 0 {
                    println!(
                        "Loaded {} ways {} nodes in {}ms",
                        ways_count,
                        nodes_count,
                        start.elapsed().as_millis()
                    );
                }
                let name = if let Some(name) = way.tags.get("name") {
                    name.to_string()
                } else {
                    continue;
                };

                let way_data = Arc::new(name);
                let mut start_point = None;
                for node in &way.nodes {
                    if let Some(node_point) = nodes.get(&(node.0)) {
                        if let Some(start_point) = &start_point {
                            let line = Line::new(*start_point, *node_point);
                            lines_count += 1;
                            tree.insert(GeomWithData::new(line, way_data.clone()));
                        } else {
                            start_point = Some(*node_point);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    drop(nodes);
    drop(pbf);
    println!(
        "Loaded {} ways {} lines in {}ms",
        ways_count,
        lines_count,
        start.elapsed().as_millis()
    );

    loop {
        let mut lat = 0.0;
        let mut lon = 0.0;
        if scanf::scanf!("{}, {}", lat, lon).is_ok() {
            println!("Finding address for {} {}", lat, lon);
            let start = Instant::now();
            match tree.nearest_neighbor(&[lat, lon]) {
                Some(res) => {
                    println!(
                        "Found address: {} in {}micro seconds",
                        res.data,
                        start.elapsed().as_micros()
                    );
                }
                None => {
                    println!(
                        "Not found address in {}micro seconds",
                        start.elapsed().as_micros()
                    );
                }
            }
        }
    }
}
