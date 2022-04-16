import React from "react";

import {
  UnstyledButton,
  Group,
  Text,
  Box,
  useMantineTheme,
} from "@mantine/core";
import { MainLink } from "./_mainLinks";
import { Wallet } from "tabler-icons-react";

export default function User(props: {
  connected: boolean;
  updateAccountInformation: () => void;
}) {
  const theme = useMantineTheme();

  return (
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
              <Text weight={700} color={"green"}>
                <b>&bull; &nbsp;</b> Connected to Casper Signer
              </Text>
            ) : (
              <MainLink
                color={"red"}
                func={async () => {
                  await window.casperlabsHelper.requestConnection();
                  props.updateAccountInformation();
                }}
                icon={<Wallet />}
                label={"Connect to Casper Signer"}
              />
            )}
          </Box>
        </Group>
      </UnstyledButton>
    </Box>
  );
}
