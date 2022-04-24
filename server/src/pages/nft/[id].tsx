import React, { useEffect, useState } from "react";

import {
  Anchor,
  AppShell,
  Button,
  Card,
  Group,
  Image,
  SimpleGrid,
  Title,
} from "@mantine/core";
import { EventStream, Signer } from "casper-js-sdk";
import { useRouter } from "next/router";
import { toast, Toaster } from "react-hot-toast";
import { CustomHeader } from "../../components/CustomHeader";
import { CustomNavbar } from "../../components/CustomNavbar";
import styles from "../../styles/dashboard-cyber.module.scss";
import { toastConfig } from "../../toastConfig";
import {
  getNFT,
  RetrievedNFT,
  subscribeToContractEvents,
} from "../../utils/cep47_utils";
import { Prism } from "@mantine/prism";

export default function DashboardCyber() {
  const [address, setAddress] = useState(null);
  const [connected, setConnected] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const [locked, setLocked] = useState(false);
  const [item, setItem] = useState<RetrievedNFT | null>();

  const router = useRouter();
  const { id } = router.query;

  const [retrieving, setRetrieving] = useState(false);

  const retrieveNFT = async () => {
    if (!retrieving) {
      setRetrieving(true);
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
      setRetrieving(false);
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
      // padding="md"
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
      {item ? (
        <Card
          shadow="sm"
          p="lg"
          style={{
            margin: "3em",
            marginLeft: "3em",
            minHeight: "90%",
            // borderRadius: " 30px",
          }}
        >
          <SimpleGrid cols={1}>
            <Group position="center" style={{ marginBottom: 5 }}>
              <Title align={"center"}>{item?.meta.get("name")}</Title>
            </Group>
            <Card.Section>
              <div style={{ textAlign: "center" }}>
                <Image
                  src={item?.meta.get("image_url")}
                  height={160}
                  alt="Norway"
                  fit="contain"
                  withPlaceholder
                  placeholder={
                    <Image
                      src={`http://localhost:3000/logoipsum-logo-39.svg`}
                      height={160}
                      alt="Norway"
                      fit="contain"
                    />
                  }
                />
              </div>
            </Card.Section>
            <Group position={"left"}>
              <p>
                <b>Description: </b>
                <br />
                {item?.meta.get("description")}
              </p>
            </Group>
            <Group position={"left"}>
              <p>
                <b>URL: </b>
                <br />
                <Anchor href={item?.meta.get("url")} target="_blank">
                  {item?.meta.get("url")}
                </Anchor>
              </p>
            </Group>
            <Group position={"left"} grow>
              <p>
                <b>JSON Data: </b>
                <br />
                <Prism language={"json"} color={"blue"}>
                  {item?.meta.get("json_data") || ""}
                </Prism>
              </p>
            </Group>
            {item.isOwner ? (
              <Group position={"apart"} grow>
                <Button>Sell</Button>
                <Button color={"red"}>Burn</Button>
              </Group>
            ) : (
              <>
                {" "}
                <Group position={"left"}>
                  <p>
                    <b>Price:</b> <br />{" "}
                    <b className={"text-3xl"}>120000 CSPR</b>
                  </p>
                </Group>
                <Group position={"apart"} grow className={"mt-2"}>
                  <Button color={"green"}>Buy Now</Button>
                </Group>
              </>
            )}
          </SimpleGrid>
        </Card>
      ) : null}

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
