//! F-Martingale Bot API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::fmart_bot::*;

impl BybitClient {
    /// Create a new futures Martingale bot with DCA averaging strategy.
    pub async fn create_f_mart_bot(
        &self,
        params: CreateFMartBotParams,
    ) -> Result<CreateFMartBotResponse> {
        self.post("/v5/fmartingalebot/create", &params).await
    }
}
