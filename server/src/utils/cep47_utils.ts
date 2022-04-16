// Tutorial
// https://casper.network/docs/dapp-dev-guide/tutorials/casper-signer/

import {
  CasperClient,
  CasperServiceByJsonRPC,
  CLPublicKey,
  DeployUtil,
  EventName,
  EventStream,
} from "casper-js-sdk";
import { Deploy } from "casper-js-sdk/dist/lib/DeployUtil";
import {
  CEP47Client,
  CEP47EventParser,
  CEP47Events,
} from "casper-cep47-js-client";
import { getAccountInfo, getAccountNamedKeyValue, getDeploy } from "./utils";

export const NODE_ADDRESS = process.env.NEXT_PUBLIC_CASPER_NODE_ADDRESS || "http://localhost:11100/http://mynctl:11101/rpc";
export const EVENT_STREAM_ADDRESS = process.env.NEXT_PUBLIC || "http://localhost:11100/http://mynctl:18101/events/main";
export const CHAIN_NAME = process.env.NEXT_PUBLIC || "casper-net-1";
export const MINT_ONE_PAYMENT_AMOUNT = process.env.NEXT_PUBLIC || "2000000000";

//TODO make following dynamic
export const CONTRACT_NAME = "doggy_contract";
export const CONTRACT_HOLDER_ADDRESS =
  "0146c64d0506c486f2b19f9cf73479fba550f33227b6ec1c12e58b437d2680e96d";
//Create Casper client and service to interact with Casper node.
const cep47_utils = new CasperServiceByJsonRPC(NODE_ADDRESS);
const casperClient = new CasperClient(NODE_ADDRESS);

export const subscribeToContractEvents = (
  es: EventStream,
  callback: () => void
) => {
  const contractPackageHash =
    "hash-998700a74b6107443ee1ddbb35286f2e2c7f2629ea18e9f545d448b7d2f5a0d8";

  es.subscribe(EventName.DeployProcessed, (event) => {
    const parsedEvents = CEP47EventParser(
      {
        contractPackageHash,
        eventNames: [
          CEP47Events.MintOne,
          CEP47Events.TransferToken,
          CEP47Events.BurnOne,
          CEP47Events.MetadataUpdate,
          CEP47Events.ApproveToken,
        ],
      },
      event
    );

    // if (parsedEvents && parsedEvents.success) {
    if (parsedEvents) {
      console.log("*** EVENT ***");
      console.log(parsedEvents.data);
      console.log("*** ***");
      callback();
    }
  });
  try {
    es.start();
  } catch (e) {
    console.log(e);
  }
};

export const accountInformation = async (): Promise<{
  textAddress: string;
  textBalance: string;
  publicKey: string;
}> => {
  let textAddress = "";
  let textBalance = "";
  let publicKey = "";
  const isConnected = await window.casperlabsHelper.isConnected();
  console.log({ isConnected });
  if (isConnected) {
    const publicKey = await window.casperlabsHelper.getActivePublicKey();
    textAddress = publicKey;

    const latestBlock = await cep47_utils.getLatestBlockInfo();
    console.log({ latestBlock });
    if (latestBlock.block) {
      const root = await cep47_utils.getStateRootHash(latestBlock.block.hash);
      const balanceUref = await cep47_utils.getAccountBalanceUrefByPublicKey(
        root,
        CLPublicKey.fromHex(publicKey)
      );
      //account balance from the last block
      const balance = await cep47_utils.getAccountBalance(
        latestBlock.block.header.state_root_hash,
        balanceUref
      );
      textBalance = `${balance.toString()}`;
    }
  }
  return {
    textAddress,
    textBalance,
    publicKey,
  };
};

export const sendTransaction = async (
  to: string,
  amount: string
): Promise<string> => {
  let tx = "";
  // For native-transfers the payment price is fixed.
  const paymentAmount = 10000000000;

  // transfer_id field in the request to tag the transaction and to correlate it to your back-end storage.
  const id = 287821;

  // gasPrice for native transfers can be set to 1.
  const gasPrice = 1;

  // Time that the deploy will remain valid for, in milliseconds
  // The default value is 1800000 ms (30 minutes).
  const ttl = 1800000;
  const publicKeyHex = await window.casperlabsHelper.getActivePublicKey();
  const publicKey = CLPublicKey.fromHex(publicKeyHex);

  let deployParams = new DeployUtil.DeployParams(
    publicKey,
    CHAIN_NAME,
    gasPrice,
    ttl
  );

  // We create a public key from account-address (it is the hex representation of the public-key with an added prefix).
  const toPublicKey = CLPublicKey.fromHex(to);

  const session = DeployUtil.ExecutableDeployItem.newTransfer(
    amount,
    toPublicKey,
    null,
    id
  );

  const payment = DeployUtil.standardPayment(paymentAmount);
  const deploy = DeployUtil.makeDeploy(deployParams, session, payment);

  // Turn your transaction data to format JSON
  const json = DeployUtil.deployToJson(deploy);

  // Sign transcation using casper-signer.
  const signature = await window.casperlabsHelper.sign(json, publicKeyHex, to);
  const deployObject = DeployUtil.deployFromJson(signature);

  if (deployObject.val) {
    // Here we are sending the signed deploy.
    const signed = await casperClient.putDeploy(deployObject.val as Deploy);
    tx = `tx: ${signed}`;
  }
  return tx;
};

