import { config } from "dotenv";
import { CEP47Client } from "casper-cep47-js-client";
import {
  getAccountInfo,
  getAccountNamedKeyValue,
  getDeploy,
  sleep,
} from "../utils";

import { Keys } from "casper-js-sdk";
import { mockData } from "../../mockData";

config({ path: ".env.cep47" });

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  WASM_PATH,
  MASTER_KEY_PAIR_PATH,
  USER_KEY_PAIR_PATH,
  TOKEN_NAME,
  CONTRACT_NAME,
  TOKEN_SYMBOL,
  CONTRACT_HASH,
  INSTALL_PAYMENT_AMOUNT,
  MINT_ONE_PAYMENT_AMOUNT,
  MINT_COPIES_PAYMENT_AMOUNT,
  TRANSFER_ONE_PAYMENT_AMOUNT,
  BURN_ONE_PAYMENT_AMOUNT,
  MINT_ONE_META_SIZE,
  MINT_COPIES_META_SIZE,
  MINT_COPIES_COUNT,
  MINT_MANY_META_SIZE,
  MINT_MANY_META_COUNT,
} = process.env;

const KEYS = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

const KEYS_USER = Keys.Ed25519.parseKeyFiles(
  `${USER_KEY_PAIR_PATH}/public_key.pem`,
  `${USER_KEY_PAIR_PATH}/secret_key.pem`
);

const loadFixture = async () => {
  const cep47 = new CEP47Client(NODE_ADDRESS!, CHAIN_NAME!);

  let accountInfo = await getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = await getAccountNamedKeyValue(
    accountInfo,
    `${CONTRACT_NAME!}_contract_hash`
  );

  const contractPackageHash = await getAccountNamedKeyValue(
    accountInfo,
    `contract_package_hash`
  );

  console.log(`... Contract Hash: ${contractHash}`);
  console.log(`... Contract Package Hash: ${contractPackageHash}`);

  await cep47.setContractHash(contractHash, contractPackageHash);

  await sleep(5 * 1000);
  //
  // const es = new EventStream(EVENT_STREAM_ADDRESS!);
  //
  // es.subscribe(EventName.DeployProcessed, (event) => {
  //   const parsedEvents = CEP47EventParser(
  //     {
  //       contractPackageHash,
  //       eventNames: [
  //         CEP47Events.MintOne,
  //         CEP47Events.TransferToken,
  //         CEP47Events.BurnOne,
  //         CEP47Events.MetadataUpdate,
  //         CEP47Events.ApproveToken,
  //       ],
  //     },
  //     event
  //   );
  //
  //   if (parsedEvents && parsedEvents.success) {
  //     console.log("*** EVENT ***");
  //     console.log(parsedEvents.data);
  //     console.log("*** ***");
  //   }
  // });
  //
  // es.start();

  const name = await cep47.name();
  console.log(`... Contract name: ${name}`);

  const symbol = await cep47.symbol();
  console.log(`... Contract symbol: ${symbol}`);

  const meta = await cep47.meta();
  console.log(`... Contract meta: ${JSON.stringify(meta)}`);

  let totalSupply = await cep47.totalSupply();
  console.log(`... Total supply: ${totalSupply}`);

  //****************//
  //* Mint Section *//
  //****************//
  console.log("\n*************************\n");

  console.log("... Mint mock tokens \n");
  const ids: string[] = [];
  const metas = mockData.planets.map((item, index) => {
    ids.push(`${index}`);
    return new Map<string, string>(Object.entries(item));
  });

  for (let i = 0; i < ids.length; i++) {
    try {
      const mintDeploy = await cep47.mint(
        KEYS.publicKey,
        [ids[i]],
        [metas[i]],
        MINT_ONE_PAYMENT_AMOUNT!,
        KEYS.publicKey,
        [KEYS]
      );
      console.log("...... Mint deploy name: ", metas[i].get("name"));

      const mintDeployHash = await mintDeploy.send(NODE_ADDRESS!);

      console.log("...... Mint deploy hash: ", mintDeployHash);

      await getDeploy(NODE_ADDRESS!, mintDeployHash);
      console.log("...... Token minted successfully");
    } catch (e) {
      console.log(e);
    }
  }
};

loadFixture();
