import React, { useEffect, useState } from "react";

import { AppShell, SimpleGrid, Title } from "@mantine/core";

import { MyCard } from "../components/MyCard";
import { mockData } from "../mockData";
import styles from "../styles/dashboard-cyber.module.scss";
import {
  accountInformation,
  EVENT_STREAM_ADDRESS,
  getActiveAccountBalance,
  subscribeToContractEvents,
} from "../utils/cep47_utils";
import { EventStream } from "casper-js-sdk";
import { CustomNavbar } from "../components/CustomNavbar";
import { CustomHeader } from "../components/CustomHeader";

export default function DashboardCyber() {
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
      <Title order={1}>Distant Planet Collection</Title>

      <SimpleGrid cols={3} spacing={50} style={{ margin: "5em" }}>
        {mockData.planets.map((planet, index) => (
          <MyCard
            key={index}
            image={planet.url}
            title={planet.name}
            description={planet.description}
            buttonText={planet.actionText}
          />
        ))}
      </SimpleGrid>

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
