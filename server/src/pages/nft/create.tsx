import React, { useEffect, useState } from "react";

import {
  AppShell,
  Box,
  Button,
  Group,
  Textarea,
  TextInput,
  Title,
} from "@mantine/core";
import { useForm } from "@mantine/hooks";
import { Signer } from "casper-js-sdk";
import { toast, Toaster } from "react-hot-toast";

import { CustomHeader } from "../../components/CustomHeader";
import { CustomNavbar } from "../../components/CustomNavbar";
import styles from "../../styles/dashboard-cyber.module.scss";
import { toastConfig } from "../../toastConfig";
import {
  getDeployResult,
  initClient,
  triggerMintDeploy,
} from "../../utils/cep47_utils";
import { NFTMeta } from "../../utils/types";

export default function Mint() {
  const [address, setAddress] = useState(null);
  const [menuOpen, setMenuOpen] = useState(false);
  const [connected, setConnected] = useState(false);
  const [locked, setLocked] = useState(false);

  // Without the timeout it doesn't always work properly
  setTimeout(async () => {
    try {
      setConnected(await Signer.isConnected());
    } catch (err) {
      console.log(err);
    }
  }, 100);

  // useEffect(() => {
  //   const es = new EventStream(EVENT_STREAM_ADDRESS!);
  //   subscribeToContractEvents(es, () => getActiveAccountBalance());
  // }, []);

  useEffect(() => {
    window.addEventListener("signer:connected", (msg) => {
      setConnected(true);
      // @ts-ignore
      setLocked(!msg.detail.isUnlocked);
      // @ts-ignore
      setAddress(msg.detail.activeKey);
      toast.success("Connected to Signer!", toastConfig);
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

  const form = useForm({
    initialValues: {
      name: "test",
      symbol: "TEST",
      url: "test.com",
      image_url: "http://localhost:3000/assets/planets/1.png",
      json_data: `{
  "hello": "world",
  "list": [
    1,
    2,
    3
  ],
  "nested_1": {
    "nested_2": "Im nested 2 deep"
  }
}`,
      description: "test description",
    },
  });
  const mintNFT = async (item: NFTMeta) => {
    const { cep47 } = await initClient();
    if (!cep47) return;
    const totalSupply = await cep47.totalSupply();
    const startIndex = totalSupply;

    const mapped: Map<string, string> = new Map(Object.entries(item));
    const mintDeployHash = await triggerMintDeploy([`${startIndex}`], [mapped]);
    if (mintDeployHash) {
      toast.promise(
        getDeployResult(mintDeployHash),
        {
          loading: "Minting in progress",
          success: "Minting Successful",
          error: "Error when minting",
        },
        toastConfig
      );
    } else {
      toast.error("Failed to mint NFT.", toastConfig);
    }
  };

  const createNFT = async (values: {
    name: string;
    symbol: string;
    json_data: string;
    url: string;
    image_url: string;
    description: string;
  }) => {
    try {
      // Test meta is parsable
      JSON.parse(values.json_data);
      const item = {
        ...values,
      };
      console.log({ item });
      await mintNFT(item);
    } catch (e) {
      console.log(e);
      toast.error(
        "Invalid Custom Meta format. Expecting JSON Object.",
        toastConfig
      );
    }
  };
  return (
    <AppShell
      padding="md"
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
      <Toaster />

      <Title order={1}>Create your NFT</Title>
      <Box sx={{ maxWidth: 300 }} mx="auto">
        <form onSubmit={form.onSubmit((values) => createNFT(values))}>
          <TextInput required label="Name" {...form.getInputProps("name")} />
          <TextInput
            required
            label="Symbol"
            {...form.getInputProps("symbol")}
          />
          <TextInput
            required
            label="Description"
            {...form.getInputProps("description")}
          />
          <TextInput
            required
            label="Image URL"
            {...form.getInputProps("image_url")}
          />
          <TextInput required label="URL" {...form.getInputProps("url")} />
          {/* eslint-disable-next-line react/jsx-no-undef */}
          <Textarea
            // placeholder="Enter"
            label="JSON Data"
            autosize
            {...form.getInputProps("json_data")}
            minRows={2}
          />

          <Group position="right" mt="md">
            <Button type="submit">Create</Button>
          </Group>
        </form>
      </Box>
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
