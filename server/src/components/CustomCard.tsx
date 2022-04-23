import React from "react";

import { Button, Card, Group, Image, Title } from "@mantine/core";

import { textShortener } from "../utils/utils";

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
            <Image
              src={image}
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

        <Group position="apart" style={{ marginBottom: 5 }}>
          <Title align={"center"}>{title}</Title>
        </Group>
        {textShortener(description, 200)}
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
