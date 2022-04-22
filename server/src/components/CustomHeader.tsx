import React from "react";

import {
  Burger,
  Button,
  Group,
  Header,
  MediaQuery,
  useMantineTheme,
} from "@mantine/core";
import { useClipboard } from "@mantine/hooks";
import { Signer } from "casper-js-sdk";
import { toast, Toaster } from "react-hot-toast";
import { LockOpen, Wallet } from "tabler-icons-react";

import styles from "../styles/dashboard-cyber.module.scss";
import { toastConfig } from "../toastConfig";

export const addressShortener = (address: string) => {
  const maxLength = 6;
  const start = address.substring(0, maxLength);
  const end = address.substring(address.length - maxLength, address.length);
  return `${start}…${end}`;
};

export const CustomHeader = (props: {
  address: string | null;
  locked: boolean;
  menuOpen: boolean;
  setMenuOpen: (o: boolean) => void;
}) => {
  const theme = useMantineTheme();
  const clipboard = useClipboard({ timeout: 500 });

  return (
    <Header height={60}>
      <Toaster />
      {/* Handle other responsive styles with MediaQuery component or createStyles function */}
      <MediaQuery smallerThan="sm" styles={{ display: "none" }}>
        <Group sx={{ height: "100%" }} px={20} position="apart">
          <div
            style={{
              marginLeft: "8%",
            }}
          >
            <a href={""} className={styles.neonText}>
              GALACTIC NFTs
            </a>
          </div>
          <div />
          {/* eslint-disable-next-line no-nested-ternary */}
          {props.locked ? (
            <Button
              color={"gray"}
              onClick={() => Signer.sendConnectionRequest()}
            >
              <LockOpen size={16} /> &nbsp; Unlock Casper Signer
            </Button>
          ) : props.address && props.address !== "" ? (
            <Button
              color={clipboard.copied ? "teal" : "blue"}
              onClick={() => {
                clipboard.copy(props.address);
                toast.success("Address copied to clipboard!", toastConfig);
              }}
            >
              <Wallet size={16} /> &nbsp; {addressShortener(props.address)}
            </Button>
          ) : (
            <Button
              color={"green"}
              onClick={() => Signer.sendConnectionRequest()}
            >
              <Wallet size={16} /> &nbsp; Connect Casper Signer
            </Button>
          )}
        </Group>
      </MediaQuery>
      <MediaQuery largerThan="sm" styles={{ display: "none" }}>
        <Group sx={{ height: "100%" }} px={20} position="apart">
          <Burger
            opened={props.menuOpen}
            onClick={() => props.setMenuOpen(!props.menuOpen)}
            size="sm"
            color={theme.colors.gray[6]}
            mr="xl"
          />
          <div>
            <a href={""} className={styles.neonText}>
              GALACTIC NFTs
            </a>
          </div>
          <div />
          {/* eslint-disable-next-line no-nested-ternary */}
          {props.locked ? (
            <Button
              color={"gray"}
              onClick={() => Signer.sendConnectionRequest()}
            >
              <LockOpen size={16} /> &nbsp; Unlock
            </Button>
          ) : props.address && props.address !== "" ? (
            <Button
              color={clipboard.copied ? "teal" : "blue"}
              onClick={() => {
                clipboard.copy(props.address);
                toast.success("Address copied to clipboard!", toastConfig);
              }}
            >
              <Wallet size={16} /> &nbsp; {addressShortener(props.address)}
            </Button>
          ) : (
            <Button
              color={"green"}
              onClick={() => Signer.sendConnectionRequest()}
            >
              <Wallet size={16} /> &nbsp; Connect
            </Button>
          )}
        </Group>
      </MediaQuery>
    </Header>
  );
};
