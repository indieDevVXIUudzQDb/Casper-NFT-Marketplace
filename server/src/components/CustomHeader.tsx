import React, {useState} from "react";
import {ActionIcon, Burger, Group, Header, MediaQuery, useMantineTheme} from "@mantine/core";
import styles from "../styles/dashboard-cyber.module.scss";
import {Wallet} from "tabler-icons-react";

export const CustomHeader = (props: { address: string }) => {
    const [opened, setOpened] = useState(false);

    const theme = useMantineTheme();

    return (
        <Header height={60}>
            {/* Handle other responsive styles with MediaQuery component or createStyles function */}
            <MediaQuery largerThan="sm" styles={{ display: "none" }}>
                <Burger
                    opened={opened}
                    onClick={() => setOpened((o) => !o)}
                    size="sm"
                    color={theme.colors.gray[6]}
                    mr="xl"
                />
            </MediaQuery>
            <Group sx={{ height: "100%" }} px={20} position="apart">
                <div
                    style={{
                        margin: "auto",
                    }}
                >
                    <a href={""} className={styles.neonText}>
                        GALACTIC NFTs
                    </a>
                </div>
                <div />
                <ActionIcon variant="default" size={30}>
                    {props.address}
                    <Wallet size={16} />
                </ActionIcon>
            </Group>
        </Header>
    );
};
