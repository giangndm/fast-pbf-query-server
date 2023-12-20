use std::{sync::Arc, time::Instant};

use osmpbfreader::OsmObj;
use rstar::{RTree, primitives::{Line, GeomWithData}};

fn main() {
    let start = Instant::now();
    let mut pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open("/Users/giangminh/Downloads/vietnam-latest.osm.pbf").unwrap());
    println!("Loaded pbf in {}ms", start.elapsed().as_millis());
    let mut tree = RTree::new();
    let objs = pbf.get_objs_and_deps(|obj| {
        obj.is_way() || obj.is_node()
    }).unwrap();
    println!("Loaded objs in {}ms", start.elapsed().as_millis());

    let mut ways = 0;
    let mut lines = 0;

    for (_id, way) in &objs {
        if let OsmObj::Way(way) = way {
            ways += 1;
            //get all lines
            let name = if let Some(name) = way.tags.get("name") {
                name.to_string()
            } else {
                continue;
            };
            let way_data = Arc::new(name);
            let mut start_point = None;
            for node in &way.nodes {
                if let Some(OsmObj::Node(node)) = objs.get(&osmpbfreader::OsmId::Node(*node)) {
                    if let Some(start_point) = &start_point {
                        let to_point = [node.lat(), node.lon()];
                        let line = Line::new(*start_point, to_point);
                        lines += 1;
                        tree.insert(GeomWithData::new(line, way_data.clone()));
                    } else {
                        start_point = Some([node.lat(), node.lon()]);
                    }
                }
            }
        }
    }
    drop(objs);
    drop(pbf);

    println!("Loaded {} ways {} lines in {}ms", ways, lines, start.elapsed().as_millis());

    loop {
        let mut lat = 0.0;
        let mut lon = 0.0;
        if scanf::scanf!("{}, {}", lat, lon).is_ok() {
            println!("Finding address for {} {}", lat, lon);
            let start = Instant::now();
            match tree.nearest_neighbor(&[lat, lon]) {
                Some(res) => {
                    println!("Found address: {} in {}micro seconds", res.data, start.elapsed().as_micros());
                },
                None => {
                    println!("Not found address in {}micro seconds", start.elapsed().as_micros());
                }
            }
        }
    }
}
