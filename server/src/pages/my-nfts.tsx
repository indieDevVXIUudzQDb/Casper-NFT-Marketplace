import React, { useEffect, useState } from "react";

import { AppShell, SimpleGrid, Title } from "@mantine/core";

import styles from "../styles/dashboard-cyber.module.scss";
import { supabaseServerSideClient } from "../utils/supabaseServerSideClient";
import { MyCard } from "../components/MyCard";
import {
  accountInformation,
  EVENT_STREAM_ADDRESS,
  getActiveAccountBalance,
  subscribeToContractEvents,
} from "../utils/cep47_utils";
import { EventStream } from "casper-js-sdk";
import { CustomNavbar } from "../components/CustomNavbar";
import { CustomHeader } from "../components/CustomHeader";

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
  const items = props.items;
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

  console.log(items);
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
          <div className={styles.layer} />
          <div className={styles.layer} />
          <div className={styles.layer} />
          <div className={styles.layer} />
        </div>
      </div>
    </AppShell>
  );
}
