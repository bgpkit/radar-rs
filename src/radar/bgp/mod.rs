use serde::{Deserialize, Serialize};

mod prefix_origins;
mod routing_stats;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BgpRoutesMeta {
    pub data_time: String,
    pub query_time: String,
    pub total_peers: i64,
}

pub use prefix_origins::*;
pub use routing_stats::*;
