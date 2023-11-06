use hwlocality::{
    object::depth::NormalDepth,
    topology::{
        builder::BuildFlags,
        support::{DiscoverySupport, FeatureSupport},
    },
    Topology,
};

fn main() {
    let flags = BuildFlags::INCLUDE_DISALLOWED;
    let topology = Topology::builder()
        .with_flags(flags)
        .unwrap()
        .build()
        .unwrap();
    // let topology = Topology::new().unwrap();

    assert!(topology.supports(FeatureSupport::discovery, DiscoverySupport::pu_count));

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
        println!("{:?} {:?}", object.name(), object.total_memory());
    }

    println!("PCI devices:");
    for (idx, object) in topology.pci_devices().enumerate() {
        println!("{idx}: {object}");
    }

    println!("OS devices:");
    for (idx, object) in topology.os_devices().enumerate() {
        println!("{idx}: {object}");
    }
}
