import {
  CasperClient,
  CasperServiceByJsonRPC,
  CLPublicKey,
  Keys,
} from "casper-js-sdk";

export const contractHashToByteArray = (contractHash: string): Uint8Array => {
  return Uint8Array.from(Buffer.from(contractHash, "hex"));
};

export const parseTokenMeta = (str: string): Array<[string, string]> =>
  // @ts-ignore
  str.split(",").map((s) => {
    const map = s.split(" ");
    return [map[0], map[1]];
  });

export const sleep = (ms: number) => {
  // eslint-disable-next-line no-promise-executor-return
  return new Promise((resolve) => setTimeout(resolve, ms));
};

/**
 * Returns a set ECC key pairs - one for each NCTL user account.
 * @param {String} pathToUsers - Path to NCTL user directories.
 * @return {Array} An array of assymmetric keys.
 */
export const getKeyPairOfUserSet = (pathToUsers: string) => {
  return [1, 2, 3, 4, 5].map((userID) => {
    return Keys.Ed25519.parseKeyFiles(
      `${pathToUsers}/user-${userID}/public_key.pem`,
      `${pathToUsers}/user-${userID}/secret_key.pem`
    );
  });
};

// @ts-ignore
// eslint-disable-next-line consistent-return
export const getDeploy = async (NODE_URL: string, deployHash: string) => {
  const client = new CasperClient(NODE_URL);
  let i = 300;
  // eslint-disable-next-line eqeqeq
  while (i != 0) {
    // eslint-disable-next-line no-await-in-loop
    const [deploy, raw] = await client.getDeploy(deployHash);
    if (raw.execution_results.length !== 0) {
      // @ts-ignore
      if (raw.execution_results[0].result.Success) {
        return deploy;
      }
      // @ts-ignore
      throw Error(
        `Contract execution: ${
          // @ts-ignore
          raw.execution_results[0].result.Failure.error_message
        }`
      );
    } else {
      // eslint-disable-next-line no-plusplus
      i--;
      // eslint-disable-next-line no-await-in-loop
      await sleep(1000);
    }
  }
  // throw Error("Timeout after " + i + "s. Something's wrong");
};

export const getAccountInfo: any = async (
  nodeAddress: string,
  publicKey: CLPublicKey
) => {
  const client = new CasperServiceByJsonRPC(nodeAddress);
  const stateRootHash = await client.getStateRootHash();
  const accountHash = publicKey.toAccountHashStr();
  const blockState = await client.getBlockState(stateRootHash, accountHash, []);
  return blockState.Account;
};

/**
 * Returns a value under an on-chain account's storage.
 * @param accountInfo - On-chain account's info.
 * @param namedKey - A named key associated with an on-chain account.
 */
export const getAccountNamedKeyValue = (accountInfo: any, namedKey: string) => {
  const found = accountInfo.namedKeys.find((i: any) => i.name === namedKey);
  if (found) {
    return found.key;
  }
  return undefined;
};

export const textShortener = (text: string, maxLength: number) => {
  if (text.length > maxLength) {
    const start = text.substring(0, maxLength);
    return `${start}…`;
  }
  return text;
};
