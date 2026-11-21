use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Resolve DNS names by following the DNS hierarchy.",
    long_about = "Orbit is a learning-focused DNS resolver.\n\
\n\
It extracts the hostname from a URL, queries DNS root servers, follows TLD \
and authoritative-server referrals, and displays the final DNS response.\n\
\n\
Orbit does not delegate resolution to a public recursive resolver."
)]
struct Args {
    #[arg(help = "URL whose hostname Orbit should resolve")]
    url: String,
}

fn main() {
    let args = Args::parse();
    let resolver = orbit::dns::Resolver::new("8.8.8.8");

    println!("{}", resolver.resolve(&args.url));
}
