import styles from "../styles/dashboard-cyber.module.scss";
import React from "react";

export const CustomBackground = () => {
  return (
    <div className={styles.bg}>
      <div className={styles.starField}>
        <div className={styles.layer}></div>
        <div className={styles.layer}></div>
        <div className={styles.layer}></div>
        <div className={styles.layer}></div>
        <div className={styles.layer}></div>
      </div>
    </div>
  );
};
