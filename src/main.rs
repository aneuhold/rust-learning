use svg_generation::run_svg_code;

mod networking;
mod svg_generation;

#[tokio::main]
async fn main() {
    run_svg_code().await;
}
