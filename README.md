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

**Trader Y** pays to subscribe to X's prediction data stream to optimize high-frequency trading strategies.

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

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

### Start the Frontend

```bash
npm install
npm run dev
```
