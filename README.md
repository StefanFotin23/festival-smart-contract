# Festival Smart Contract

This document provides a comprehensive overview of the Festival Smart Contract project, presenting its logic and features.

### 1. High-Level Summary

The core purpose of this smart contract is to create a decentralized, transparent, and engaging platform for managing music festivals or similar events on the MultiversX blockchain. It handles the entire lifecycle of a ticket—from initial sale to resale on a secondary market—and adds a layer of user engagement through on-chain profiles and a gamified points system.

### 2. Key Personas

There are two main roles that interact with your smart contract:

*   **Owner:** This is the deployer of the contract, responsible for all administrative tasks. The owner creates the festival, defines its parameters, adds events, and sets ticket prices. Their access is secured by the `#[only_owner]` attribute on specific functions.
*   **Participant/User:** This is any person who interacts with the public features of the contract. They can create a profile, buy tickets, "check-in" to the festival to track their time, and participate in the secondary ticket market.

### 3. Core Features & Code Mapping

Here is a breakdown of the main features and how they are implemented in the code:

#### Festival Management
The owner can create a new festival, defining its name, duration, total ticket capacity, and the tax percentage for resold tickets.
*   **Endpoint:** `addFestival`
*   **Storage Mappers:** `festival_count`, `festival_name`, `festival_config`, `festival_tax`

#### Event & Schedule Management
The owner can add regular events (like artist performances) and special, time-limited "flash events" to the festival's schedule.
*   **Endpoints:** `addEvent`, `addFlashEvent`
*   **Storage Mappers:** `events`, `flash_events`

#### Ticket Sales (Primary Market)
The owner sets the prices for different ticket types or sales phases. Users can then buy these tickets directly from the contract by sending the correct amount of EGLD. The contract then mints a brand-new ticket NFT and sends it to the buyer.
*   **Endpoints:** `addTicketPrice` (Owner), `buyTicket` (User)
*   **Storage Mappers:** `ticket_prices`, `festival_state` (tracks number of tickets sold), `ticket_token_identifier`
*   **Event:** `ticketBought` is emitted after a successful purchase.

#### Secondary Ticket Market (Resale)
A user who owns a ticket can list it for sale. They do this by sending their ticket NFT to the contract, which holds it in escrow, and setting a price. Another user can then buy this ticket by paying the listed EGLD price. The contract automatically handles the distribution of funds (sending the payment to the seller, minus a tax for the owner) and transfers the NFT to the new buyer.
*   **Endpoints:** `putTicketForSale`, `buyResaleTicket`
*   **Storage Mapper:** `resale_info` (tracks which tickets are for sale and their prices)

#### Participant & Profile Management
Users can create a public profile with a username, making them an official participant in the festival ecosystem.
*   **Endpoint:** `createParticipant`
*   **Storage Mappers:** `user_list`, `user_name`
*   **Event:** `participantCreated` is emitted when a new profile is made.

#### Gamification
The contract includes features to reward user engagement.
*   **Check-in/Check-out:** Users can call `checkIn` when they arrive and `checkOut` when they leave. The contract calculates the total time they spent at the festival and converts it into points for their score.
*   **Flash Events:** Users can claim bonus points by calling `claimFlashEventPoints` during the active window of a flash event.
*   **Endpoints:** `checkIn`, `checkOut`, `claimFlashEventPoints`
*   **Storage Mappers:** `user_time_data`, `user_score`, `claimed_flash_events`

### 4. Typical User Journey

Here’s a simple end-to-end example of how these features work together:

1.  **Owner Setup:** The owner first calls `set_ticket_token_identifier` to define the NFT collection for the tickets. They then call `addFestival` to create the event, `addEvent` to populate the schedule with artists, and `addTicketPrice` to set the ticket costs.
2.  **User Creates Profile:** A fan, Alex, calls `createParticipant` with the username "Alex" to join the festival community.
3.  **User Buys Ticket:** Alex calls the `buyTicket` endpoint, sending the required EGLD. The smart contract mints a ticket NFT and sends it to Alex's wallet.
4.  **User Participates:** Upon arriving at the festival, Alex calls `checkIn`. Later, a surprise "flash event" is announced, and Alex calls `claimFlashEventPoints` to earn bonus points. When they leave, Alex calls `checkOut`, and their `user_score` is automatically updated based on the hours they were present.
5.  **User Resells Ticket:** Alex can't attend the second day and decides to sell the ticket. Alex calls `putTicketForSale`, sending the ticket NFT to the contract and setting a price. Another user, Ben, sees the listing and calls `buyResaleTicket`. The EGLD is transferred to Alex (minus the tax), and the ticket NFT is transferred to Ben's wallet.