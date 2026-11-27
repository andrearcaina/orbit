# orbit

A toy DNS resolver CLI/TUI tool for learning how DNS and networking fundamentals works, as well as writing idiomatic Rust.

## DNS resolution flow

At a high level, resolving `example.com` looks like this:

```plaintext
Orbit CLI
    -> DNS query: example.com
    -> Root server
    <- Referral to .com TLD server
    -> .com TLD server
    <- Referral to example.com authoritative server
    -> example.com authoritative server
    <- A/AAAA/CNAME answer or NXDOMAIN
Orbit parses and displays the response
```

For a deeper explanation of packets, recursion, referrals, caching, and transport, see [`docs/dns.md`](docs/dns.md).

## Roadmap

- [x] Initialize Rust CLI with `clap`
- [ ] Build and parse DNS queries (either with or without UDP)
- [ ] Query root, TLD, and authoritative servers; add caching and better output
- [ ] Add a Ratatui/Crossterm TUI
