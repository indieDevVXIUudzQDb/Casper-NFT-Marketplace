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
import {concat} from "@ethersproject/bytes";
import blake from "blakejs";

const { Contract, toCLMap, fromCLMap } = Contracts;

export interface MARKETInstallArgs {
  marketName: string;
  contractName: string;
  marketSymbol: string;
  marketMeta: Map<string, string>;
}

export enum MARKETEvents {
}

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

const keyAndValueToHex = (key: CLValue, value: CLValue) => {
  const aBytes = CLValueParsers.toBytes(key).unwrap();
  const bBytes = CLValueParsers.toBytes(value).unwrap();

  const blaked = blake.blake2b(concat([aBytes, bBytes]), undefined, 32);
  const hex = Buffer.from(blaked).toString("hex");

  return hex;
};

export class MARKETClient {
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

}
