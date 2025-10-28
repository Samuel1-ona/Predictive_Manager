## What it does
Our Prediction Market Game transforms decentralized prediction markets into an engaging, arcade-style gaming experience built on the Linera blockchain. The platform allows players to:

Predict Multiple-Outcome Events: Players buy shares in different outcomes of events (sports, crypto trends, game scenarios, etc.)
Trade in Real-Time: Dynamic AMM-style pricing adjusts based on collective player predictions
Earn Through Gameplay: Win tokens by making accurate predictions, with payouts proportional to shares held
Progress and Level Up: Gain experience points, unlock achievements, and climb leaderboards
Form Guilds: Create or join teams to collaborate on predictions and share rewards
Create Markets: Players can launch their own prediction markets and earn creator fees
Participate in Oracle Voting: Use reputation scores to vote on event outcomes in community-driven resolution
Compete Across Market Types:

Quick Predictions (minutes/hours)
Tournament Brackets (days)
Seasonal Events (weeks/months)
Direct PvP Challenges

The game leverages Linera's microchain architecture, where each player and market runs on its own chain, enabling unprecedented scalability and personalization while maintaining cross-chain synchronization through the Master Game Chain.

## The problem it solves
1. Prediction Markets are Too Complex
Traditional prediction markets like Augur or Polymarket are intimidating for casual users. They require deep understanding of DeFi concepts, liquidity provision, and complex trading interfaces. Our solution gamifies the experience, making prediction markets accessible to anyone who enjoys mobile or casual games.
2. Lack of Engagement and Retention
Existing platforms treat prediction markets purely as financial instruments, leading to:

One-time usage patterns
No progression systems
Limited social features
Zero entertainment value beyond speculation

We solve this by adding:

RPG-style progression (levels, XP, achievements)
Social guilds and leaderboards
Multiple game modes for different playstyles
Visual feedback and arcade-style UX

3. Scalability Bottlenecks
Traditional blockchain prediction markets face:

High gas fees per trade
Network congestion during popular events
Slow transaction finality
Limited throughput for simultaneous users

Linera's microchain architecture solves this by:

Giving each player their own chain (instant personal transactions)
Running each market on its own chain (no congestion)
Using asynchronous cross-chain messages (parallel processing)
Eliminating gas fees through Linera's fee-less model

4. Oracle Problem
Centralized oracles create single points of failure and trust issues. We implement:

Community Oracle Voting: Weighted by player reputation
Hybrid Resolution: Combining votes with automated on-chain data
Reputation Staking: Accurate voters gain reputation, bad actors lose it

5. Poor Token Economics
Many platforms have extractive models. We create a circular economy:

Players earn tokens through accurate predictions
Tokens are spent on market creation and trading
Market creators earn fees
Guilds pool resources for collaborative wins
No external token purchases required to play

## Challenges I ran into
1. Linera's Novel Architecture
Challenge: Linera's microchain model is fundamentally different from traditional blockchains. Understanding how to properly structure applications across multiple chains required rethinking standard smart contract patterns.
Solution:

Deep dive into Linera's documentation and examples
Designed a clear separation: Master Chain (coordination) → Market Chains (trading) → Player Chains (personal state)
Implemented robust cross-chain messaging with proper error handling

