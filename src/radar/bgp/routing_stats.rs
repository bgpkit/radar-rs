use serde::{Deserialize, Serialize};
use crate::radar::bgp::BgpRoutesMeta;
use crate::radar::client::RadarClient;
use crate::radar::error::RadarError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoutingStatsEntry {
    pub distinct_origins: u32,
    pub distinct_origins_ipv4: u32,
    pub distinct_origins_ipv6: u32,
    pub distinct_prefixes: u32,
    pub distinct_prefixes_ipv4: u32,
    pub distinct_prefixes_ipv6: u32,
    pub routes_invalid: u32,
    pub routes_invalid_ipv4: u32,
    pub routes_invalid_ipv6: u32,
    pub routes_total: u32,
    pub routes_total_ipv4: u32,
    pub routes_total_ipv6: u32,
    pub routes_unknown: u32,
    pub routes_unknown_ipv4: u32,
    pub routes_unknown_ipv6: u32,
    pub routes_valid: u32,
    pub routes_valid_ipv4: u32,
    pub routes_valid_ipv6: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoutingStatsResult {
    pub meta: BgpRoutesMeta,
    pub stats: RoutingStatsEntry,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RoutingStatsResponse {
    pub result: RoutingStatsResult,
    pub success: bool,
}

impl RadarClient{

    pub fn get_bgp_routing_stats(&self, asn: Option<u32>, country_code: Option<String>) -> Result<RoutingStatsResult, RadarError>{
        if asn.is_some() && country_code.is_some() {
            return Err(RadarError::InvalidParamsError("bgp_routing_stats: only one of asn or country code can be specified".to_string()));
        }

        let mut route = "radar/bgp/routes/stats".to_string();

        let mut params = vec![];
        if let Some(asn) = asn {
            params.push(format!("asn={}", asn));
        }
        if let Some(code) = country_code {
            if code.len() != 2 {
                return Err(RadarError::InvalidParamsError("bgp_routing_stats: country code must be 2 characters".to_string()));
            }
            params.push(format!("location={}", code));
        }

        if params.len() > 0 {
            route = format!("{}?{}", route, params.join("&").as_str());
        }

        let response = self.send_request(route.as_str())?;
        if !response.status().is_success() {
            return Err(RadarError::RequestError(response.error_for_status().unwrap_err()));
        }
        Ok(response.json::<RoutingStatsResponse>()?.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_origin() {
        let client = RadarClient::new().unwrap();

        // global routing table stats
        let res = client.get_bgp_routing_stats(None, None);
        assert!(res.is_ok());
        assert!(res.unwrap().stats.routes_total>1_000_000);

        // per_asn routing table stats
        let res = client.get_bgp_routing_stats(Some(13335), None);
        assert!(res.is_ok());
        assert!(res.unwrap().stats.routes_total>1_000);

        // per_asn routing table stats
        let res = client.get_bgp_routing_stats(None, Some("US".to_string()));
        assert!(res.is_ok());
        assert!(res.unwrap().stats.routes_total>1_000);

        // per_asn routing table stats
        let res = client.get_bgp_routing_stats(None, Some("us".to_string()));
        assert!(res.is_ok());
        assert!(res.unwrap().stats.routes_total>1_000);

        // error cases
        assert!(client.get_bgp_routing_stats(None, Some("united stats".to_string())).is_err());
        assert!(client.get_bgp_routing_stats(Some(13335), Some("US".to_string())).is_err());
    }
}
