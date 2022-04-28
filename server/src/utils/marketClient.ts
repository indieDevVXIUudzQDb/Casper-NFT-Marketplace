import {
  CasperClient,
  CLMap,
  CLPublicKey,
  CLTypeTag,
  CLValue,
  CLValueBuilder,
  CLValueParsers,
  Contracts,
  Keys,
  RuntimeArgs,
} from "casper-js-sdk";
import { contractHashToByteArray } from "./utils";
import { NFT } from "./cep47_utils";

const { Contract, toCLMap } = Contracts;

export interface MARKETInstallArgs {
  marketName: string;
  contractName: string;
  marketSymbol: string;
  marketMeta: Map<string, string>;
}

export const toAccountHashString = (hash: Uint8Array) =>
  Buffer.from(hash).toString("hex");

export enum MARKETEvents {}

export const MARKETEventParser = (
  {
    contractPackageHash,
    eventNames,
  }: { contractPackageHash: string; eventNames: MARKETEvents[] },
  value: any
) => {
  if (value.body.DeployProcessed.execution_result.Success) {
    const { transforms } =
      value.body.DeployProcessed.execution_result.Success.effect;

    const cep47Events = transforms.reduce((acc: any, val: any) => {
      if (
        // eslint-disable-next-line no-prototype-builtins
        val.transform.hasOwnProperty("WriteCLValue") &&
        typeof val.transform.WriteCLValue.parsed === "object" &&
        val.transform.WriteCLValue.parsed !== null
      ) {
        const maybeCLValue = CLValueParsers.fromJSON(
          val.transform.WriteCLValue
        );
        const clValue = maybeCLValue.unwrap();
        if (clValue && clValue.clType().tag === CLTypeTag.Map) {
          const hash = (clValue as CLMap<CLValue, CLValue>).get(
            CLValueBuilder.string("contract_package_hash")
          );
          const event = (clValue as CLMap<CLValue, CLValue>).get(
            CLValueBuilder.string("event_type")
          );
          if (
            hash &&
            // NOTE: Calling toLowerCase() because current JS-SDK doesn't support checksumed hashes and returns all lower case value
            // Remove it after updating SDK
            hash.value() === contractPackageHash.slice(5).toLowerCase() &&
            event &&
            eventNames.includes(event.value())
          ) {
            // eslint-disable-next-line no-param-reassign
            acc = [...acc, { name: event.value(), clValue }];
          }
        }
      }
      return acc;
    }, []);

    return { error: null, success: !!cep47Events.length, data: cep47Events };
  }

  return null;
};

// const keyAndValueToHex = (key: CLValue, value: CLValue) => {
//   const aBytes = CLValueParsers.toBytes(key).unwrap();
//   const bBytes = CLValueParsers.toBytes(value).unwrap();
//
//   const blaked = blake.blake2b(concat([aBytes, bBytes]), undefined, 32);
//   const hex = Buffer.from(blaked).toString("hex");
//
//   return hex;
// };

export interface MarketItem extends NFT {
  isApproved: boolean;
  available: boolean;
  askingPrice: string;
  approvalHash: any;
  marketItemId: string;
}

export class MarketClient {
  casperClient: CasperClient;

  contractClient: Contracts.Contract;

  constructor(public nodeAddress: string, public networkName: string) {
    this.casperClient = new CasperClient(nodeAddress);
    this.contractClient = new Contract(this.casperClient);
  }

