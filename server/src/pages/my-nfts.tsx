import React, { useEffect, useState } from "react";

import { AppShell, SimpleGrid, Title } from "@mantine/core";
import { Signer } from "casper-js-sdk";
import { toast } from "react-hot-toast";

import { CustomCard } from "../components/CustomCard";
import { CustomHeader } from "../components/CustomHeader";
import { CustomNavbar } from "../components/CustomNavbar";
import styles from "../styles/dashboard-cyber.module.scss";
import { toastConfig } from "../toastConfig";
import { getOwnedNFTS } from "../utils/cep47_utils";
import { supabaseServerSideClient } from "../utils/supabaseServerSideClient";

export async function getServerSideProps(_context: any) {
  const { data: items } = await supabaseServerSideClient
    .from("item")
    .select("*");
  return {
    props: { items }, // will be passed to the page component as props
  };
}

export default function DashboardCyber() {
  // const { items } = props;

  const [items, setItems] = useState<Map<string, string>[]>([]);
  const [address, setAddress] = useState(null);
  const [menuOpen, setMenuOpen] = useState(false);
  const [connected, setConnected] = useState(false);
  const [locked, setLocked] = useState(false);

  // useEffect(() => {
  //   console.log("subscription called");
  //   const es = new EventStream(EVENT_STREAM_ADDRESS!);
  //   subscribeToContractEvents(es, () => getActiveAccountBalance());
  // }, []);

  const retrieveNFTS = async () => {
    const result = await toast.promise(
      getOwnedNFTS(),
      {
        loading: "Loading",
        success: "Loaded NFTs",
        error: "Error retrieving owned NFTs",
      },
      toastConfig
    );
    if (result) {
      setItems(result);
    }
  };
  useEffect(() => {
    // Without the timeout it doesn't always work properly
    setTimeout(async () => {
      try {
        setConnected(await Signer.isConnected());
        retrieveNFTS();
      } catch (err) {
        console.log(err);
      }
    }, 100);
  }, []);

  useEffect(() => {
    window.addEventListener("signer:connected", (msg) => {
      setConnected(true);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
      toast.success("Connected to Signer!", toastConfig);
      retrieveNFTS();
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
      retrieveNFTS();
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

  console.log(items);
  return (
    <AppShell
      padding="md"
      navbarOffsetBreakpoint="sm"
      asideOffsetBreakpoint="sm"
      fixed
      navbar={
        <CustomNavbar
          connected={connected}
          locked={locked}
          menuOpen={menuOpen}
        />
      }
      header={
        <CustomHeader
          address={address}
          locked={locked}
          menuOpen={menuOpen}
          setMenuOpen={setMenuOpen}
        />
      }
    >
      <div
        style={{
          textAlign: "center",
          margin: "1em",
          marginLeft: "3em",
        }}
      >
        <Title order={1}>My NFTs</Title>
      </div>
      <SimpleGrid
        cols={3}
        spacing="lg"
        breakpoints={[
          { maxWidth: 980, cols: 2, spacing: "md" },
          { maxWidth: 755, cols: 1, spacing: "sm" },
          { maxWidth: 600, cols: 1, spacing: "sm" },
        ]}
        style={{ marginLeft: "3em" }}
      >
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
