import React from "react";

import {
  Box,
  Group,
  Text,
  UnstyledButton,
  useMantineTheme,
} from "@mantine/core";
import { Signer } from "casper-js-sdk";
import { Wallet } from "tabler-icons-react";

import { MainLink } from "./_mainLinks";

export default function User(props: { connected: boolean }) {
  const theme = useMantineTheme();
  return (
    <>
      <Box
        sx={{
          paddingTop: theme.spacing.sm,
          borderTop: `1px solid ${
            theme.colorScheme === "dark"
              ? theme.colors.dark[4]
              : theme.colors.gray[2]
          }`,
        }}
      >
        <UnstyledButton
          sx={{
            display: "block",
            width: "100%",
            padding: theme.spacing.xs,
            borderRadius: theme.radius.sm,
            color:
              theme.colorScheme === "dark" ? theme.colors.dark[0] : theme.black,

            "&:hover": {
              backgroundColor:
                theme.colorScheme === "dark"
                  ? theme.colors.dark[6]
                  : theme.colors.gray[0],
            },
          }}
        >
          <Group>
            <Box sx={{ flex: 1 }}>
              {props.connected ? (
                <>
                  <Text weight={700} color={"green"}>
                    <b>&bull; &nbsp;</b> Connected to Casper Signer
                  </Text>
                </>
              ) : null}
            </Box>
          </Group>
        </UnstyledButton>
      </Box>
      <Box sx={{ flex: 1 }}>
        {props.connected ? (
          <>
            <MainLink
              color={"red"}
              func={async () => {
                await window.casperlabsHelper.disconnectFromSite();
              }}
              icon={<Wallet />}
              label={"Disconnect Casper Signer"}
            />
          </>
        ) : (
          <MainLink
            color={"green"}
            func={async () => {
              await Signer.sendConnectionRequest();
            }}
            icon={<Wallet />}
            label={"Connect to Casper Signer"}
          />
        )}
      </Box>
    </>
  );
}
