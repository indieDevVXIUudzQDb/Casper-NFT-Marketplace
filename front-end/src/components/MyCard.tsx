import React from "react";

import { Badge, Button, Card, Group, Title } from "@mantine/core";

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
          <div style={{ textAlign: "center" }}>
            <img
              style={{ maxWidth: 290, padding: "1em" }}
              src={`${prefix}${image}`}
            />
          </div>
        </Card.Section>

        <Group position="apart" style={{ marginBottom: 5 }}>
          <Title align={"center"}>{title}</Title>
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
