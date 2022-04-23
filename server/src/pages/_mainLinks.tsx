import React from "react";

import { Group, Text, ThemeIcon, UnstyledButton } from "@mantine/core";
import { useRouter } from "next/router";
import { PictureInPicture, Plus, ShoppingCart } from "tabler-icons-react";

interface MainLinkProps {
  icon: React.ReactNode;
  color: string;
  label: string;
  func: () => void;
}

export function MainLink({ icon, color, label, func }: MainLinkProps) {
  return (
    <UnstyledButton
      onClick={func}
      sx={(theme) => ({
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
      })}
    >
      <Group>
        <ThemeIcon color={color} variant="light">
          {icon}
        </ThemeIcon>

        <Text size="sm">{label}</Text>
      </Group>
    </UnstyledButton>
  );
}

export default function MainLinks() {
  const router = useRouter();

  const data = [
    {
      icon: <ShoppingCart size={16} />,
      color: "blue",
      label: "Browse Market",
      func: () => router.push("/"),
    },
    {
      icon: <PictureInPicture size={16} />,
      color: "blue",
      label: "My NFTs",
      func: () => router.push("/my-nfts"),
    },
    {
      icon: <Plus size={16} />,
      color: "blue",
      label: "Create NFT",
      func: () => router.push("/nft/create"),
    },
  ];

  const links = data.map((link) => <MainLink {...link} key={link.label} />);
  return <div>{links}</div>;
}
