import { config } from "dotenv";
import {
  getAccountInfo,
  getAccountNamedKeyValue,
  getDeploy,
  parseTokenMeta,
} from "../utils";
import * as fs from "fs";

import { Keys } from "casper-js-sdk";
import { MARKETClient } from "../../packages/market-client/src";

config({ path: ".env" });

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  MARKET_WASM_PATH,
  MASTER_KEY_PAIR_PATH,
  MARKET_NAME,
  MARKET_CONTRACT_NAME,
  TOKEN_SYMBOL,
  CONTRACT_HASH,
  INSTALL_PAYMENT_AMOUNT,
  MINT_ONE_PAYMENT_AMOUNT,
  MINT_COPIES_PAYMENT_AMOUNT,
  BURN_ONE_PAYMENT_AMOUNT,
  MINT_ONE_META_SIZE,
  MINT_COPIES_META_SIZE,
  MINT_COPIES_COUNT,
  MINT_MANY_META_SIZE,
  MINT_MANY_META_COUNT,
} = process.env;

export const getBinary = (pathToBinary: string) => {
  return new Uint8Array(fs.readFileSync(pathToBinary, null).buffer);
};

const TOKEN_META = new Map(parseTokenMeta(process.env.TOKEN_META!));

const KEYS = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

const test = async () => {
  //TODO change to generic client
  const marketClient = new MARKETClient(NODE_ADDRESS!, CHAIN_NAME!);
  const installDeployHash = await marketClient.install(
    getBinary(MARKET_WASM_PATH!),
    {
      marketName: MARKET_NAME!,
      contractName: MARKET_CONTRACT_NAME!,
      marketSymbol: TOKEN_SYMBOL!,
      marketMeta: TOKEN_META,
    },
    INSTALL_PAYMENT_AMOUNT!,
    KEYS.publicKey,
    [KEYS]
  );

  const hash = await installDeployHash.send(NODE_ADDRESS!);

  console.log(`... Market Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS!, hash);

  console.log(`... Market Contract installed successfully.`);

  let accountInfo = await getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = await getAccountNamedKeyValue(
    accountInfo,
    `${MARKET_CONTRACT_NAME!}_contract_hash`
  );

  console.log(`... Market Contract Hash: ${contractHash}`);

  const contractPackageHash = await getAccountNamedKeyValue(
    accountInfo,
    `contract_package_hash`
  );
  console.log(`... Market Contract Package Hash: ${contractPackageHash}`);

  const marketPackageHash = await getAccountNamedKeyValue(
    accountInfo,
    `market_item_hash`
  );
  console.log(`... Market Item Hash: ${marketPackageHash}`);
};

test();
