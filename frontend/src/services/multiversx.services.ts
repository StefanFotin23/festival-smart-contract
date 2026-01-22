import {
  Account,
  Address,
  AddressValue,
  BigUIntValue,
  BytesValue,
  ContractFunction,
  Query,
  SmartContract,
  AbiRegistry,
  SmartContractAbi,
  Transaction,
  U64Value,
} from '@multiversx/sdk-core';
import { ApiNetworkProvider } from '@multiversx/sdk-network-providers';
import { contractAddress, apiNetwork } from '../config/network';
import festivalAbi from '../config/festival.abi.json';

// Use ApiNetworkProvider for network requests
const apiProvider = new ApiNetworkProvider(apiNetwork.apiAddress, {
  timeout: apiNetwork.apiTimeout,
});

let contract: SmartContract;
let abi: AbiRegistry;

// Use an async function to initialize and memoize the contract and ABI
export async function getContract() {
  if (contract && abi) {
    return { contract, abi };
  }
  try {
    abi = AbiRegistry.create(festivalAbi as any);
    contract = new SmartContract({
      address: new Address(contractAddress),
      abi: abi,
    });
    return { contract, abi };
  } catch (error) {
    console.error('Error creating contract from ABI', error);
    throw error;
  }
}

export async function getFestivalData(festivalId: number): Promise<any> {
  try {
    const { contract, abi } = await getContract();
    const query = new Query({
      address: contract.getAddress(),
      func: new ContractFunction('getFestivalData'),
      args: [new U64Value(festivalId)],
    });

    const queryResponse = await apiProvider.query(query);
    const endpoint = abi.getEndpoint('getFestivalData');
    const { firstValue } = new ResultsParser().parseQueryResponse(
      queryResponse,
      endpoint
    );

    const data = firstValue?.valueOf();
    if (!data) return null;

    return {
      name: data.field0.toString(),
      startTime: data.field1.toNumber(),
      endTime: data.field2.toNumber(),
      maxTickets: data.field3.toNumber(),
      soldTickets: data.field4.toNumber(),
      insideNow: data.field5.toNumber(),
    };
  } catch (error) {
    console.error(`Error fetching festival data for ID ${festivalId}:`, error);
    return null;
  }
}

export async function getAllFestivals(): Promise<any[]> {
  try {
    const { contract, abi } = await getContract();
    const query = new Query({
      address: contract.getAddress(),
      func: new ContractFunction('festivalCount'),
    });

    const queryResponse = await apiProvider.query(query);
    const endpoint = abi.getEndpoint('festivalCount');
    const { firstValue } = new ResultsParser().parseQueryResponse(
      queryResponse,
      endpoint
    );
    const festivalCount = firstValue?.valueOf()?.toNumber() || 0;

    const festivals = [];
    for (let i = 1; i <= festivalCount; i++) {
      const festival = await getFestivalData(i);
      if (festival) {
        festivals.push({ id: i, ...festival });
      }
    }
    return festivals;
  } catch (error) {
    console.error('Error fetching all festivals:', error);
    return [];
  }
}

// This function is now handled by TransactionFactory, but we keep the file for queries.
// If you want to merge them, you can move createParticipant logic from TransactionFactory here,
// but using the Interaction builder pattern.
// For now, we'll leave this file for queries only to maintain separation of concerns.
// You can remove the createParticipant export if it's not used elsewhere.
/*
export async function createParticipant(username: string): Promise<Transaction> {
  const { contract } = await getContract();
  const interaction = contract.methods
    .createParticipant([new BytesValue(Buffer.from(username))])
    .withGasLimit(60000000)
    .withChainID(apiNetwork.chainId);

  return interaction.buildTransaction();
}
*/

// Temporary re-add of ResultsParser until a better pattern is found for parsing
class ResultsParser {
    parseQueryResponse(queryResponse, endpoint) {
        const bundle = endpoint.decodeOutput(queryResponse.getReturnData());
        return {
            firstValue: bundle.firstValue,
            secondValue: bundle.secondValue,
            allValues: bundle.values,
        };
    }
}
