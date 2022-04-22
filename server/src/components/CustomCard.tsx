import React from "react";

import { Button, Card, Group, Image, Title } from "@mantine/core";

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
            <Image
              src={`${prefix}${image}`}
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
              // style={{ padding: "3em" }}
              // onError={({ currentTarget }) => {
              //   // eslint-disable-next-line no-param-reassign
              //   currentTarget.onerror = null; // prevents looping
              //   // eslint-disable-next-line no-param-reassign
              // //   currentTarget.src =
              // //     "http://localhost:3000/logoipsum-logo-39.svg";
              // // }}
            />

            {/* <img */}
            {/*  style={{ width: "80%", padding: "1em" }} */}
            {/*  src={`${prefix}${image}`} */}
            {/*  onError={({ currentTarget }) => { */}
            {/*    // eslint-disable-next-line no-param-reassign */}
            {/*    currentTarget.onerror = null; // prevents looping */}
            {/*    // eslint-disable-next-line no-param-reassign */}
            {/*    currentTarget.src = */}
            {/*      "http://localhost:3000/logoipsum-logo-39.svg"; */}
            {/*  }} */}
            {/*  alt={title} */}
            {/* /> */}
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
