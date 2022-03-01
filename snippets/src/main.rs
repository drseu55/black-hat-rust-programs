use crate::chapter04_dispatch::Processor;

mod chapter04;
mod chapter04_dispatch;

fn main() {
    println!("{}", chapter04::add(2, 3));

    let number: u64 = 1;
    let name: &str = "John";

    chapter04::generic_display(number);
    chapter04::generic_display(name);

    // Doesn't work because Vector does not implement Display trait
    // let v: Vec<i64> = vec![1, 2, 3];
    // chapter04::generic_display(v);

    // Static dispatch
    let processor1 = chapter04_dispatch::Cisc {};
    let processor2 = chapter04_dispatch::Risc {};

    chapter04_dispatch::process_static_dispatch(&processor1, 1);
    chapter04_dispatch::process_static_dispatch(&processor2, 2);

    // Dynamic dispatch
    let processors: Vec<Box<dyn Processor>> = vec![
        Box::new(chapter04_dispatch::Cisc {}),
        Box::new(chapter04_dispatch::Risc {}),
    ];

    for processor in processors {
        chapter04_dispatch::process_dynamic_dispatch(&*processor, 1);
    }
}
