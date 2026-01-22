import {
  Address,
  Transaction,
  TokenTransfer,
} from '@multiversx/sdk-core';

const CONFIG = {
  CONTRACT_ADDRESS: 'erd1qqqqqqqqqqqqqpgq7kpf8d4eyy6umdgeug8la0ss64uxeg4cn2jsuxvsq2',
  TICKET_COLLECTION: 'FEST-c51ed4', // Using the Token ID from your Makefile
  CHAIN_ID: 'D', // 'D' for Devnet, '1' for Mainnet
};

// Helper to encode strings to Hex
const toHex = (str: string) => Buffer.from(str, 'utf-8').toString('hex');
// Helper to encode numbers to Hex (even length)
const numToHex = (num: number) => {
  let hex = num.toString(16);
  if (hex.length % 2 !== 0) hex = '0' + hex;
  return hex;
};

export const TransactionFactory = {
  /**
   * 1. CREATE PARTICIPANT
   */
  createParticipant: (senderAddress: string, username: string) => {
    return new Transaction({
      value: TokenTransfer.egldFromAmount(0),
      data: `createParticipant@${toHex(username)}`,
      receiver: new Address(CONFIG.CONTRACT_ADDRESS),
      gasLimit: 10000000n,
      sender: new Address(senderAddress),
      chainID: CONFIG.CHAIN_ID,
    });
  },

  /**
   * 2. BUY TICKET
   */
  buyTicket: (
    senderAddress: string,
    festivalId: number,
    ticketName: string,
    priceInEgld: number
  ) => {
    const args = [numToHex(festivalId), toHex(ticketName)];
    return new Transaction({
      value: TokenTransfer.egldFromAmount(priceInEgld),
      data: `buyTicket@${args.join('@')}`,
      receiver: new Address(CONFIG.CONTRACT_ADDRESS),
      gasLimit: 20000000n,
      sender: new Address(senderAddress),
      chainID: CONFIG.CHAIN_ID,
    });
  },

  /**
   * 3. CHECK IN (Use Ticket via ESDTNFTTransfer)
   */
  checkIn: (senderAddress: string, ticketNonce: number) => {
    const tokenHex = toHex(CONFIG.TICKET_COLLECTION);
    const nonceHex = numToHex(ticketNonce);
    const amountHex = '01';
    const destinationHex = new Address(CONFIG.CONTRACT_ADDRESS).hex();
    const funcHex = toHex('checkIn');
    const payload = `ESDTNFTTransfer@${tokenHex}@${nonceHex}@${amountHex}@${destinationHex}@${funcHex}`;

    return new Transaction({
      value: TokenTransfer.egldFromAmount(0),
      data: payload,
      receiver: new Address(senderAddress),
      gasLimit: 20000000n,
      sender: new Address(senderAddress),
      chainID: CONFIG.CHAIN_ID,
    });
  },

  /**
   * 4. PUT TICKET FOR SALE (Resale via ESDTNFTTransfer)
   */
  sellTicket: (
    senderAddress: string,
    ticketNonce: number,
    askingPriceEgld: number
  ) => {
    const tokenHex = toHex(CONFIG.TICKET_COLLECTION);
    const nonceHex = numToHex(ticketNonce);
    const amountHex = '01';
    const destinationHex = new Address(CONFIG.CONTRACT_ADDRESS).hex();
    const funcHex = toHex('putTicketForSale');

    const priceWei = TokenTransfer.egldFromAmount(askingPriceEgld).valueOf(); // BigInt
    let priceHex = priceWei.toString(16);
    if (priceHex.length % 2 !== 0) priceHex = '0' + priceHex;

    const payload = `ESDTNFTTransfer@${tokenHex}@${nonceHex}@${amountHex}@${destinationHex}@${funcHex}@${priceHex}`;

    return new Transaction({
      value: TokenTransfer.egldFromAmount(0),
      data: payload,
      receiver: new Address(senderAddress),
      gasLimit: 20000000n,
      sender: new Address(senderAddress),
      chainID: CONFIG.CHAIN_ID,
    });
  },

  /**
   * 5. CHECK OUT
   */
  checkOut: (senderAddress: string, festivalId: number) => {
    return new Transaction({
      value: TokenTransfer.egldFromAmount(0),
      data: `checkOut@${numToHex(festivalId)}`,
      receiver: new Address(CONFIG.CONTRACT_ADDRESS),
      gasLimit: 10000000n,
      sender: new Address(senderAddress),
      chainID: CONFIG.CHAIN_ID,
    });
  },

  /**
   * 6. CLAIM FLASH EVENT POINTS
   */
  claimFlashPoints: (
    senderAddress: string,
    festivalId: number,
    flashEventIndex: number
  ) => {
    const args = [numToHex(festivalId), numToHex(flashEventIndex)];
    return new Transaction({
      value: TokenTransfer.egldFromAmount(0),
      data: `claimFlashEventPoints@${args.join('@')}`,
      receiver: new Address(CONFIG.CONTRACT_ADDRESS),
      gasLimit: 10000000n,
      sender: new Address(senderAddress),
      chainID: CONFIG.CHAIN_ID,
    });
  },

  /**
   * 7. BUY RESALE TICKET
   */
  buyResaleTicket: (
    senderAddress: string,
    ticketNonce: number,
    priceInEgld: number
  ) => {
    return new Transaction({
      value: TokenTransfer.egldFromAmount(priceInEgld),
      data: `buyResaleTicket@${numToHex(ticketNonce)}`,
      receiver: new Address(CONFIG.CONTRACT_ADDRESS),
      gasLimit: 20000000n,
      sender: new Address(senderAddress),
      chainID: CONFIG.CHAIN_ID,
    });
  },
};
