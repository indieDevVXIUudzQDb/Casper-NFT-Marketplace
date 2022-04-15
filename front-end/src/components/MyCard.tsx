import React from "react";

import { Badge, Button, Card, Group } from "@mantine/core";

import { prefix } from "../prefix";

interface Props {
  image: string;
  title: string;
  description: string;
  buttonText: string;
  key: number;
}

export const MyCard = (props: Props) => {
  const { image, title, description, buttonText, key } = props;
  return (
    <div style={{ width: 340 }} key={key}>
      <Card shadow="sm" p="lg">
        <Card.Section>
          <img style={{ width: 300 }} src={`${prefix}${image}`} />
        </Card.Section>

        <Group position="apart" style={{ marginBottom: 5 }}>
          {title}
          <Badge color="pink" variant="light">
            On Sale
          </Badge>
        </Group>

        {description}

        <Button
          variant="light"
          color="blue"
          fullWidth
          style={{ marginTop: 14 }}
        >
          {buttonText}
        </Button>
      </Card>
    </div>
  );
};