const initClient = async () => {
  try {
    const contractPublicKey = CLPublicKey.fromHex(CONTRACT_HOLDER_ADDRESS);
    const cep47 = new CEP47Client(NODE_ADDRESS!, CHAIN_NAME!);
    let contractAccountInfo = await getAccountInfo(
        NODE_ADDRESS!,
        contractPublicKey
    );
    console.log(`... Account Info: `, contractAccountInfo);
    const contractHashKey = `${CONTRACT_NAME!}_contract_hash`;
    console.log({contractHashKey: contractHashKey});
    const contractHash = await getAccountNamedKeyValue(
        contractAccountInfo,
        contractHashKey
    );
    console.log(`... Contract Hash: ${contractHash}`);

    const contractPackageHash = await getAccountNamedKeyValue(
        contractAccountInfo,
        `contract_package_hash`
    );
    console.log(`... Contract Package Hash: ${contractPackageHash}`);

    cep47.setContractHash(contractHash, contractPackageHash);
    return {
      cep47,
      contractPublicKey,
    };
  }catch (e) {
    console.log(e);
  }

};

export const triggerMintDeploy = async (
  ids: string[],
  metas: Map<string, string>[]
) => {
  const { cep47 } = await initClient();

  const publicKeyHex = await window.casperlabsHelper.getActivePublicKey();
  const activePublicKey = CLPublicKey.fromHex(publicKeyHex);

  const mintDeploy = await cep47.mint(
    activePublicKey,
    ids,
    metas,
    MINT_ONE_PAYMENT_AMOUNT!,
    activePublicKey
  );

  // Turn your transaction data to format JSON
  const json = DeployUtil.deployToJson(mintDeploy);

  // Sign transcation using casper-signer.
  const signature = await window.casperlabsHelper.sign(
    json,
    publicKeyHex,
    publicKeyHex
  );
  const deployObject = DeployUtil.deployFromJson(signature);
  let mintDeployHash;
  if (deployObject.val) {
    // Here we are sending the signed deploy.
    mintDeployHash = await casperClient.putDeploy(deployObject.val as Deploy);
    console.log("...... Mint deployed");
    await getDeploy(NODE_ADDRESS!, mintDeployHash);
    console.log("...... Token minted successfully");
  }

  return { mintDeployHash };
};

export const getActiveAccountBalance = async (): Promise<any> => {
  const { contractPublicKey, cep47 } = await initClient();
  const publicKeyHex = await window.casperlabsHelper.getActivePublicKey();

  const activePublicKey = CLPublicKey.fromHex(publicKeyHex);

  try {
    const balanceOf1 = await cep47.balanceOf(contractPublicKey);

    console.log("...... Balance of master account: ", balanceOf1);

    const activeAccountBalance = await cep47.balanceOf(activePublicKey);

    console.log("...... Balance of active account: ", activeAccountBalance);

    const ownerOfTokenOne = await cep47.getOwnerOf("1");

    console.log("...... Owner of token one: ", ownerOfTokenOne);

    const tokenOneMeta = await cep47.getTokenMeta("1");
    console.log("...... Token one metadata: ", tokenOneMeta);

    const indexByToken1 = await cep47.getIndexByToken(activePublicKey, "1");
    console.log("...... index of token one: ", indexByToken1);

    const tokenByIndex1 = await cep47.getTokenByIndex(
      activePublicKey,
      indexByToken1
    );
    console.log("...... token one id: ", tokenByIndex1);
    return activeAccountBalance;
  } catch (e) {
    console.log(e);
    return 0;
  }
};

export const triggerBurnDeploy = async (ids: string[]) => {
  const { cep47 } = await initClient();
  const publicKeyHex = await window.casperlabsHelper.getActivePublicKey();
  const activePublicKey = CLPublicKey.fromHex(publicKeyHex);

  console.log("\n*************************\n");

  console.log("... Burn token one \n");

  const burnDeploy = await cep47.burn(
    activePublicKey,
    ids,
    MINT_ONE_PAYMENT_AMOUNT!,
    activePublicKey
  );
  // Turn your transaction data to format JSON
  const json = DeployUtil.deployToJson(burnDeploy);

  // Sign transcation using casper-signer.
  const signature = await window.casperlabsHelper.sign(
    json,
    publicKeyHex,
    publicKeyHex
  );
  const deployObject = DeployUtil.deployFromJson(signature);
  let burnDeployHash;
  if (deployObject.val) {
    // Here we are sending the signed deploy.
    burnDeployHash = await casperClient.putDeploy(deployObject.val as Deploy);
    console.log("... Burn deploy hash: ", burnDeployHash);

    await getDeploy(NODE_ADDRESS!, burnDeployHash);
    console.log("... Token burned successfully");
  }
};
