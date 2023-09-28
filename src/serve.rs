/// Serve webpage (HTML, CSS, game, etc) located at the directory
pub fn serve(web_dir: &str, bind_addr: &str) {
    println!(
        "Listening to {} and serving content at {}",
        bind_addr, web_dir
    );
}
