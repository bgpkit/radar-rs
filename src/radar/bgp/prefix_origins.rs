use serde::{Deserialize, Serialize};
use crate::radar::bgp::BgpRoutesMeta;
use crate::radar::client::RadarClient;
use crate::radar::error::RadarError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrefixOriginsEntry {
    pub origin: i64,
    pub peer_count: i64,
    pub prefix: String,
    pub rpki_validation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrefixOriginsResult {
    pub meta: BgpRoutesMeta,
    pub prefix_origins: Vec<PrefixOriginsEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PrefixOriginsResponse {
    pub result: PrefixOriginsResult,
    pub success: bool,
}

impl RadarClient{

    pub fn get_bgp_prefix_origins(&self, origin: Option<u32>, prefix: Option<String>, rpki_status: Option<String>) -> Result<PrefixOriginsResult, RadarError>{
        if origin.is_none() && prefix.is_none() {
            return Err(RadarError::InvalidParamsError("prefix_origins: origin or prefix must be specified".to_string()));
        }

        let mut route = "radar/bgp/routes/pfx2as".to_string();

        let mut params = vec![];
        if let Some(origin) = origin {
            params.push(format!("origin={}", origin));
        }
        if let Some(prefix) = prefix {
            params.push(format!("prefix={}", prefix));
        }
        if let Some(rpki_status) = rpki_status {
            params.push(format!("rpkiStatus={}", rpki_status));
        }

        if params.len() > 0 {
            route = format!("{}?{}", route, params.join("&").as_str());
        }

        let response = self.send_request(route.as_str())?;
        if !response.status().is_success() {
            return Err(RadarError::RequestError(response.error_for_status().unwrap_err()));
        }
        Ok(response.json::<PrefixOriginsResponse>()?.result)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_origin() {
        let client = RadarClient::new().unwrap();

        assert!(client.get_bgp_prefix_origins(None, None, None).is_err());
        let res = client.get_bgp_prefix_origins(Some(13335), None, None).unwrap();
        assert!(res.prefix_origins.len()>1);
        let res = client.get_bgp_prefix_origins(None, Some("1.1.1.0/24".to_string()), None).unwrap();
        assert_eq!(res.prefix_origins.len(), 1);
        // non-existing prefix
        let res = client.get_bgp_prefix_origins(None, Some("1.1.1.1/25".to_string()), None).unwrap();
        assert_eq!(res.prefix_origins.len(), 0);
    }
}