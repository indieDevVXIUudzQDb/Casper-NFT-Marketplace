import { CasperClient, CLPublicKey, DeployUtil } from "casper-js-sdk";
import { Deploy } from "casper-js-sdk/dist/lib/DeployUtil";
import { MARKETClient } from "./marketClient";

export const NODE_ADDRESS =
  process.env.NEXT_PUBLIC_CASPER_NODE_ADDRESS ||
  "http://localhost:11100/http://mynctl:11101/rpc";
export const EVENT_STREAM_ADDRESS =
  process.env.NEXT_PUBLIC ||
  "http://localhost:11100/http://mynctl:18101/events/main";
export const CHAIN_NAME = process.env.NEXT_PUBLIC || "casper-net-1";
export const MINT_ONE_PAYMENT_AMOUNT = process.env.NEXT_PUBLIC || "2000000000";

// TODO make following dynamic
export const CONTRACT_NAME = "doggy_contract";
export const CONTRACT_HOLDER_ADDRESS =
  "0146c64d0506c486f2b19f9cf73479fba550f33227b6ec1c12e58b437d2680e96d";
// Create Casper client and service to interact with Casper node.
// const casperUtils = new CasperServiceByJsonRPC(NODE_ADDRESS);
const casperClient = new CasperClient(NODE_ADDRESS);

export const initClient = async () => {
  let marketClient;
  let contractPublicKey;
  try {
    contractPublicKey = CLPublicKey.fromHex(CONTRACT_HOLDER_ADDRESS);
    marketClient = new MARKETClient(NODE_ADDRESS!, CHAIN_NAME!);
    const contractHash = process.env.NEXT_PUBLIC_MARKET_CONTRACT_HASH;
    const contractPackageHash =
      process.env.NEXT_PUBLIC_MARKET_CONTRACT_PACKAGE_HASH;

    marketClient.setContractHash(contractHash!, contractPackageHash!);
  } catch (e) {
    console.log(e);
  }
  return {
    marketClient,
    contractPublicKey,
  };
};

// const getDeployResult = (deployHash: string) => {
//   // eslint-disable-next-line no-async-promise-executor
//   return new Promise(async (resolve, reject) => {
//     const timeout = setTimeout(reject, 10000);
//     try {
//       // @ts-ignore
//       const { cep47 } = await initClient();
//       if (!cep47) reject();
//
//       await getDeploy(
//         process.env.NEXT_PUBLIC_CASPER_NODE_ADDRESS!!,
//         deployHash
//       );
//       console.log("...... Deployed successfully");
//       clearTimeout(timeout);
//       resolve(deployHash);
//     } catch (e) {
//       console.log(e);
//       reject();
//     }
//   });
// };

export const triggerCreateMarketItemDeploy = async (
  ids: string[]
): Promise<unknown> => {
  return new Promise(async (resolve, reject) => {
    try {
      // @ts-ignore
      const { marketClient } = await initClient();
      if (marketClient) {
        const publicKeyHex = await window.casperlabsHelper.getActivePublicKey();
        const activePublicKey = CLPublicKey.fromHex(publicKeyHex);

        const nftContractAddress = process.env.NEXT_PUBLIC_CEP47_CONTRACT_HASH!;
        // Currently only supporting one market contract on the front end
        // const nftContractAddresses = [].fill(nftContractAddress, 0, ids.length);
        const nftContractAddresses = [nftContractAddress.slice(5)];
        console.log(nftContractAddress, nftContractAddresses);
        const deployItem = marketClient.createMarketItem(
          activePublicKey,
          ids,
          nftContractAddresses,
          ["5000000"],
          ["0"],
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
          console.log({ ids });
          // eslint-disable-next-line consistent-return
          resolve(deployItemHash);
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
      const { marketClient: client } = await initClient();
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
