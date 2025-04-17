
#[unsafe(no_mangle)]
pub extern "C" fn plugin_entry() {
    println!("CFPlugin loaded!");
}
