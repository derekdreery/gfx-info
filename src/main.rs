use gfx_backend_vulkan as back;
use std::collections::BTreeSet;
use bytesize::ByteSize;
use std::fmt;

use gfx_hal::{
    Instance,
    memory::Properties,
    adapter::PhysicalDevice,
};

fn main() {
    let instance = back::Instance::create("gfx-info", 1);
    let adapters = instance.enumerate_adapters();
    for (idx, adapter) in adapters.into_iter().enumerate() {
        println!("Adapter {}", idx);
        println!("  Name:          {}", adapter.info.name);
        println!("  Vendor/Device: {}:{}", adapter.info.vendor, adapter.info.device);
        println!("  Type:          {:?}", adapter.info.device_type);
        println!();

        let memory_properties = adapter.physical_device.memory_properties();
        let mut memory_types: BTreeSet<(usize, Properties)> = BTreeSet::new();
        for memory_type in memory_properties.memory_types.iter() {
            memory_types.insert((memory_type.heap_index, memory_type.properties));
        }
        println!("  GPU Memory Heaps:");
        for (idx, size) in memory_properties.memory_heaps.iter().enumerate() {
            println!("    Heap {:2} - {}", idx, ByteSize::b(*size));
            println!("      Configurations:");
            for tys in memory_types.iter().filter(|(idx2, _)| *idx2 == idx).map(|(_, val)| val) {
                print_neat_bitfield(tys, 8);
            }
        }
        println!();

        println!("  Adapter Features:");
        print_neat_bitfield(adapter.physical_device.features(), 4);
        println!();

        println!("  Adapter Limits:");
        let limits = format!("{:#?}", adapter.physical_device.limits());
        let limits = limits.lines().map(|v| v.replace('_', " ")).collect::<Vec<_>>();
        for win in limits.windows(2).skip(1) {
            println!("{}", win[0]);
        }
        println!();
    }

}


fn print_neat<'a>(iter: impl Iterator<Item=&'a str>, separator: Option<char>, indent: usize) {
    let mut acc = indent;
    for _ in 0..indent {
        print!(" ");
    }
    let mut iter = iter.peekable();
    while let Some(val) = iter.next() {
        let extra = match separator {
            Some(..) => 3,
            None => 1
        };
        if acc + val.len() + extra >= 80 {
            if acc == indent {
                panic!("Not enough screen space");
            }
            print!("\n");
            for _ in 0..indent {
                print!(" ");
            }
            acc = indent;
        }
        acc += val.len() + extra;
        print!("{}", val);
        if iter.peek().is_some() {
            print!(" ");
            if let Some(s) = separator {
                print!("{} ", s);
            }
        }
    }
    println!();
}

fn print_neat_bitfield(t: impl fmt::Debug, indent: usize) {
    let t = format!("{:?}", t);
    print_neat(t.split_whitespace().filter(|&v| v != "|"), Some('|'), indent);
}
