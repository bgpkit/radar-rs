# `radar-rs`: unofficial Cloudflare Radar Rust SDK

*The library is still in very early development. Use with caution.*

`radar-rs` is an **unofficial** [Cloudflare Radar](https://radar.cloudflare.com/) API Rust SDK. This library provides
a convenient way to access the Cloudflare Radar API, such as [BGP routing stats](https://developers.cloudflare.com/api/operations/radar-get-bgp-routes-stats).

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
radar-rs = "0.0.1" 
```

Add your API token to the environment variable `CF_API_TOKEN`.

Then you can use the library as follows:
```rust
use radar_rs::RadarClient;
fn main(){
  let client = RadarClient::new().unwrap();
  let global_stats = client.get_bgp_routing_stats(None, None).unwrap();
  println!("global stats: {:?}", global_stats);
  assert!(global_stats.stats.routes_total > 1_000_000);
}
```

## Data License

The use of the Cloudflare data is under [`CC BY-SA 4.0`](https://creativecommons.org/licenses/by-nc/4.0/) license.

This library does not provide any direct access to the API data.

See [Cloudflare Radar about page](https://radar.cloudflare.com/about) for more details.

## Obtain API Token

See [Cloudflare Radar API getting started guide](https://developers.cloudflare.com/radar/get-started/first-request/) 
for detailed steps on obtaining a API token.

Once you have the API token, you can set it as an environment variable `CF_API_TOKEN`.

## Supported Cloudflare Radar API

- [ ] Radar AS112
- [ ] Radar Annotation
- [ ] Radar Attacks
- [ ] Radar BGP
  - [ ] hijack events
  - [ ] route leak events
  - [ ] MOASes
  - [X] prefix-to-ASN mapping
  - [X] routing table stats
  - [ ] BGP messages volume time series
  - [ ] top ASNs by BGP update count
  - [ ] top ASNs by prefix count
  - [ ] top prefixes by BGP update count
- [ ] Radar DNS
- [ ] Radar Datasets
- [ ] Radar Email Security
- [ ] Radar Entities
- [ ] Radar HTTP
- [ ] Radar Netflows
- [ ] Radar Quality
- [ ] Radar Ranking
- [ ] Radar Search
- [ ] Radar Special Events
- [ ] Radar Verified Bots