  public install(
    wasm: Uint8Array,
    args: MARKETInstallArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      market_name: CLValueBuilder.string(args.marketName),
      market_symbol: CLValueBuilder.string(args.marketSymbol),
      market_meta: toCLMap(args.marketMeta),
      contract_name: CLValueBuilder.string(args.contractName),
    });

    return this.contractClient.install(
      wasm,
      runtimeArgs,
      paymentAmount,
      deploySender,
      this.networkName,
      keys || []
    );
  }

  public setContractHash(contractHash: string, contractPackageHash?: string) {
    this.contractClient.setContractHash(contractHash, contractPackageHash);
  }

  public async name() {
    return this.contractClient.queryContractData(["market_name"]);
  }

  public async totalSupply() {
    const result = await this.contractClient.queryContractData([
      "item_total_supply",
    ]);
    // @ts-ignore
    return result.toNumber();
  }

  public async getMarketItemIds(itemTokenId: string) {
    const result = await this.contractClient.queryContractDictionary(
      "nft_market_item_ids",
      itemTokenId
    );

    const maybeValue = result.value().unwrap();
    const values = maybeValue
      .value()
      .map((value: CLValue) => value.data.toString());
    return values;
  }

  public async getMarketItemStatus(itemId: string) {
    const result = await this.contractClient.queryContractDictionary(
      "item_statuses",
      itemId
    );
    return result.value().unwrap().data;
  }

  public async getMarketItemPrice(itemId: string) {
    const result = await this.contractClient.queryContractDictionary(
      "item_asking_prices",
      itemId
    );
    const value = result.value().unwrap();
    return value.data.toNumber();
  }

  public async marketItemHash() {
    // Used for approving market contract
    const result = await this.contractClient.queryContractData([
      "market_item_hash",
    ]);
    // return toAccountHashString(result.data);
    return result;
  }

  public createMarketItem(
    recipient: CLPublicKey,
    itemIds: string[],
    itemNFTContractAddresses: string[],
    itemAskingPrices: string[],
    itemTokenIds: string[],
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const contractHashAsByteArrays = itemNFTContractAddresses.map((value) =>
      // utils.utils.contractHashToByteArray(value)
      contractHashToByteArray(value)
    );
    const args = {
      itemIds,
      itemNFTContractAddresses,
      itemAskingPrices,
      itemTokenIds,
      paymentAmount,
    };
    window.createArgs = args;
    console.log({
      itemIds,
      itemNFTContractAddresses,
      itemAskingPrices,
      itemTokenIds,
      paymentAmount,
    });
    const runtimeArgs = RuntimeArgs.fromMap({
      recipient: CLValueBuilder.key(recipient),
      item_ids: CLValueBuilder.list(
        itemIds.map((value) => CLValueBuilder.u256(value))
      ),
      item_nft_contract_addresses: CLValueBuilder.list(
        contractHashAsByteArrays.map((value) => CLValueBuilder.byteArray(value))
      ),
      item_asking_prices: CLValueBuilder.list(
        itemAskingPrices.map((value) => CLValueBuilder.u512(value))
      ),
      item_token_ids: CLValueBuilder.list(
        itemTokenIds.map((value) => CLValueBuilder.u256(value))
      ),
    });

    return this.contractClient.callEntrypoint(
      "create_market_item",
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );
  }
  private getBinary = async (binaryUrl: string): Promise<Uint8Array> => {
    return new Promise((resolve, reject) => {
      try {
        fetch(binaryUrl)
          .then((r) => {
            window.r = r;
            return r.arrayBuffer();
          })
          .then((t) => {
            window.t = t;
            return resolve(new Uint8Array(t));
          });
      } catch (e) {
        console.log(e);
        reject();
      }
    });
  };

  public async processMarketSale(
    recipient: CLPublicKey,
    itemId: string,
    askingAmount: string,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const marketHash = process.env.NEXT_PUBLIC_MARKET_CONTRACT_HASH!;
    const marketHashAsByteArray = contractHashToByteArray(marketHash.slice(5));

    const binary = await this.getBinary(
      "http://localhost:3000/assets/bin/market-offer-purse.wasm"
    );
    console.log({ binary });

    const args = {
      recipient,
      itemId,
      askingAmount,
      marketHashAsByteArray,
    };
    window.buyArgs = args;
    console.log("processMarketSale", {
      recipient,
      itemId,
      askingAmount,
      marketHashAsByteArray,
    });
    const runtimeArgs = RuntimeArgs.fromMap({
      recipient: CLValueBuilder.key(recipient),
      item_id: CLValueBuilder.u256(itemId),
      amount: CLValueBuilder.u512(askingAmount),
      market_contract_hash: CLValueBuilder.byteArray(marketHashAsByteArray),
    });

    return this.contractClient.install(
      binary,
      runtimeArgs,
      paymentAmount,
      deploySender,
      this.networkName,
      keys
    );
  }
}
