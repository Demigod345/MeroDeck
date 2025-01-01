import React from 'react';
import styles from '../styles/ActionButtons.module.css'

export default function ActionButtons() {
  const [amount, setAmount] = React.useState(50);
  return (
      <div className={styles.actions}>
        <div className={styles.slidercontainer}>
        <span className={styles.amount}>{amount}</span>
          <input
          type="range"
          min="1"
          max="100"
          value={amount}
          onChange={(e) => setAmount(Number(e.target.value))}
          className={styles.slider}
        />
        
        </div>
        
        <div className={styles.actionbuttons}>
          <button className={styles.button}>Fold</button>
          <button className={styles.button}>Call</button>
          <button className={styles.button}>Raise</button>
          <button className={styles.button}>Bet</button>
        </div>
      </div>
    );
}