2. AMM Pricing Without Complex Math
Challenge: Traditional AMM formulas (like Uniswap's x*y=k) are complex for gaming. We needed simple, intuitive pricing that still felt fair.
Solution:

Simplified bonding curve: price = base_price × (supply / liquidity)^smoothing_factor
Easy to understand: more demand = higher price
Configurable smoothing factor per market type
Built-in slippage protection

5. Oracle Resolution at Scale
Challenge: How do you resolve thousands of markets fairly without centralization or excessive gas costs?
Solution:

Reputation-weighted voting (not plutocratic)
Separate Oracle Application for vote aggregation
Automated resolution for deterministic outcomes
Hybrid model: community votes + blockchain data

## Technologies I used
Blockchain & Smart Contracts

Linera Blockchain: Core platform for microchain architecture
Rust: Smart contract language for Linera applications
linera-sdk: Framework for building Linera applications
linera-views: On-chain storage abstractions (MapView, RegisterView, RootView)

Smart Contract Components

RootView: Application state management
MapView: Key-value storage for markets, players, guilds
RegisterView: Single-value storage for global config
Operations: User-initiated actions (synchronous)
Messages: Cross-chain communication (asynchronous)

AMM & Economics

Custom bonding curve implementation
Proportional payout calculations
Fee distribution algorithms
Token minting/burning mechanisms

Oracle System

Reputation-weighted voting
Quorum calculations
Automated data feeds (for deterministic outcomes)
Multi-signature resolution for disputes

## How we built it
Phase 1: Architecture Design

Studied Linera's Documentation

Understood microchain model
Learned cross-chain messaging patterns
Identified optimal state distribution


Designed Data Models

Market structure with multiple outcomes
Player progression system
Guild collaboration mechanics
Oracle voting framework


Mapped Cross-Chain Flows

Player onboarding flow
Market creation → trading → resolution
Leaderboard synchronization
Guild operations

Phase 2: Core Contract Development
1. Master Game Application
rust- GameConfig: Global parameters
- Market Registry: All active markets
- Leaderboard: Top players and guilds by points
- Points Management: Scoring and distribution system
- Player Registry: All player chains
- Achievement System: Badge unlocks and rewards
2. Market Application
rust- Market State: Outcomes, participation, positions
- Prediction Tracking: Player choices per outcome
- Position Recording: Who predicted what and when
- Resolution Handling: Winner determination and points calculation
- Bonus Points: Early prediction rewards, accuracy multipliers
3. Player Application
rust- Points Balance: Total career points and seasonal points
- Stats: Win rate, level, reputation, prediction accuracy
- Positions: Active market participations
- Achievements: Unlocked badges and milestones
- Prediction History: All past predictions with outcomes
4. Oracle Application
rust- Voting Management: Collect and weight votes
- Reputation Scoring: Track voter accuracy (impacts voting power)
- Resolution Triggers: Automated outcome determination
- Points Distribution: Calculate and award points to winners

Phase 3: Feature Implementation
Points & Reward System

Base points for correct predictions
Bonus multipliers for early predictions
Accuracy streaks increase point multipliers
Difficulty ratings affect point rewards
Leaderboard position tracking

Prediction System

Make predictions on multiple outcomes
Lock-in predictions before market closes
Track prediction confidence levels
Real-time position updates
Prediction history and analytics

Progression System

XP calculation based on prediction activity
Level unlocks for new features and markets
Achievement definitions and checking
Reputation updates for oracle accuracy
Seasonal rankings and resets

Guild System

Guild creation with founder privileges
Member management and invites
Shared point pools and team rankings
Collaborative prediction challenges
Guild vs Guild tournaments

Oracle & Resolution

Vote submission with reputation weighting
Quorum checking for valid resolution
Automated resolution for deterministic outcomes
Points calculation and distribution to winners
Accuracy tracking for reputation updates

Phase 4: Cross-Chain Integration

Implemented message passing between chains
Created synchronization protocols
Built event emission for tracking
Developed error handling for failed messages

Phase 5: Testing & Optimization

Unit tests for core functions
Integration tests for cross-chain flows

## What we learned
Microchain Architecture is Powerful

Each user having their own chain eliminates congestion
Parallel processing is the future of blockchain scalability
Cross-chain messaging requires careful state management

## What's next for
1. Frontend Development

React-based web interface
Wallet integration (Linera wallet)
Real-time market updates via WebSocket
Mobile-responsive design
Spectator mode with live charts

2. Enhanced Game Modes

Battle Royale: Last predictor standing wins maximum points
Prediction Pools: Group predictions on related events with shared rewards
Chain Predictions: Predict outcomes of sequential events for multiplier bonuses
Mystery Markets: Hidden outcomes until resolution with higher point rewards
Speed Rounds: Quick predictions with time-decay point bonuses

3. AI Integration
AI-powered prediction suggestions
