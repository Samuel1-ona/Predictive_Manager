use async_graphql::{Request, Response};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{Amount, ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};

pub mod state;

// Re-export types for convenience
pub use state::{
    MarketId, PlayerId, OutcomeId, GuildId, AchievementId,
    MarketType, MarketStatus, ResolutionMethod,
    GameConfig, Market, Player, Guild, Leaderboard,
};

pub struct PredictiveManagerAbi;

impl ContractAbi for PredictiveManagerAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for PredictiveManagerAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    // Player operations
    RegisterPlayer { display_name: Option<String> },
    UpdateProfile { display_name: Option<String> },
    ClaimDailyReward,
    
    // Market operations
    CreateMarket {
        title: String,
        description: String,
        outcome_names: Vec<String>,
        duration_seconds: u64,
        resolution_method: ResolutionMethod,
    },
    BuyShares {
        market_id: MarketId,
        outcome_id: OutcomeId,
        amount: Amount,
        max_price_per_share: Amount,
    },
    SellShares {
        market_id: MarketId,
        outcome_id: OutcomeId,
        shares: Amount,
        min_price_per_share: Amount,
    },
    
    // Voting operations
    VoteOnOutcome {
        market_id: MarketId,
        outcome_id: OutcomeId,
    },
    TriggerResolution { market_id: MarketId },
    ClaimWinnings { market_id: MarketId },
    
    // Guild operations
    CreateGuild { name: String },
    JoinGuild { guild_id: GuildId },
    LeaveGuild,
    ContributeToGuild { amount: Amount },
    
    // Admin operations
    UpdateGameConfig { config: GameConfig },
}
