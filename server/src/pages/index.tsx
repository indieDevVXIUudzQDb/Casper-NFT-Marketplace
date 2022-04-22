import React, {useEffect, useState} from "react";

import {AppShell, SimpleGrid, Title} from "@mantine/core";
import {Signer} from "casper-js-sdk";
import {toast, Toaster} from "react-hot-toast";

import {CustomCard} from "../components/CustomCard";
import {CustomHeader} from "../components/CustomHeader";
import {CustomNavbar} from "../components/CustomNavbar";
import styles from "../styles/dashboard-cyber.module.scss";
import {toastConfig} from "../toastConfig";
import {getNFTS} from "../utils/cep47_utils";
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
  const [items, setItems] = useState<Map<string, string>[]>([]);

  // Without the timeout it doesn't always work properly
  setTimeout(async () => {
    try {
      setConnected(await Signer.isConnected());
    } catch (err) {
      console.log(err);
    }
  }, 100);

  // useEffect(() => {
  //   console.log("subscription called");
  //   const es = new EventStream(EVENT_STREAM_ADDRESS!);
  //   subscribeToContractEvents(es, () => getActiveAccountBalance());
  // }, []);

  const retrieveNFTS = async () => {
    const result = await toast.promise(
      getNFTS(),
      {
        loading: "Loading",
        success: "Loaded NFTs",
        error: "Error retrieving NFTs",
      },
      toastConfig
    );
    if (result) {
      setItems(result);
    }
  };

  useEffect(() => {
    window.addEventListener("signer:connected", (msg) => {
      setConnected(true);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
      toast.success("Connected to Signer!", toastConfig);
    });
    window.addEventListener("signer:disconnected", (msg) => {
      setConnected(false);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
      toast("Disconnected from Signer", toastConfig);
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
      toast("Active key changed", toastConfig);
      retrieveNFTS();
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

  useEffect(() => {
    retrieveNFTS();
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
        {items.map((item, index) => (
          <CustomCard
            key={index}
            image={item.get("image_url") || ""}
            title={item.get("name") || ""}
            description={item.get("description") || ""}
            buttonText={"Coming Soon"}
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
