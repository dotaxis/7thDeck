mod proton;

fn main() {
    let ff7_id: u32 = 39140;
    println!("{}", proton::find_prefix(ff7_id).unwrap().display());
}
