# orbit

A Rust-based toy DNS resolver CLI/TUI tool for learning DNS and networking fundamentals.

## DNS resolution flow

At a high level, resolving `example.com` looks like this:

```plaintext
Orbit CLI
    -> DNS query: example.com
    -> Recursive resolver
        -> Root server
            -> .com TLD server
                -> example.com authoritative server
                <- A/AAAA/CNAME answer or NXDOMAIN
            <- Referral or answer
        <- Recursive answer
    <- DNS response over UDP/TCP
Orbit parses and displays the response
```

For a deeper explanation of packets, recursion, referrals, caching, and transport, see [`docs/dns.md`](docs/dns.md).

## Roadmap

- [x] Initialize Rust CLI with `clap`
- [ ] Implement basic DNS resolution through a recursive resolver
- [ ] Implement iterative recursive resolution, caching, and better output
- [ ] Add a Ratatui/Crossterm TUI
