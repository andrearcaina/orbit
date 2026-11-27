use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Resolve DNS names by following the DNS hierarchy.",
    long_about = "Orbit is a learning-focused DNS resolver.\n\
\n\
It queries DNS root servers, follows TLD \
and authoritative-server referrals, and displays the final DNS response.\n\
\n\
Orbit does not delegate resolution to a public recursive resolver."
)]
struct Args {
    #[arg(help = "URL or domain name Orbit should resolve")]
    input: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let resolver = orbit::dns::Resolver::new();

    println!("{}", resolver.resolve(&args.input)?);

    Ok(())
}
