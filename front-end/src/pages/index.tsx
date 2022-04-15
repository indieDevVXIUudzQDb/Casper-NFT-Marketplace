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
  SimpleGrid,
  Title,
} from "@mantine/core";
import { Wallet } from "tabler-icons-react";

import { MyCard } from "../components/MyCard";
import { mockData } from "../mockData";
import styles from "../styles/dashboard-cyber.module.scss";
import MainLinks from "./_mainLinks";
import User from "./_user";
import {
  accountInformation,
  EVENT_STREAM_ADDRESS,
  getActiveAccountBalance,
  subscribeToContractEvents,
} from "../utils/cep47_utils";
import { EventStream } from "casper-js-sdk";
import { isConnected } from "casper-js-sdk/dist/lib/Signer";

const CustomHeader = (props: { address: string }) => {
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
          {props.address}
          <Wallet size={16} />
        </ActionIcon>
      </Group>
    </Header>
  );
};

const CustomNavbar = (props: { connected: boolean }) => {
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
            <User connected={props.connected} />
          </Navbar.Section>
        </div>
        <div className={styles.lineRight}>
          <div className={styles.scanner} />
        </div>
      </div>
    </Navbar>
  );
};

export default function DashboardCyber() {
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
  return (
    <AppShell
      padding="md"
      navbar={<CustomNavbar connected={connected} />}
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
