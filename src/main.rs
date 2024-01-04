fn main() -> Result<(), Box<dyn std::error::Error>> {
    busybody::run(std::env::args().collect(), dispatch)
}

fn dispatch(cmd: busybody::Cmd) -> busybody::Result<()> {
    match cmd {
        busybody::Cmd::Yes => busybody::yes(),
    }
}
