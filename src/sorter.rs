use crate::schema::Table;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;
use std::collections::HashMap;
use anyhow::Result; // FIXED: Removed unused `anyhow` macro import

pub fn sort_tables(tables: Vec<Table>) -> Result<Vec<Table>> {
    let mut graph = DiGraph::<&Table, ()>::new();
    let mut indices: HashMap<String, NodeIndex> = HashMap::new();

    // 1. Create Nodes
    for table in &tables {
        let idx = graph.add_node(table);
        indices.insert(table.table_name.clone(), idx);
    }

    // 2. Create Edges (Dependencies)
    for table in &tables {
        if let Some(child_idx) = indices.get(&table.table_name) {
            for fk in &table.foreign_keys {
                if let Some(parent_idx) = indices.get(&fk.ref_table) {
                    // Dependency: Parent -> Child (Parent must exist before Child)
                    if child_idx != parent_idx {
                        graph.add_edge(*parent_idx, *child_idx, ());
                    }
                }
            }
        }
    }

    // 3. Perform Topological Sort
    match toposort(&graph, None) {
        Ok(sorted_indices) => {
            let sorted_tables: Vec<Table> = sorted_indices
                .iter()
                .map(|idx| {
                    let t = graph[*idx];
                    t.clone()
                })
                .collect();
            Ok(sorted_tables)
        }
        Err(cycle) => {
            let node = graph[cycle.node_id()];
            println!("⚠️ Warning: Circular dependency detected involving table '{}'. Falling back to standard order.", node.table_name);
            Ok(tables)
        }
    }
}