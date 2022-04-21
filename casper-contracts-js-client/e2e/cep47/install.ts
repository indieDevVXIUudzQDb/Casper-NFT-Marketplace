import { config } from "dotenv";
import { CEP47Client } from "casper-cep47-js-client";
import {
  getAccountInfo,
  getAccountNamedKeyValue,
  getDeploy,
  parseTokenMeta,
} from "../utils";
import * as fs from "fs";

import { Keys } from "casper-js-sdk";

config({ path: ".env.cep47" });

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  WASM_PATH,
  MASTER_KEY_PAIR_PATH,
  TOKEN_NAME,
  CONTRACT_NAME,
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
  const cep47 = new CEP47Client(NODE_ADDRESS!, CHAIN_NAME!);

  const installDeployHash = await cep47.install(
    getBinary(WASM_PATH!),
    {
      name: TOKEN_NAME!,
      contractName: CONTRACT_NAME!,
      symbol: TOKEN_SYMBOL!,
      meta: TOKEN_META,
    },
    INSTALL_PAYMENT_AMOUNT!,
    KEYS.publicKey,
    [KEYS]
  );

  const hash = await installDeployHash.send(NODE_ADDRESS!);

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS!, hash);

  console.log(`... Contract installed successfully.`);

  let accountInfo = await getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = getAccountNamedKeyValue(
    accountInfo,
    `${CONTRACT_NAME!}_contract_hash`
  );

  console.log(`... Contract Hash: ${contractHash}`);
};

const getContractHash = async () => {
  let accountInfo = await getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  const contractHash = getAccountNamedKeyValue(
    accountInfo,
    `${CONTRACT_NAME!}_contract_hash`
  );

  console.log(`... Contract Hash: ${contractHash}`);

  const contractPackageHash = await getAccountNamedKeyValue(
    accountInfo,
    `contract_package_hash`
  );
  console.log(`... Contract Package Hash: ${contractPackageHash}`);
  return contractHash;
};

const getTotalSupply = async () => {
  const cep47 = new CEP47Client(NODE_ADDRESS!, CHAIN_NAME!);
  const contractHash = await getContractHash();

  cep47.setContractHash(contractHash);
  let totalSupply = await cep47.totalSupply();
  console.log(`... Total supply: ${totalSupply}`);
  // return totalSupply;
};

// test();

getTotalSupply();
