import React, { useEffect, useState } from "react";

import { AppShell, Box, Button, Group, TextInput, Title } from "@mantine/core";

import styles from "../styles/dashboard-cyber.module.scss";
import { useForm } from "@mantine/hooks";
import { CustomNavbar } from "../components/CustomNavbar";
import { CustomHeader } from "../components/CustomHeader";
import {
  EVENT_STREAM_ADDRESS,
  getActiveAccountBalance,
  subscribeToContractEvents,
} from "../utils/cep47_utils";
import { EventStream } from "casper-js-sdk";
import { toast } from "react-hot-toast";

export default function Mint() {
  const [address, setAddress] = useState("");
  const [connected, setConnected] = useState(false);
  const [locked, setLocked] = useState(false);

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
      name: "",
      symbol: "",
      url: "",
      description: "",
    },
  });
  return (
    <AppShell
      padding="md"
      navbar={<CustomNavbar connected={connected} />}
      header={<CustomHeader address={address} locked={locked} />}
    >
      <Title order={1}>Mint your NFT</Title>
      <Box sx={{ maxWidth: 300 }} mx="auto">
        <form onSubmit={form.onSubmit((values) => console.log(values))}>
          <TextInput required label="Name" {...form.getInputProps("name")} />
          <TextInput
            required
            label="Symbol"
            {...form.getInputProps("symbol")}
          />
          <TextInput required label="URL" {...form.getInputProps("url")} />
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
          <div className={styles.layer} />
        </div>
      </div>
    </AppShell>
  );
}