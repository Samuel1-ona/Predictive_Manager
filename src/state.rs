use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};
use linera_sdk::linera_base_types::{AccountOwner, Amount, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type MarketId = u64;
pub type PlayerId = AccountOwner;
pub type OutcomeId = u32;
pub type GuildId = u64;
pub type AchievementId = u32;

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::InputObject)]
pub struct GameConfig {
    pub admin: Option<AccountOwner>,
    pub initial_player_tokens: Amount,
    pub daily_login_reward: Amount,
    pub market_creation_cost: Amount,
    pub min_market_duration_seconds: u64,
    pub max_outcomes_per_market: usize,
    pub oracle_voting_duration_seconds: u64,
    pub min_oracle_voters: u32,
    pub market_creator_fee_percent: u8,
    pub platform_fee_percent: u8,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            admin: None,
            initial_player_tokens: Amount::from_tokens(1000),
            daily_login_reward: Amount::from_tokens(10),
            market_creation_cost: Amount::from_tokens(100),
            min_market_duration_seconds: 300,
            max_outcomes_per_market: 10,
            oracle_voting_duration_seconds: 3600,
            min_oracle_voters: 3,
            market_creator_fee_percent: 2,
            platform_fee_percent: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub id: MarketId,
    pub creator: PlayerId,
    pub title: String,
    pub description: String,
    pub market_type: MarketType,
    pub outcomes: Vec<Outcome>,
    pub creation_time: Timestamp,
    pub end_time: Timestamp,
    pub resolution_time: Option<Timestamp>,
    pub status: MarketStatus,
    pub total_liquidity: Amount,
    pub positions: BTreeMap<PlayerId, PlayerPosition>,
    pub total_participants: u64,
    pub base_price: Amount,
    pub smoothing_factor: f64,
    pub winning_outcome: Option<OutcomeId>,
    pub resolution_method: ResolutionMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketType {
    QuickPrediction,
    TournamentMarket,
    SeasonalEvent,
    PvPChallenge { challenger: PlayerId, challenged: PlayerId },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy, async_graphql::Enum)]
pub enum MarketStatus {
    Active,
    Closed,
    Resolved,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, async_graphql::Enum)]
pub enum ResolutionMethod {
    OracleVoting,
    Automated,
    CreatorDecides,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub id: OutcomeId,
    pub name: String,
    pub total_shares: Amount,
    pub current_price: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPosition {
    pub shares_by_outcome: BTreeMap<OutcomeId, Amount>,
    pub total_invested: Amount,
    pub entry_time: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub display_name: Option<String>,
    pub registration_time: Timestamp,
    pub last_login: Timestamp,
    pub token_balance: Amount,
    pub total_earned: Amount,
    pub total_spent: Amount,
    pub level: u32,
    pub experience_points: u64,
    pub reputation: u64,
    pub markets_participated: u64,
    pub markets_won: u64,
    pub total_profit: Amount,
    pub win_streak: u32,
    pub best_win_streak: u32,
    pub guild_id: Option<GuildId>,
    pub achievements_earned: Vec<AchievementId>,
    pub active_markets: Vec<MarketId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leaderboard {
    pub top_traders: Vec<LeaderboardEntry>,
    pub top_guilds: Vec<GuildLeaderboardEntry>,
    pub last_updated: Timestamp,
}

impl Default for Leaderboard {
    fn default() -> Self {
        Self {
            top_traders: Vec::new(),
            top_guilds: Vec::new(),
            last_updated: Timestamp::from(0u64),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub player_id: PlayerId,
    pub display_name: Option<String>,
    pub total_profit: Amount,
    pub win_rate: f64,
    pub level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    pub id: GuildId,
    pub name: String,
    pub founder: PlayerId,
    pub members: Vec<PlayerId>,
    pub creation_time: Timestamp,
    pub total_guild_profit: Amount,
    pub guild_level: u32,
    pub shared_pool: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildLeaderboardEntry {
    pub guild_id: GuildId,
    pub name: String,
    pub total_profit: Amount,
    pub member_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleVoting {
    pub market_id: MarketId,
    pub voting_start: Timestamp,
    pub voting_end: Timestamp,
    pub votes: BTreeMap<OutcomeId, WeightedVotes>,
    pub voters: Vec<PlayerId>,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedVotes {
    pub total_weight: u64,
    pub voter_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: AchievementId,
    pub name: String,
    pub description: String,
    pub reward_tokens: Amount,
    pub reward_xp: u64,
    pub requirement: AchievementRequirement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementRequirement {
    WinMarkets(u64),
    WinStreak(u32),
    TotalProfit(Amount),
    ParticipateInMarkets(u64),
    CreateMarkets(u64),
    JoinGuild,
    ReachLevel(u32),
}

#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct PredictionMarketState {
    pub config: RegisterView<GameConfig>,
    pub markets: MapView<MarketId, Market>,
    pub players: MapView<PlayerId, Player>,
    pub leaderboard: RegisterView<Leaderboard>,
    pub guilds: MapView<GuildId, Guild>,
    pub oracle_votes: MapView<MarketId, OracleVoting>,
    pub achievements: MapView<AchievementId, Achievement>,
    pub total_supply: RegisterView<Amount>,
    pub next_market_id: RegisterView<MarketId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    MarketCreated { market_id: MarketId, creator: PlayerId },
    MarketResolved { market_id: MarketId, winning_outcome: OutcomeId },
    TradeExecuted {
        player_id: PlayerId,
        market_id: MarketId,
        outcome_id: OutcomeId,
        shares: Amount,
        price: Amount,
    },
    PlayerLeveledUp { player_id: PlayerId, new_level: u32 },
    AchievementUnlocked { player_id: PlayerId, achievement_id: AchievementId },
    GuildCreated { guild_id: GuildId, name: String },
}
