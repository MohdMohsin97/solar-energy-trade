# Solar Energy Trade

## Project Description

Solar Energy Trade is a decentralized application (dApp) built on the Stellar Blockchain that enables users to trade solar energy within their neighborhood. Sellers can list their surplus solar energy for sale, specifying the amount and price. Buyers can purchase this energy using a trade ID, transferring tokens directly to the seller. The platform ensures a secure and transparent transaction process, empowering individuals to manage and monetize their solar energy independently, without relying on centralized organizations.

## Vision Statement

Solar Energy Trade aims to revolutionize energy markets by decentralizing the control and distribution of solar energy. By empowering individuals to trade their surplus energy directly, the platform promotes sustainability and energy independence. It provides a seamless and transparent marketplace where local communities can benefit from renewable energy sources, reducing reliance on centralized energy providers and fostering a greener, more resilient energy ecosystem.

## Software Development Plan

1. **Smart Contract Development:**
   - **Function Definitions:**
     - `create`: Allows sellers to create a trade offer.
     - `buy_energy`: Enables buyers to purchase energy using the trade ID.
     - `withdraw_amount`: Allows sellers to withdraw tokens received from sales.
     - `get_trades`: Retrieves a list of all active trades.

   - **Variable Definitions:**
     - `trades`: A list to store active trades with details like seller, energy amount, price, and trade ID.
     - `balances`: A mapping to keep track of tokens owned by each address.

2. **Smart Contract Testing:**
   - Write and run unit tests to ensure all smart contract functions work as intended.
   - Simulate trade creation, buying energy, and withdrawing tokens.

3. **Front-End Development:**
   - **UI Design:**
     - Create user-friendly interfaces for sellers to list energy and for buyers to purchase energy.
   - **Integration:**
     - Connect the front-end with the smart contract functions using Stellar SDK.

## Personal Story Summary

Hi, I'm Mod Mohsin. Growing up in a neighborhood with frequent power outages, I realized the potential of solar energy to bring about energy independence. My passion for blockchain technology and sustainable energy led me to create Solar Energy Trade. This platform empowers communities to trade surplus solar energy directly, fostering a sustainable and resilient energy future without relying on centralized entities. By merging technology with green energy, I aim to make a positive impact on how we consume and share energy.