use std::{fs::File, io::Read};

use codec_generator::{*, layer::Layer, protocol::Protocol};


fn main() -> std::io::Result<()> {
    let mut f = File::open("./template/packet.layer")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let proto: Protocol = toml::from_str(&buf)?;
    println!("{:#?}", proto);

    Ok(())
}
