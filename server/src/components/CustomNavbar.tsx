import {Navbar} from "@mantine/core";
import styles from "../styles/dashboard-cyber.module.scss";
import MainLinks from "../pages/_mainLinks";
import User from "../pages/_user";
import React from "react";

export const CustomNavbar = (props: { connected: boolean, updateAccountInformation:()=>void }) => {
    return (
        <Navbar
            p="md"
            hiddenBreakpoint="sm"
            width={{ sm: 300, lg: 400 }}
            className={"border-r-0"}
        >
            <div className={styles.copyBox}>
                <div className={styles.inner}>
                    <div className={styles.lineLeft}>
                        <div className={styles.scanner} />
                    </div>
                    <Navbar.Section grow mt="xs">
                        <MainLinks />
                    </Navbar.Section>
                    <Navbar.Section>
                        <User connected={props.connected} updateAccountInformation={props.updateAccountInformation} />
                    </Navbar.Section>
                </div>
                <div className={styles.lineRight}>
                    <div className={styles.scanner} />
                </div>
            </div>
        </Navbar>
    );
};