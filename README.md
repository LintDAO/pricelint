# PriceLint: Decentralized Price Prediction on Internet Computer

**PriceLint** is a decentralized platform for AI-driven price prediction, built on the Internet Computer (IC) and inspired by Predictoor.ai.

It combines the features of Canister and Burn frameworks on the chain to achieve a 100% on-chain LSTM prediction model, significantly reducing the dependence of traditional prediction models on local computing resources.

Users can make predictions without continuously running local devices, greatly improving ease of use.

Our goal is to achieve a prediction accuracy rate of slightly above 50% for this model.

Users can deploy AI in PriceLint and predict staked tokens.

Trade your own AI configurations in the upcoming decentralized market to build a self-sustaining data ecosystem.

## 📝 Why PriceLint?

PriceLint democratizes price prediction, traditionally controlled by institutions, by leveraging ICP’s scalable infrastructure. It targets:

- **Crypto Traders**: Access real-time predictive data streams for trading strategies.
- **DeFi Participants**: Stake tokens to earn rewards and optimize predictions.
- **Data Scientists**: Share AI models, earning revenue through a follow-on profit-sharing model.

## 🚀 Key Features

- **On-Chain AI**: LSTM models run in Canisters using the [ic-burn](https://github.com/LintDAO/ic-burn) framework for chain-native inference.
- **One-Click Deployment**: Deploy multiple Canisters with pre-configured or custom models.
- **Token Staking**: Stake tokens to submit predictions and earn rewards.
- **Model Marketplace**: Trade user-created AI configurations with 2-5% platform fees.
- **Real-Time Data Streams**: Data Farmer streams provide predictive data for traders.
- **Community-Driven**: Crowdsourced intelligence improves model accuracy over time.

## 💬 User Story

**Data Farmer X** logs into PriceLint, deploys dozens of Canisters with a single click, and tests the accuracy of ICP price predictions by adjusting different model parameters.

X filters models with an accuracy rate of over 50%, stakes tokens to submit predictions, and earns rewards.

X begins optimizing model parameters to improve accuracy and shares the configuration on the platform market to earn referral commissions.

**Trader Y** pays for a subscription to aggregated forecast data streams to optimize his high-frequency trading strategy.

## 🛠️ Technical Architecture

### Frontend

- **Tech Stack**: Vue3, TypeScript, Pinia.
- **Features**: User-friendly interface for model selection, parameter tuning, token staking, and marketplace trading. Displays prediction leaderboards and annualized reward status.
- **ICP Utilization**: Internet Identity for secure, decentralized login; Retrieve real-time BTC prices from APIs (such as Binance and CoinGecko) via HTTPS outbound calls, and retrieve real-time ICP prices via XRC.

### Backend

- **Canisters (Rust)**:
  - **User Canister**: Ownership belongs to individual users, stores model configurations, staking records, and prediction results. Runs LSTM inference using [ic-burn](https://github.com/LintDAO/ic-burn), generating price predictions, hold tokens.
  - **Backend Canister**: Responsible for storing user information, aggregating data, and determining whether prediction results are correct.
  - **Market Canister**: Manages model trading, transaction records, and 2-5% fee distribution.


- **ICP Utilization**:
  - **Chain-Native AI**: WebAssembly (WASM) enables on-chain LSTM inference.
  - **Low-Cost Transactions**: Fixed fees (~10B cycles per model training) support high-frequency predictions.
  - **High Throughput**: Sub-second confirmation ensures real-time data updates.
  - **100% on-chain**: with smart contracts based on Canister implementing the entire process from model training to inference, token staking, and result determination, ensuring data sovereignty and transparency.
  - **ICRC-1 Tokens**: Support staking, rewards, and marketplace payments.

+ **ICP Features Used**
  + Full-stack decentralization.
  + Fetch real-time prices using  **HTTPS Outcalls** .
  + Authenticate and authorize using **Internet Identity**.
  + Using `ic-sdk-timer` to Periodic prediction submissions from users , scheduled collection and analysis of user-submitted results and automated statistical processing.
  + Using` ic-stable-structures` to persist data.
  + Exposing APIs to enable cross-canister calls
## Build and deployment instructions for local development
### 1. Environment Preparation

+ Install dfx toolchains
```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```


+ Dependencies
```bash
# install rust toolchains
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

### 2. Build canisters

+ Build backend
```bash
cd backend
cargo build --target wasm32-unknown-unknown --release -p backend
```

+ Build frontend
```bash
cd frontend
npm install
npm run build
```
### 3. Deploy on local

+ Start local IC replica
```bash
# start the local IC replica in background mode with a clean state
dfx start --background --clean
```

+ Deploy canisters
```bash
# deploy all canisters  on local
dfx deploy

# deploy only backend
dfx deploy backend

# deploy only frontend
dfx deploy frontend
```
### 4. Validate and tests


+ Command test
```bash
dfx canister status backend
```

+ Frontend interaction
```bash
npm run dev
# visit http://localhost:3000
```
## Mainnet canister ID(s)

+ Backend canister id: `eov5t-niaaa-aaaah-arepa-cai`
+ Frontend canister id: `lcdqt-cqaaa-aaaap-an2fq-cai`

