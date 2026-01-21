import {
  
  Account,
  
  Address,
  
  AddressValue,
  
  BigUIntValue,
  
  BytesValue,
  
  ContractCallPayloadBuilder,
  
  ContractFunction,
  
  GasLimit,
  
  Interaction,
  
  NetworkConfig,
  
  ProxyProvider,
  
  Query,
  
  ResultsParser,
  
  SmartContract,
  
  Struct,
  
  StructTag,
  
  TokenIdentifierValue,
  
  Transaction, // Import Transaction
  
  TransactionWatcher,
  
  TypedValue,
  
  U64Value,
} from '@multiversx/sdk-core/out';
import {
  
  AbiRegistry,
  
  SmartContractAbi,
} from '@multiversx/sdk-abi-parser/out';
import { contractAddress, apiNetwork } from '../config/network';
import festivalAbi from '../config/festival.abi.json';

// ... (rest of the file remains the same until createParticipant)

export async function createParticipant(username: string): Promise<Transaction> {
  await initializeContract();
  try {
    const interaction = festivalContract.methods
      .createParticipant([new BytesValue(Buffer.from(username))])
      .withGasLimit(new GasLimit(60000000)) // Adjust gas limit as needed
      .withChainID(apiNetwork.chainId);

    return interaction.buildTransaction();
  } catch (error) {
    console.error('Error creating participant transaction:', error);
    throw error;
  }
}

// Placeholder for fetching leaderboard data
export async function getLeaderboard(): Promise<any[]> {
  console.log('Fetching leaderboard data...');
  return [];
}
