import { CasperClient, CLPublicKey, DeployUtil } from "casper-js-sdk";
import { Deploy } from "casper-js-sdk/dist/lib/DeployUtil";

import { NFT, triggerApproveSellDeploy } from "./cep47_utils";
import { MarketClient, MarketItem } from "./marketClient";
import { getDeploy } from "./utils";

export const NODE_ADDRESS =
  process.env.NEXT_PUBLIC_CASPER_NODE_ADDRESS ||
  "http://localhost:11100/http://mynctl:11101/rpc";
export const EVENT_STREAM_ADDRESS =
  process.env.NEXT_PUBLIC_CASPER_EVENT_STREAM_ADDRESS ||
  "http://localhost:11100/http://mynctl:18101/events/main";
export const CHAIN_NAME =
  process.env.NEXT_PUBLIC_CASPER_CHAIN_NAME || "casper-net-1";
export const MINT_ONE_PAYMENT_AMOUNT =
  process.env.NEXT_PUBLIC_CASPER_MINT_ONE_PAYMENT_AMOUNT || "2000000000";

// Create Casper client and service to interact with Casper node.
const casperClient = new CasperClient(NODE_ADDRESS);

export const initMarketClient = async () => {
  let marketClient;
  try {
    marketClient = new MarketClient(NODE_ADDRESS!, CHAIN_NAME!);
    const contractHash = process.env.NEXT_PUBLIC_MARKET_CONTRACT_HASH;
    const contractPackageHash =
      process.env.NEXT_PUBLIC_MARKET_CONTRACT_PACKAGE_HASH;

    marketClient.setContractHash(contractHash!, contractPackageHash!);
  } catch (e) {
    console.log(e);
  }
  return {
    marketClient,
  };
};

export function retrieveMarketTotalSupply() {
  // eslint-disable-next-line no-async-promise-executor
  return new Promise(async (resolve, reject) => {
    const timeout = setTimeout(reject, 10000);
    let marketClient;
    try {
      const { marketClient: client } = await initMarketClient();
      marketClient = client;
      // eslint-disable-next-line no-plusplus
    } catch (e) {
      console.log(e);
      reject();
    }
    if (!marketClient) reject();

    try {
      // @ts-ignore
      const totalSupply = await marketClient.totalSupply();
      clearTimeout(timeout);

      resolve(totalSupply);
    } catch (e) {
      console.log(e);
      reject();
    }
  });
}

export const triggerCreateMarketItemDeploy = async (
  item: NFT,
  amount: string
): Promise<boolean | null> => {
  // eslint-disable-next-line no-async-promise-executor
  return new Promise(async (resolve, reject) => {
    try {
      // @ts-ignore
      const { marketClient } = await initMarketClient();
      if (marketClient) {
        const publicKeyHex = await window.casperlabsHelper.getActivePublicKey();
        const activePublicKey = CLPublicKey.fromHex(publicKeyHex);

        const nftContractAddress = process.env.NEXT_PUBLIC_CEP47_CONTRACT_HASH!;
        // Currently only supporting one market contract on the front end
        // const nftContractAddresses = [].fill(nftContractAddress, 0, ids.length);
        const nftContractAddresses = [nftContractAddress.slice(5)];
        const marketItemId = await retrieveMarketTotalSupply();
        const deployItem = marketClient.createMarketItem(
          activePublicKey,
          [`${marketItemId}`],
          nftContractAddresses,
          [amount],
          [item.id || ""],
          MINT_ONE_PAYMENT_AMOUNT!,
          activePublicKey
        );
        // Turn your transaction data to format JSON
        const json = DeployUtil.deployToJson(deployItem);

        // Sign transcation using casper-signer.
        const signature = await window.casperlabsHelper.sign(
          json,
          publicKeyHex,
          publicKeyHex
        );
        const deployObject = DeployUtil.deployFromJson(signature);
        let deployItemHash;
        if (deployObject.val) {
          // Here we are sending the signed deploy.
          deployItemHash = await casperClient.putDeploy(
            deployObject.val as Deploy
          );
          console.log(`...... Create Market Item deployed: ${deployItemHash}`);

          await getDeploy(NODE_ADDRESS!, deployItemHash);

          // eslint-disable-next-line consistent-return
          resolve(true);
        }
      } else {
        console.log("Failed to init market client");
        reject();
      }
    } catch (e) {
      console.log(e);
      reject();
    }
  });
};

export function retrieveMarketName() {
  // eslint-disable-next-line no-async-promise-executor
  return new Promise(async (resolve, reject) => {
    const timeout = setTimeout(reject, 10000);
    let marketClient;
    try {
      const { marketClient: client } = await initMarketClient();
      marketClient = client;
      // eslint-disable-next-line no-plusplus
    } catch (e) {
      console.log(e);
      reject();
    }
    if (!marketClient) reject();

    try {
      // @ts-ignore
      const name = await marketClient.name();
      clearTimeout(timeout);

      resolve(name);
    } catch (e) {
      console.log(e);
      reject();
    }
  });
}

export function approveSell(item: NFT) {
  // eslint-disable-next-line no-async-promise-executor
  return new Promise(async (resolve, reject) => {
    const timeout = setTimeout(reject, 10000);
    let marketClient: MarketClient;
    try {
      const { marketClient: client } = await initMarketClient();
      // @ts-ignore
      marketClient = client;
      // eslint-disable-next-line unused-imports/no-unused-vars
    } catch (e) {
      console.log(e);
      reject();
    }

    try {
      // @ts-ignore
      const name = await marketClient.name();
      // @ts-ignore
      const marketItemHash = await marketClient.marketItemHash();
      console.log({ name });
      console.log({ marketItemHash });
      // @ts-ignore
      const approval = await triggerApproveSellDeploy(
        [item.id],
        // @ts-ignore
        marketItemHash
      );
      console.log({ approval });
      clearTimeout(timeout);

      resolve(true);
    } catch (e) {
      console.log(e);
      reject();
    }
  });
}

export function getMarketItem(item: NFT): Promise<MarketItem | null> {
  // eslint-disable-next-line no-async-promise-executor
  return new Promise(async (resolve, reject) => {
    // const timeout = setTimeout(reject, 10000);
    let marketClient;
    try {
      const { marketClient: client } = await initMarketClient();
      marketClient = client;
      // eslint-disable-next-line no-plusplus
    } catch (e) {
      console.log(e);
      reject();
    }
    if (!marketClient) reject();

    try {
      // @ts-ignore
      const marketItemIds = await marketClient.getMarketItemIds(item.id);
      console.log({ marketItemIds });
      // clearTimeout(timeout);
      const lastItem = marketItemIds[marketItemIds.length - 1];
      if (lastItem) {
        // @ts-ignore
        const status = await marketClient.getMarketItemStatus(lastItem);
        console.log({ status });

        // @ts-ignore
        const askingPrice = await marketClient.getMarketItemPrice(lastItem);
        console.log({ askingPrice });

        // @ts-ignore
        const approvalHash = await marketClient.marketItemHash();

        const marketItem: MarketItem = {
          ...item,
          available: status === "available",
          askingPrice,
          approvalHash,
        };
        resolve(marketItem);
      } else {
        resolve(null);
      }
    } catch (e) {
      if (e) {
        console.log(e);
      }
      resolve(null);
    }
  });
}
