import React from "react";

import { Navbar } from "@mantine/core";

import MainLinks from "../pages/_mainLinks";
import User from "../pages/_status";
import styles from "../styles/dashboard-cyber.module.scss";

export const CustomNavbar = (props: {
  connected: boolean;
  locked: boolean;
  menuOpen: boolean;
}) => {
  return (
    <Navbar
      p="md"
      hiddenBreakpoint="sm"
      width={{ sm: 300, lg: 400 }}
      // className={"border-r-0"}
      // style={{
      //   margin: "2em",
      //   borderRadius: "30px",
      //   height: "85%",
      // }}
      hidden={!props.menuOpen}
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
            <User connected={props.connected} locked={props.locked} />
          </Navbar.Section>
        </div>
        <div className={styles.lineRight}>
          <div className={styles.scanner} />
        </div>
      </div>
    </Navbar>
  );
};
