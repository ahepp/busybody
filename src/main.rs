fn main() -> Result<(), Box<dyn std::error::Error>> {
    busybody::run(std::env::args().collect())
}
