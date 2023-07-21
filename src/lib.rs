mod radar;

pub use crate::radar::RadarClient;

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
