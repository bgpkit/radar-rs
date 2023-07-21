//! # `radar-rs`
//!
//! `radar-rs` is an **unofficial** [Cloudflare Radar](https://radar.cloudflare.com/) API Rust SDK. This library provides
//! a convenient way to access the Cloudflare Radar API, such as [BGP routing stats](https://developers.cloudflare.com/api/operations/radar-get-bgp-routes-stats).
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! radar-rs = "0.0.1"
//! ```
//!
//! Add your API token to the environment variable `CF_API_TOKEN`.
//!
//! Then you can use the library as follows:
//! ```no_run
//! use radar_rs::RadarClient;
//! fn main(){
//!     let client = RadarClient::new().unwrap();
//!     let global_stats = client.get_bgp_routing_stats(None, None).unwrap();
//!     println!("global stats: {:?}", global_stats);
//!     assert!(global_stats.stats.routes_total > 1_000_000);
//! }
//! ```
//!
//! ## Data License
//!
//! The use of the Cloudflare data is under [`CC BY-SA 4.0`](https://creativecommons.org/licenses/by-nc/4.0/) license.
//!
//! This library does not provide any direct access to the API data.
//!
//! See [Cloudflare Radar about page](https://radar.cloudflare.com/about) for more details.
//!
//! ## Obtain API Token
//!
//! See [Cloudflare Radar API getting started guide](https://developers.cloudflare.com/radar/get-started/first-request/)
//! for detailed steps on obtaining a API token.
//!
//! Once you have the API token, you can set it as an environment variable `CF_API_TOKEN`.
//!

mod radar;

pub use crate::radar::*;

#[cfg(test)]
mod tests {
    use crate::RadarClient;

    #[test]
    fn test() {
        let client = RadarClient::new().unwrap();
        let global_stats = client.get_bgp_routing_stats(None, None).unwrap();
        println!("global stats: {:?}", global_stats);
        assert!(global_stats.stats.routes_total > 1_000_000);
    }
}
