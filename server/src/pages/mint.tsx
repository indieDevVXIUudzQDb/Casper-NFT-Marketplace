import React, { useEffect, useState } from "react";

import { AppShell, Group, Title, Button, Box, TextInput } from "@mantine/core";

import styles from "../styles/dashboard-cyber.module.scss";
import { useForm } from "@mantine/hooks";
import { CustomNavbar } from "../components/CustomNavbar";
import { CustomHeader } from "../components/CustomHeader";
import {
  accountInformation,
  EVENT_STREAM_ADDRESS,
  getActiveAccountBalance,
  subscribeToContractEvents,
} from "../utils/cep47_utils";
import { EventStream } from "casper-js-sdk";

export default function Mint() {
  const [address, setAddress] = useState("");
  // const [publicKey, setPublicKey] = useState("");
  // const [balance, setBalance] = useState("");
  // const [nftBalance, setNFTBalance] = useState(0);
  // const [tx, setTx] = useState("");
  // const [to, setTo] = useState("");
  // const [amount, setAmount] = useState("");
  const [connected, setConnected] = useState(false);
  const updateAccountInformation = async () => {
    const {
      textAddress,
      // textBalance,
      // publicKey: updatedPublicKey,
    } = await accountInformation();
    setAddress(textAddress);
    // setBalance(textBalance);
    // setPublicKey(updatedPublicKey);
    // setNFTBalance(await getActiveAccountBalance());
    if (textAddress) {
      setConnected(true);
    }
  };

  useEffect(() => {
    console.log("subscription called");
    const es = new EventStream(EVENT_STREAM_ADDRESS!);
    subscribeToContractEvents(es, () => getActiveAccountBalance());
    updateAccountInformation();
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
      navbar={
        <CustomNavbar
          connected={connected}
          updateAccountInformation={updateAccountInformation}
        />
      }
      header={<CustomHeader address={address} />}
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
          <div className={styles.layer} />
          <div className={styles.layer} />
          <div className={styles.layer} />
          <div className={styles.layer} />
        </div>
      </div>
    </AppShell>
  );
}
