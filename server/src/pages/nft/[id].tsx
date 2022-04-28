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
import { Prism } from "@mantine/prism";
import { EventStream, Signer } from "casper-js-sdk";
import { useRouter } from "next/router";
import { toast, Toaster } from "react-hot-toast";

import { CustomHeader } from "../../components/CustomHeader";
import { CustomNavbar } from "../../components/CustomNavbar";
import { SellModal } from "../../components/SellModal";
import styles from "../../styles/dashboard-cyber.module.scss";
import { toastConfig } from "../../toastConfig";
import {
  getNFT,
  NFT,
  subscribeToContractEvents,
} from "../../utils/cep47_utils";
import { MarketItem } from "../../utils/marketClient";
import {
  approveSell,
  getMarketItem,
  triggerCreateMarketItemDeploy,
  triggerProcessSale,
} from "../../utils/marketUtils";
import { CustomBackground } from "../../components/CustomBackground";

export default function DashboardCyber() {
  const [address, setAddress] = useState(null);
  const [connected, setConnected] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const [locked, setLocked] = useState(false);
  const [item, setItem] = useState<NFT | null>();
  const [marketItem, setMarketItem] = useState<MarketItem | null>();
  const [opened, setOpened] = useState(false);

  const router = useRouter();
  const { id } = router.query;

  // eslint-disable-next-line consistent-return
  const checkForMarketItem = async (itemToSync: NFT) => {
    const result = await getMarketItem(itemToSync);
    if (result) {
      console.log(result);
      setMarketItem(result);
    }
    return result;
  };

  const retrieveNFT = async (): Promise<NFT | null> => {
    const result = await getNFT(Number(id));
    if (result) {
      console.log(result);
      setItem(result);
    }
    return result;
  };

  const updateState = async () => {
    try {
      setConnected(await Signer.isConnected());
      const nftItem = await retrieveNFT();
      if (nftItem) {
        checkForMarketItem(nftItem);
      }
    } catch (err) {
      console.error(err);
    }
  };

  const sellNFT = async (sellItem: NFT, amount: string) => {
    const result = await toast.promise(
      triggerCreateMarketItemDeploy(sellItem, amount),
      {
        loading: "Listing NFT on Market",
        success: "Listed NFT",
        error: "Error listing NFT",
      },
      toastConfig
    );
    if (result) {
      console.log(result);
    }
  };

  const approveSellNFT = async (sellItem: NFT) => {
    const result = await toast.promise(
      approveSell(sellItem),
      {
        loading: "Approving NFT for Selling",
        success: "Approved NFT for Selling",
        error: "Error approving NFT",
      },
      toastConfig
    );
    if (result) {
      console.log(result);
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
      toast.success("Connected to Signer!", toastConfig);
      updateState();
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
  // const onBurnClick = () => {};

  const onSellClick = (sellItem: NFT, amount: string) => {
    if (sellItem) {
      sellNFT(sellItem, amount);
    }
  };

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
        {item ? (
          <SellModal
            opened={opened}
            setOpened={setOpened}
            onApproveClick={() => approveSell(item)}
            onSellClick={onSellClick}
            item={item}
          />
        ) : null}

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
              <b>JSON Data: </b>
            </Group>
            <Group position={"left"} grow>
              <Prism language={"json"} color={"blue"}>
                {item?.meta.get("json_data") || ""}
              </Prism>
            </Group>
            {/* eslint-disable-next-line no-nested-ternary */}
            {!connected ? (
              <Group position={"left"} grow>
                <Button color={"gray"}>Wallet Locked</Button>
                <div />
              </Group>
            ) : /* eslint-disable-next-line no-nested-ternary */
            locked ? (
              <Group position={"left"} grow>
                <Button color={"gray"}>Wallet Locked</Button>
                <div />
              </Group>
            ) : // eslint-disable-next-line no-nested-ternary
            item.isOwner && marketItem && marketItem.available ? (
              <Group position={"left"} grow>
                <Button
                  color={"yellow"}
                  onClick={() => {
                    // setOpened(true);
                  }}
                >
                  Cancel Sale
                </Button>
                <div />
              </Group>
            ) : // eslint-disable-next-line no-nested-ternary
            item.isOwner && item.isApproved ? (
              <Group position={"left"} grow>
                <Button
                  onClick={() => {
                    setOpened(true);
                  }}
                >
                  Sell
                </Button>
                <div />
              </Group>
            ) : // eslint-disable-next-line no-nested-ternary
            item.isOwner ? (
              <Group position={"left"} grow>
                <Button
                  color={"green"}
                  onClick={() => {
                    approveSellNFT(item);
                  }}
                >
                  Approve for Sale
                </Button>
                <div />
              </Group>
            ) : marketItem && !item.isOwner ? (
              <>
                <Group position={"left"}>
                  <p>
                    <b>Price:</b> <br />{" "}
                    <b className={"text-3xl"}>{marketItem?.askingPrice} CSPR</b>
                  </p>
                </Group>
                <Group position={"apart"} grow className={"mt-2"}>
                  <Button
                    color={"green"}
                    disabled={!marketItem?.available}
                    onClick={() => {
                      triggerProcessSale(marketItem);
                    }}
                  >
                    Buy Now
                  </Button>
                </Group>
              </>
            ) : null}
          </SimpleGrid>
        </Card>
      ) : null}

      <div className={styles.bg}>
        <CustomBackground />
      </div>
    </AppShell>
  );
}
