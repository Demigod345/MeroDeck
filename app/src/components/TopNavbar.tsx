import styles from '../styles/TopNavbar.module.css'
import React from 'react';

export default function TopNavbar() {
  return (
    <header className={styles.header}>
      <div className={styles.logo}>MeroDeck</div>
      <div className={styles.wallet}>Starknet Wallet</div>
    </header>
  )
}

