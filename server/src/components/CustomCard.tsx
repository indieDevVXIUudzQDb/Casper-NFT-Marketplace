import React from "react";

import { Button, Card, Group, Title } from "@mantine/core";

import { prefix } from "../prefix";

interface Props {
  image: string;
  title: string;
  description: string;
  buttonText: string;
}

export const CustomCard = (props: Props) => {
  const { image, title, description, buttonText } = props;
  return (
    <div style={{}}>
      <Card shadow="sm" p="lg">
        <Card.Section>
          <div style={{ textAlign: "center" }}>
            <img
              style={{ width: "80%", padding: "1em" }}
              src={`${prefix}${image}`}
              alt={title}
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
