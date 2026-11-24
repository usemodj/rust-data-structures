// Only these bundled images may be rendered or served by the site.
pub fn get(path: &str) -> Option<&'static str> {
    match path {
        "/assets/btree-map.svg" => Some(include_str!("btree-map.svg")),
        "/assets/vec.svg" => Some(include_str!("diagrams/vec.svg")),
        "/assets/vecdeque.svg" => Some(include_str!("diagrams/vecdeque.svg")),
        "/assets/hashmap.svg" => Some(include_str!("diagrams/hashmap.svg")),
        "/assets/stack.svg" => Some(include_str!("diagrams/stack.svg")),
        "/assets/hashset.svg" => Some(include_str!("diagrams/hashset.svg")),
        "/assets/binary-heap.svg" => Some(include_str!("diagrams/binary-heap.svg")),
        "/assets/linked-list.svg" => Some(include_str!("diagrams/linked-list.svg")),
        "/assets/btree-set.svg" => Some(include_str!("diagrams/btree-set.svg")),
        "/assets/union-find.svg" => Some(include_str!("diagrams/union-find.svg")),
        "/assets/binary-search-tree.svg" => Some(include_str!("diagrams/binary-search-tree.svg")),
        "/assets/bst-traversals.svg" => Some(include_str!("diagrams/bst-traversals.svg")),
        "/assets/bst-extra-traversals.svg" => {
            Some(include_str!("diagrams/bst-extra-traversals.svg"))
        }
        "/assets/graph-representations.svg" => {
            Some(include_str!("diagrams/graph-representations.svg"))
        }
        "/assets/binary-search.svg" => Some(include_str!("diagrams/binary-search.svg")),
        "/assets/upper-bound.svg" => Some(include_str!("diagrams/upper-bound.svg")),
        "/assets/lower-bound.svg" => Some(include_str!("diagrams/lower-bound.svg")),
        "/assets/sorting.svg" => Some(include_str!("diagrams/sorting.svg")),
        "/assets/bfs.svg" => Some(include_str!("diagrams/bfs.svg")),
        "/assets/dfs.svg" => Some(include_str!("diagrams/dfs.svg")),
        "/assets/dijkstra.svg" => Some(include_str!("diagrams/dijkstra.svg")),
        "/assets/merge-sort.svg" => Some(include_str!("diagrams/merge-sort.svg")),
        "/assets/two-pointers.svg" => Some(include_str!("diagrams/two-pointers.svg")),
        "/assets/topological-sort.svg" => Some(include_str!("diagrams/topological-sort.svg")),
        "/assets/insertion-sort.svg" => Some(include_str!("diagrams/insertion-sort.svg")),
        "/assets/prefix-sum.svg" => Some(include_str!("diagrams/prefix-sum.svg")),
        "/assets/sliding-window.svg" => Some(include_str!("diagrams/sliding-window.svg")),
        "/assets/kruskal.svg" => Some(include_str!("diagrams/kruskal.svg")),
        "/assets/kmp.svg" => Some(include_str!("diagrams/kmp.svg")),
        "/assets/bellman-ford.svg" => Some(include_str!("diagrams/bellman-ford.svg")),
        "/assets/floyd-warshall.svg" => Some(include_str!("diagrams/floyd-warshall.svg")),
        "/assets/knapsack.svg" => Some(include_str!("diagrams/knapsack.svg")),
        "/assets/quicksort.svg" => Some(include_str!("diagrams/quicksort.svg")),
        _ => None,
    }
}
