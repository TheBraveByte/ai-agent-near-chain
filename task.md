# Blockchain Yield Optimization Application on NEAR


## select the one you will like to work on and let me know


## Overview
The project involves building a blockchain application on NEAR that automatically finds the best yield for the user by monitoring feeds from various protocols (Aave, Compound, etc.) and automatically moving funds to the protocol offering the best yield using AI agents like Bitte.

## Key Components
1. **Blockchain Layer**:
   - Smart contract on NEAR to handle the movement of funds and interaction with other protocols (Aave, Compound, etc.).
   
2. **Yield Monitoring**:
   - A system that monitors yield rates across various protocols and fetches the latest data from their feeds.
   
3. **AI Agent (e.g., Bitte)**:
   - AI agents to decide when to move funds to the protocol offering the best yield.
   
4. **User Interface**:
   - Frontend for users to interact with the system, view yield performance, and set preferences.

## Task Division

### Developer 1: Backend and Blockchain Development
1. **Blockchain Interaction**:
   - Develop the core smart contract on the NEAR blockchain that allows interaction with other protocols like Aave, Compound, and other DeFi platforms.
   - Handle fund deposits, withdrawals, and movements to protocols with the best yield.
   - Implement logic to track the best yield and make decisions based on real-time data.

2. **AI Agent Development**:
   - Integrate AI agents like Bitte to automate the process of moving funds between protocols.
   - Build the logic for the agent to analyze yield rates and decide on fund movement.

3. **Integration with Protocol Feeds**:
   - Set up mechanisms to fetch yield rates from Aave, Compound, and other platforms using their APIs or oracles.
   - Implement monitoring and trigger mechanisms for identifying the best yield.

### Developer 2: Frontend and User Interaction
1. **User Interface**:
   - Develop a frontend application where users can connect their wallet, view their funds, and monitor yield rates across protocols.
   - Design user-friendly dashboards to display real-time yield comparisons and automated actions taken by the AI agents.

2. **User Settings and Preferences**:
   - Allow users to set preferences, such as the threshold for yield differences that trigger fund movements or risk levels they are comfortable with.
   - Provide a UI to enable users to pause or resume the automatic yield optimization.

3. **AI Agent Monitoring**:
   - Build an interface to visualize AI agent activity, showing what actions the agent is performing, and offer transparency to the user.

## Steps to Proceed
1. **Set up the NEAR Smart Contract**:
   - Deploy a smart contract on the NEAR blockchain. Use NEAR SDKs and possibly integrate Chainlink or other oracles to fetch yield data.
   
2. **Integrate Yield Monitoring**:
   - Fetch yield data from Aave, Compound, and other protocols, either directly from their APIs or through decentralized oracle solutions.
   
3. **Implement AI Agents**:
   - Integrate AI agents like Bitte to handle the decision-making process regarding fund movement. Use their SDK to interact with protocols and automate the decision-making based on yield performance.
   
4. **Build Frontend**:
   - Design and develop a web application to allow users to interact with the smart contract and monitor their yield. Use frameworks like React, Next.js, or Vue.js for the frontend.

5. **Testing and Deployment**:
   - Thoroughly test the system for edge cases, such as sudden yield fluctuations, low liquidity, or failed transactions.
   - Deploy the smart contract on NEAR mainnet and ensure the frontend is connected properly.

## Conclusion
By splitting tasks between the two developers, you can efficiently handle both the backend (smart contract and AI agent logic) and frontend (user interface and interaction). This allows for parallel work while focusing on each developer's area of expertise.
