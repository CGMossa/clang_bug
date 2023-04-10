use clang::{Clang, Index};

fn main() {
    // println!("clang version: {}", clang::get_version());
    let clang = Clang::new().unwrap();
    let idx = Index::new(&clang, false, false);
    // let tu = idx.parser(source).parse().unwrap();
    let tu = idx.parser("src/unnamed_items.c").parse().unwrap();
    // dbg!(&tu);
    // dbg!(tu.get_entity());
    dbg!(tu.get_entity().get_children());
    for ele in tu.get_entity().get_children() {
        if let Some(name) = ele.get_name() {
            println!("name: {name}");
        }
        if let Some(display_name) = ele.get_display_name() {
            println!("display_name: {display_name}");
        }
    }
}
