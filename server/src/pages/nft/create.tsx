import React, { useEffect, useState } from "react";

import {
  AppShell,
  Box,
  Button,
  Group,
  Textarea,
  TextInput,
  Title,
} from "@mantine/core";
import { useForm } from "@mantine/hooks";
import { EventStream, Signer } from "casper-js-sdk";
import { toast, Toaster } from "react-hot-toast";

import { CustomHeader } from "../../components/CustomHeader";
import { CustomNavbar } from "../../components/CustomNavbar";
import styles from "../../styles/dashboard-cyber.module.scss";
import {
  EVENT_STREAM_ADDRESS,
  getActiveAccountBalance,
  initClient,
  subscribeToContractEvents,
  triggerMintDeploy,
} from "../../utils/cep47_utils";
import { NFTMeta } from "../../utils/types";

export default function Mint() {
  const [address, setAddress] = useState(null);
  const [connected, setConnected] = useState(false);
  const [locked, setLocked] = useState(false);

  // Without the timeout it doesn't always work properly
  setTimeout(async () => {
    try {
      setConnected(await Signer.isConnected());
    } catch (err) {
      console.log(err);
    }
  }, 100);

  // const [publicKey, setPublicKey] = useState("");
  // const [balance, setBalance] = useState("");
  // const [nftBalance, setNFTBalance] = useState(0);
  // const [tx, setTx] = useState("");
  // const [to, setTo] = useState("");
  // const [amount, setAmount] = useState("");

  useEffect(() => {
    console.log("subscription called");
    const es = new EventStream(EVENT_STREAM_ADDRESS!);
    subscribeToContractEvents(es, () => getActiveAccountBalance());
  }, []);
  useEffect(() => {
    window.addEventListener("signer:connected", (msg) => {
      setConnected(true);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
      toast.success("Connected to Signer!");
    });
    window.addEventListener("signer:disconnected", (msg) => {
      setConnected(false);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
      toast("Disconnected from Signer");
    });
    window.addEventListener("signer:tabUpdated", (msg) => {
      // @ts-ignore
      setConnected(msg.detail.isConnected);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
    });
    window.addEventListener("signer:activeKeyChanged", (msg) => {
      // @ts-ignore
      setAddress(msg.detail.activeKey);
      toast("Active key changed");
    });
    window.addEventListener("signer:locked", (msg) => {
      // @ts-ignore
      setConnected(msg.detail.isConnected);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
    });
    window.addEventListener("signer:unlocked", (msg) => {
      // @ts-ignore
      setConnected(msg.detail.isConnected);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
    });
    window.addEventListener("signer:initialState", (msg) => {
      // @ts-ignore
      setConnected(msg.detail.isConnected);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
    });
  }, []);

  const form = useForm({
    initialValues: {
      name: "test",
      symbol: "TEST",
      url: "test.com",
      customMeta: `{"hello":"world"}`,
      description: "test description",
    },
  });
  const mintNFT = async (item: NFTMeta) => {
    const { cep47 } = await initClient();
    if (!cep47) return;
    const totalSupply = await cep47.totalSupply();
    const startIndex = totalSupply;

    const mapped: Map<string, string> = new Map(Object.entries(item));
    console.log("...... Triggered Mint Deploy: ");
    toast.promise(triggerMintDeploy([`${startIndex}`], [mapped]), {
      loading: "Minting in progress",
      success: "Minting Successful",
      error: "Error when minting",
    });
  };

  const createNFT = async (values: {
    name: string;
    symbol: string;
    url: string;
    customMeta: string;
    description: string;
  }) => {
    let customMeta;
    let item;
    try {
      customMeta = JSON.parse(values.customMeta);
    } catch (e) {
      console.log(e);
    }

    if (
      typeof customMeta === "object" &&
      !Array.isArray(customMeta) &&
      customMeta !== null
    ) {
      item = {
        ...values,
        ...customMeta,
      };
    } else if (customMeta === null) {
      item = {
        ...values,
      };
    } else {
      console.log(
        "Invalid format passed to Custom Meta. Expecting JSON Object."
      );
      toast.error("Invalid Custom Meta format. Expecting JSON Object.");
      return;
    }
    const deployHash = await mintNFT(item);
    console.log({ deployHash });
  };
  return (
    <AppShell
      padding="md"
      navbar={<CustomNavbar connected={connected} />}
      header={<CustomHeader address={address} locked={locked} />}
    >
      <Toaster />

      <Title order={1}>Create your NFT</Title>
      <Box sx={{ maxWidth: 300 }} mx="auto">
        <form onSubmit={form.onSubmit((values) => createNFT(values))}>
          <TextInput required label="Name" {...form.getInputProps("name")} />
          <TextInput
            required
            label="Symbol"
            {...form.getInputProps("symbol")}
          />
          <TextInput
            required
            label="Symbol"
            {...form.getInputProps("symbol")}
          />
          <TextInput
            required
            label="Image URL"
            {...form.getInputProps("url")}
          />
          {/* eslint-disable-next-line react/jsx-no-undef */}
          <Textarea
            // placeholder="Enter"
            label="Custom Meta (JSON Format)"
            autosize
            {...form.getInputProps("customMeta")}
            minRows={2}
          />
          <TextInput
            required
            label="Description"
            {...form.getInputProps("description")}
          />

          <Group position="right" mt="md">
            <Button type="submit">Create</Button>
          </Group>
        </form>
      </Box>
      <div className={styles.bg}>
        <div className={styles.starField}>
          <div className={styles.layer}></div>
          <div className={styles.layer}></div>
          <div className={styles.layer}></div>
          <div className={styles.layer}></div>
          <div className={styles.layer}></div>
        </div>
      </div>
    </AppShell>
  );
}
