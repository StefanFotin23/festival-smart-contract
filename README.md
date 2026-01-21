# Festival Smart Contract

This document provides a comprehensive overview of the Festival Smart Contract project, presenting its logic and features.

### 1. High-Level Summary

The core purpose of this smart contract is to create a decentralized, transparent, and engaging platform for managing music festivals or similar events on the MultiversX blockchain. It handles the entire lifecycle of a ticket—from initial sale to resale on a secondary market—and incorporates enhanced user engagement through on-chain profiles and a gamified points system. A key enhancement is the introduction of distinct ticket types with specific validity rules and improved security measures.

### 2. Key Personas

There are two main roles that interact with your smart contract:

*   **Owner:** This is the deployer of the contract, responsible for all administrative tasks. The owner creates the festival, defines its parameters, adds events, and sets various ticket prices and sale periods. Their access is secured by the `#[only_owner]` attribute on specific functions.
*   **Participant/User:** This is any person who interacts with the public features of the contract. They can create a profile, buy different types of tickets, "check-in" to the festival using their ticket, and participate in the secondary ticket market under new restrictions.

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
The owner now defines ticket prices with specific **sales periods** (`sale_start_time`, `sale_end_time`) and **ticket types** (`FullPass` or `DayTicket`). Users can buy tickets only within these active sale periods. Each minted ticket NFT now has a dynamic name (e.g., "Summer Fest 2024 - Full Pass") and stores its type and associated `festival_id` in its attributes.
*   **Endpoints:** `addTicketPrice` (Owner), `buyTicket` (User)
*   **Storage Mappers:** `ticket_prices`, `festival_state` (tracks number of tickets sold), `ticket_token_identifier`
*   **Event:** `ticketBought` is emitted after a successful purchase.

#### Secondary Ticket Market (Resale)
A user who owns a ticket can list it for sale. However, a crucial new restriction is in place: **a ticket cannot be sold if it has already been used for check-in**. If a ticket has been used, it is "soul-bound" to the initial user.
*   **Endpoints:** `putTicketForSale`, `buyResaleTicket`
*   **Storage Mappers:** `resale_info` (tracks which tickets are for sale and their prices), `ticket_usage_data` (tracks used tickets)

#### Participant & Profile Management
Users can create a public profile with a username, making them an official participant in the festival ecosystem.
*   **Endpoint:** `createParticipant`
*   **Storage Mappers:** `user_list`, `user_name`
*   **Event:** `participantCreated` is emitted when a new profile is made.

#### Gamification
The contract includes features to reward user engagement.
*   **Check-in/Check-out:** The `checkIn` endpoint now requires the user to send their ticket NFT. On the very first check-in, the ticket is "soul-bound" to that user. For `DayTicket` types, a 24-hour validity window is enforced from the moment of first check-in; after this period, the ticket cannot be used to re-enter. `FullPass` tickets allow unlimited entry for the festival duration. The contract handles returning the NFT to the user after processing. Users can still `checkOut` to track their time spent at the festival, which is converted into points for their score.
*   **Flash Events:** Users can claim bonus points by calling `claimFlashEventPoints` during the active window of a flash event.
*   **Endpoints:** `checkIn`, `checkOut`, `claimFlashEventPoints`
*   **Storage Mappers:** `user_time_data`, `user_score`, `claimed_flash_events`, `ticket_usage_data`

### 4. Typical User Journey

Here’s a simple end-to-end example of how these features work together:

1.  **Owner Setup:** The owner first calls `set_ticket_token_identifier` to define the NFT collection for the tickets. They then call `addFestival` to create the event, `addEvent` to populate the schedule with artists, and `addTicketPrice` multiple times to set various ticket costs (e.g., a "Full Pass" with a long sales window and a "Day 1 Pass" with a short, specific sales window).
2.  **User Creates Profile:** A fan, Alex, calls `createParticipant` with the username "Alex" to join the festival community.
3.  **User Buys Ticket:** Alex calls the `buyTicket` endpoint, sending the required EGLD. The smart contract mints a ticket NFT (e.g., "Summer Fest 2024 - Full Pass") and sends it to Alex's wallet.
4.  **User Participates:** Upon arriving at the festival, Alex calls `checkIn`, sending their ticket NFT to the contract. Since it's the first time, the ticket becomes "soul-bound" to Alex. Later, a surprise "flash event" is announced, and Alex calls `claimFlashEventPoints` to earn bonus points. If Alex had a "Day Ticket," the 24-hour countdown for re-entry would begin here. When they leave, Alex calls `checkOut`, and their `user_score` is automatically updated based on the hours they were present. The ticket NFT is always returned to Alex after `checkIn` or `checkOut`.
5.  **User Resells Ticket:** Alex's "Full Pass" has been used once, so according to the new rules, Alex **cannot** call `putTicketForSale` to sell it. If Alex had an unused ticket, they could still sell it. Another user, Ben, can then buy an available ticket by calling `buyResaleTicket`. The EGLD is transferred to the original seller (minus the tax), and the ticket NFT is transferred to Ben's wallet.