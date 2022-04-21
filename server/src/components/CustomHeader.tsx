import React, { useState } from "react";

import {
  Burger,
  Button,
  Group,
  Header,
  MediaQuery,
  useMantineTheme,
} from "@mantine/core";
import { useClipboard } from "@mantine/hooks";
import { Lock, Wallet } from "tabler-icons-react";

import styles from "../styles/dashboard-cyber.module.scss";
import { Signer } from "casper-js-sdk";
import { toast, Toaster } from "react-hot-toast";

export const addressShortener = (address: string) => {
  const maxLength = 6;
  const start = address.substring(0, maxLength);
  const end = address.substring(address.length - maxLength, address.length);
  return `${start}…${end}`;
};

export const CustomHeader = (props: {
  address: string | null;
  locked: boolean;
}) => {
  const [opened, setOpened] = useState(false);

  const theme = useMantineTheme();
  const clipboard = useClipboard({ timeout: 500 });

  return (
    <Header height={60}>
      <Toaster />
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
        {/* eslint-disable-next-line no-nested-ternary */}
        {props.locked ? (
          <Button color={"gray"} onClick={() => Signer.sendConnectionRequest()}>
            <Lock size={16} />
          </Button>
        ) : props.address ? (
          <Button
            color={clipboard.copied ? "teal" : "blue"}
            onClick={() => {
              clipboard.copy(props.address);
              toast.success("Address copied to clipboard!");
            }}
          >
            <Wallet size={16} /> &nbsp; {addressShortener(props.address)}
          </Button>
        ) : (
          <Button
            color={clipboard.copied ? "teal" : "blue"}
            onClick={() => clipboard.copy(props.address)}
          >
            <Wallet size={16} />
          </Button>
        )}
      </Group>
    </Header>
  );
};
