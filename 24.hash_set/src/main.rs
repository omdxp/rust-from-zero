use std::collections::HashSet;

fn main() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("delta"); // nothing happens
    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("vega added!") // happens only if vega was not in the hash set
    }

    if !greeks.contains("kappa") {
        println!("kappa is not here")
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("delta removed")
    }
    println!("{:?}", greeks);

    let _1_5: HashSet<_> = (1..=5).collect(); // the order is always random
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );

    // disjoint (no common elements)
    println!(
        "is {:?} a disjoint of {:?}? {}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    // union, intersection
    println!(
        "items in either {:?} and {:?} are {:?}",
        _2_8,
        _6_10,
        _2_8.union(&_6_10)
    );

    // difference
    // symmetric_difference = (union - intersection)
}

// hash set all the elements are unique and no guarantees they are in order
