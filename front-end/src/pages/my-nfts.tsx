import React, { useEffect, useState } from "react";

import {
  AppShell,
  Navbar,
  Header,
  Group,
  ActionIcon,
  MediaQuery,
  Burger,
  useMantineTheme,
  Title,
  SimpleGrid,
} from "@mantine/core";
import { Wallet } from "tabler-icons-react";

import styles from "../styles/dashboard-cyber.module.scss";
import MainLinks from "./_mainLinks";
import User from "./_user";
import { supabaseServerSideClient } from "../utils/supabaseServerSideClient";
import { mockData } from "../mockData";
import { MyCard } from "../components/MyCard";
import {
  accountInformation,
  EVENT_STREAM_ADDRESS,
  getActiveAccountBalance,
  subscribeToContractEvents,
} from "../utils/cep47_utils";
import { EventStream } from "casper-js-sdk";

const CustomHeader = () => {
  const [opened, setOpened] = useState(false);

  const theme = useMantineTheme();

  return (
    <Header height={60}>
      {/* Handle other responsive styles with MediaQuery component or createStyles function */}
      <MediaQuery largerThan="sm" styles={{ display: "none" }}>
        <Burger
          opened={opened}
          onClick={() => setOpened((o) => !o)}
          size="sm"
          color={theme.colors.gray[6]}
          mr="xl"
        />
      </MediaQuery>
      <Group sx={{ height: "100%" }} px={20} position="apart">
        <div
          style={{
            margin: "auto",
          }}
        >
          <a href={""} className={styles.neonText}>
            GALACTIC NFTs
          </a>
        </div>
        <div />
        <ActionIcon variant="default" size={30}>
          <Wallet size={16} />
        </ActionIcon>
      </Group>
    </Header>
  );
};

const CustomNavbar = () => {
  return (
    <Navbar
      p="md"
      hiddenBreakpoint="sm"
      width={{ sm: 300, lg: 400 }}
      className={"border-r-0"}
    >
      <div className={styles.copyBox}>
        <div className={styles.inner}>
          <div className={styles.lineLeft}>
            <div className={styles.scanner} />
          </div>
          <Navbar.Section grow mt="xs">
            <MainLinks />
          </Navbar.Section>
          <Navbar.Section>
            <User />
          </Navbar.Section>
        </div>
        <div className={styles.lineRight}>
          <div className={styles.scanner} />
        </div>
      </div>
    </Navbar>
  );
};
export async function getServerSideProps(context) {
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
  const [publicKey, setPublicKey] = useState("");
  const [balance, setBalance] = useState("");
  const [nftBalance, setNFTBalance] = useState(0);
  const [tx, setTx] = useState("");
  const [to, setTo] = useState("");
  const [amount, setAmount] = useState("");
  const [connected, setConnected] = useState(false);
  const updateAccountInformation = async () => {
    const {
      textAddress,
      textBalance,
      publicKey: updatedPublicKey,
    } = await accountInformation();
    setAddress(textAddress);
    setBalance(textBalance);
    setPublicKey(updatedPublicKey);
    setNFTBalance(await getActiveAccountBalance());
    setConnected(true);
  };

  useEffect(() => {
    console.log("subscription called");
    const es = new EventStream(EVENT_STREAM_ADDRESS!);
    subscribeToContractEvents(es, () => getActiveAccountBalance());
    updateAccountInformation();
  }, []);

  console.log(items);
  return (
    <AppShell padding="md" navbar={<CustomNavbar />} header={<CustomHeader />}>
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
