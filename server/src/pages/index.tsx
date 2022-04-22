import React, {useEffect, useState} from "react";

import {AppShell, SimpleGrid, Title} from "@mantine/core";
import {Signer} from "casper-js-sdk";
import {toast, Toaster} from "react-hot-toast";

import {CustomCard} from "../components/CustomCard";
import {CustomHeader} from "../components/CustomHeader";
import {CustomNavbar} from "../components/CustomNavbar";
import styles from "../styles/dashboard-cyber.module.scss";
import {initClient} from "../utils/cep47_utils";
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
  const [items, setItems] = useState<Map<string, string>[]>();

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

  const retrieveNFTS = async () => {
    const { cep47 } = await initClient();
    if (!cep47) return;
    const totalSupply = await cep47.totalSupply();

    const nfts = [];

    // eslint-disable-next-line no-plusplus
    for (let i = 0; i < totalSupply; i++) {
      // eslint-disable-next-line no-await-in-loop
      const tokenMeta = await cep47.getTokenMeta(`${i}`);
      nfts.push(tokenMeta);
    }
    setItems(nfts);
    console.log(nfts);
  };
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
        {/* {mockData.planets.map((planet, index) => ( */}
        {items?.map((item, index) => (
          <CustomCard
            // index={index}
            key={index}
            image={item.get("image_url") || ""}
            title={item.get("name") || ""}
            description={item.get("description") || ""}
            buttonText={"Coming Soon"}
          />
        ))}

        {/* ))} */}
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
