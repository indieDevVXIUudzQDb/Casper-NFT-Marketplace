import React, { useEffect, useState } from "react";

import { AppShell, SimpleGrid, Title } from "@mantine/core";
import { EventStream, Signer } from "casper-js-sdk";
import { useRouter } from "next/router";
import { toast, Toaster } from "react-hot-toast";

import { CustomCard } from "../../components/CustomCard";
import { CustomHeader } from "../../components/CustomHeader";
import { CustomNavbar } from "../../components/CustomNavbar";
import styles from "../../styles/dashboard-cyber.module.scss";
import { toastConfig } from "../../toastConfig";
import {
  getNFT,
  RetrievedNFT,
  subscribeToContractEvents,
} from "../../utils/cep47_utils";

export default function DashboardCyber() {
  const [address, setAddress] = useState(null);
  const [connected, setConnected] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const [locked, setLocked] = useState(false);
  const [item, setItem] = useState<RetrievedNFT | null>();

  const router = useRouter();
  const { id } = router.query;

  const retrieveNFT = async () => {
    if (id) {
      const result = await toast.promise(
        getNFT(Number(id)),
        {
          loading: "Loading",
          success: "Loaded NFT",
          error: "Error retrieving NFT",
        },
        toastConfig
      );
      if (result) {
        setItem(result);
      }
    }
  };

  useEffect(() => {
    // Without the timeout it doesn't always work properly
    setTimeout(async () => {
      try {
        setConnected(await Signer.isConnected());
        retrieveNFT();
      } catch (err) {
        console.error(err);
      }
    }, 100);
  }, []);

  useEffect(() => {
    const es = new EventStream(
      process.env.NEXT_PUBLIC_CASPER_EVENT_STREAM_ADDRESS!
    );
    subscribeToContractEvents(es, () => {
      retrieveNFT();
      console.log(es);
    });
  }, []);

  useEffect(() => {
    window.addEventListener("signer:connected", (msg) => {
      setConnected(true);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
      toast.success("Connected to Signer!", toastConfig);
      retrieveNFT();
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
      retrieveNFT();
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
      retrieveNFT();
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
      <div>
        <Toaster />
      </div>

      <div
        style={{
          textAlign: "center",
          margin: "1em",
          marginLeft: "3em",
        }}
      >
        <Title order={1}>Distant Planet Collection</Title>
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
        <CustomCard
          key={item?.meta.get("id")}
          linkTo={""}
          id={item?.meta.get("id") || ""}
          image={item?.meta.get("image_url") || ""}
          title={item?.meta.get("name") || ""}
          description={item?.meta.get("description") || ""}
        />
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
