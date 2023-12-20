use std::{collections::HashMap, time::Instant};

use osmpbfreader::OsmObj;
use rstar::{
    primitives::{GeomWithData, Line},
    RTree,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GeoIndex {
    tree: RTree<GeomWithData<Line<[f32; 2]>, i64>>,
    ways: HashMap<i64, String>,
}

impl GeoIndex {
    pub fn new() -> GeoIndex {
        GeoIndex {
            tree: RTree::new(),
            ways: HashMap::new(),
        }
    }

    pub fn build(&mut self, path: &str) {
        let start = Instant::now();
        let mut pbf = osmpbfreader::OsmPbfReader::new(std::fs::File::open(path).unwrap());
        println!("Loaded pbf in {}ms", start.elapsed().as_millis());

        let mut nodes = HashMap::new();
        let mut nodes_count = 0;
        let mut ways_count = 0;
        let mut lines_count = 0;

        let tree = &mut self.tree;
        let ways = &mut self.ways;

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
                    ways.insert(way.id.0, name);

                    let mut start_point = None;
                    for node in &way.nodes {
                        if let Some(node_point) = nodes.get(&(node.0)) {
                            if let Some(start_point) = &start_point {
                                let line = Line::new(*start_point, *node_point);
                                lines_count += 1;
                                tree.insert(GeomWithData::new(line, way.id.0));
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
    }

    pub fn find(&self, lat: f32, lon: f32) -> Option<String> {
        let way_id = self
            .tree
            .nearest_neighbor(&[lat, lon])
            .map(|res| res.data)?;
        self.ways.get(&way_id).map(|name| name.clone())
    }
}
