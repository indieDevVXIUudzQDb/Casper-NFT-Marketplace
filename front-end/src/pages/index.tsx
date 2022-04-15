import React, { useState } from "react";

import {
  AppShell,
  Navbar,
  Header,
  Group,
  ActionIcon,
  MediaQuery,
  Burger,
  useMantineTheme,
} from "@mantine/core";
import { Wallet } from "tabler-icons-react";

import styles from "../styles/dashboard-cyber.module.scss";
import { MainLinks } from "./_mainLinks";
import { User } from "./_user";

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
        <div></div>
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

export default function DashboardCyber() {
  return (
    <AppShell padding="md" navbar={<CustomNavbar />} header={<CustomHeader />}>
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
