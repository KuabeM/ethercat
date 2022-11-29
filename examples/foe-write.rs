use ethercat::{Master, MasterAccess};

fn main() -> Result<(), std::io::Error> {
    let mut master = Master::open(0, MasterAccess::ReadWrite)?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: foe-read <slave-position> <foe-name> <file>");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not enough arguments",
        ));
    }
    let slave_idx = args[1]
        .parse::<u16>()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        .into();
    let foe_name = &args[2];
    let file = &args[3];
    let buf = std::fs::read(file)?;

    master.foe_write(slave_idx, foe_name, &buf)?;
    Ok(())
}
