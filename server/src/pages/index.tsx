import React, {useEffect, useState} from "react";

import {AppShell, SimpleGrid, Title} from "@mantine/core";
import {Signer} from "casper-js-sdk";
import {toast, Toaster} from "react-hot-toast";

import {CustomHeader} from "../components/CustomHeader";
import {CustomNavbar} from "../components/CustomNavbar";
import {CustomCard} from "../components/MyCard";
import {mockData} from "../mockData";
import styles from "../styles/dashboard-cyber.module.scss";
// import {
//   EVENT_STREAM_ADDRESS,
//   getActiveAccountBalance,
//   subscribeToContractEvents,
// } from "../utils/cep47_utils";
// import { EventStream } from "casper-js-sdk";

export default function DashboardCyber() {
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
  // const updateAccountInformation = async () => {
  //   // const {
  //   //   textAddress: updatedTextAddress,
  //   //   // textBalance,
  //   //   // publicKey: updatedPublicKey,
  //   // } = await accountInformation();
  //   // setAddress(updatedTextAddress);
  //   // setBalance(textBalance);
  //   // setPublicKey(updatedPublicKey);
  //   // setNFTBalance(await getActiveAccountBalance());
  // };

  // useEffect(() => {
  //   console.log("subscription called");
  //   const es = new EventStream(EVENT_STREAM_ADDRESS!);
  //   subscribeToContractEvents(es, () => getActiveAccountBalance());
  // }, []);

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
  return (
    <AppShell
      padding="md"
      navbar={<CustomNavbar connected={connected} locked={locked} />}
      header={<CustomHeader address={address} locked={locked} />}
    >
      <div>
        <Toaster />
      </div>

      <Title order={1}>Distant Planet Collection</Title>

      <SimpleGrid cols={3} spacing={50} style={{ margin: "5em" }}>
        {mockData.planets.map((planet, index) => (
          <CustomCard
            // index={index}
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
