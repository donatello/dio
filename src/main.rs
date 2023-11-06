use hwlocality::{object::depth::NormalDepth, Topology};

fn main() {
    let topology = Topology::new().unwrap();

    for depth in NormalDepth::iter_range(NormalDepth::MIN, topology.depth()) {
        println!("*** Objects at depth {depth}");

        for (idx, object) in topology.objects_at_depth(depth).enumerate() {
            println!("{idx}: {object}");
        }
    }

    println!("Visible CPUs in this topology: {}", topology.cpuset());

    println!("Memory objects:");
    for (idx, object) in topology.memory_objects().enumerate() {
        println!("{idx}: {object}");
    }
}
