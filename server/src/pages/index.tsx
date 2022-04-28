import React, { useEffect, useState } from "react";

import { AppShell, SimpleGrid, Title } from "@mantine/core";
import { EventStream, Signer } from "casper-js-sdk";
import { Toaster } from "react-hot-toast";

import { CustomCard } from "../components/CustomCard";
import { CustomHeader } from "../components/CustomHeader";
import { CustomNavbar } from "../components/CustomNavbar";
import {
  getOwnedNFTS,
  NFT,
  subscribeToContractEvents,
} from "../utils/cep47_utils";
import { CustomBackground } from "../components/CustomBackground";

export default function DashboardCyber() {
  const [address, setAddress] = useState(null);
  const [connected, setConnected] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const [locked, setLocked] = useState(false);
  const [items, setItems] = useState<NFT[]>([]);

  const retrieveNFTS = async () => {
    const result = await getOwnedNFTS();
    if (result) {
      setItems(result);
    }
  };
  const updateState = async () => {
    try {
      setConnected(await Signer.isConnected());
      retrieveNFTS();
    } catch (err) {
      console.error(err);
    }
  };

  useEffect(() => {
    // Without the timeout it doesn't always work properly
    setTimeout(async () => {
      updateState();
    }, 100);
  }, []);

  useEffect(() => {
    const es = new EventStream(
      process.env.NEXT_PUBLIC_CASPER_EVENT_STREAM_ADDRESS!
    );
    subscribeToContractEvents(es, () => {
      updateState();
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

      updateState();
    });
    window.addEventListener("signer:disconnected", (msg) => {
      setConnected(false);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
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

      updateState();
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
      updateState();
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
        {items.map((item, index) => (
          <CustomCard
            key={index}
            id={item.id}
            image={item.meta.get("image_url") || ""}
            title={item.meta.get("name") || ""}
            description={item.meta.get("description") || ""}
            linkTo={`nft/${item.id}`}
          />
        ))}
      </SimpleGrid>

      <CustomBackground />
    </AppShell>
  );
}
