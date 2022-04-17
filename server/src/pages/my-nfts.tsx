import React, { useEffect, useState } from "react";

import { AppShell, SimpleGrid, Title } from "@mantine/core";
import { EventStream } from "casper-js-sdk";
import { toast } from "react-hot-toast";

import { CustomHeader } from "../components/CustomHeader";
import { CustomNavbar } from "../components/CustomNavbar";
import { MyCard } from "../components/MyCard";
import styles from "../styles/dashboard-cyber.module.scss";
import {
  EVENT_STREAM_ADDRESS,
  getActiveAccountBalance,
  subscribeToContractEvents,
} from "../utils/cep47_utils";
import { supabaseServerSideClient } from "../utils/supabaseServerSideClient";

export async function getServerSideProps(_context: any) {
  const { data: items } = await supabaseServerSideClient
    .from("item")
    .select("*");
  return {
    props: { items }, // will be passed to the page component as props
  };
}

export interface NFTItem {
  hash: string;
  image_url: string;
  name: string;
  copies: number;
  symbol: string;
  contract_name: string;
}

export default function DashboardCyber(props: { items: NFTItem[] }) {
  const { items } = props;
  const [address, setAddress] = useState("");
  const [connected, setConnected] = useState(false);
  const [locked, setLocked] = useState(false);
  // const [publicKey, setPublicKey] = useState("");
  // const [balance, setBalance] = useState("");
  // const [nftBalance, setNFTBalance] = useState(0);
  // const [tx, setTx] = useState("");
  // const [to, setTo] = useState("");
  // const [amount, setAmount] = useState("");
  // const updateAccountInformation = async () => {

  //   // const {
  //   //   textAddress,
  //   //   // textBalance,
  //   //   // publicKey: updatedPublicKey,
  //   // } = await accountInformation();
  //   // setAddress(textAddress);
  //   // // setBalance(textBalance);
  //   // // setPublicKey(updatedPublicKey);
  //   // // setNFTBalance(await getActiveAccountBalance());
  //   // if (textAddress) {
  //   //   setConnected(true);
  //   // }
  // };

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

  console.log(items);
  return (
    <AppShell
      padding="md"
      navbar={<CustomNavbar connected={connected} />}
      header={<CustomHeader address={address} locked={locked} />}
    >
      <Title order={1}>My NFTs</Title>
      <SimpleGrid cols={3} spacing={50} style={{ margin: "5em" }}>
        {items.map((nft, index) => (
          <MyCard
            key={index}
            image={nft.image_url}
            title={nft.name}
            description={""}
            buttonText={"Sell (Coming Soon)"}
          />
        ))}
      </SimpleGrid>
      <div className={styles.bg}>
        <div className={styles.starField}>
          <div className={styles.layer} />
        </div>
      </div>
    </AppShell>
  );
}
