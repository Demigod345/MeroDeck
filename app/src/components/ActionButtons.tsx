import React from 'react';
import styles from '../styles/ActionButtons.module.css'

import {
  CreateActionRequest,
  CreateActionResponse,
  startGameRequest,
  startGameResponse,
} from '../api/clientApi';
import { ResponseData } from '@calimero-is-near/calimero-p2p-sdk';
import { LogicApiDataSource } from '../api/dataSource/LogicApiDataSource';


export default function ActionButtons() {
  const [amount, setAmount] = React.useState(50);

  //get player index from local storage
  const playerIndex = localStorage.getItem('playerIndex');


  // Setting functions ========================

  async function makeAction(request: CreateActionRequest) {

    const result: ResponseData<CreateActionResponse> = 
     await new LogicApiDataSource().createAction(request);

    if (result.error) {
      console.error('Error creating action', result.error);
    }

    console.log('Action created', result.data);
  }

  async function startgame(request: startGameRequest) {

    const result: ResponseData<startGameResponse> = 
      await new LogicApiDataSource().startGame(request);

    if (result.error) {
      console.error('Error starting game', result.error);
    }

    window.alert('Game started');
  }


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
          <button 
            className={styles.startgamebutton}
            onClick={() => startgame({request:{}})}
          >
              Start Game
          </button>
          <button 
            className={styles.button}
            onClick={() => makeAction({request:{ action: 'Check', player_index: playerIndex ? Number(playerIndex) : 0 }})}
          >
              Check
          </button>
          <button 
            className={styles.button} 
            onClick={() => makeAction({request:{ action: 'Fold', player_index: playerIndex ? Number(playerIndex) : 0 }})}
          >
            Fold
          </button>
          <button 
            className={styles.button} 
            onClick={() => makeAction({request:{ action: 'Call', player_index: playerIndex ? Number(playerIndex) : 0 }})}
          >
            Call
          </button>
          <button 
            className={styles.button} 
            onClick={() => makeAction({request:{ action: {'Raise': amount}, player_index: playerIndex ? Number(playerIndex) : 0 }})}
          >
            Raise
          </button>
          <button 
            className={styles.button} 
            onClick={() => makeAction({request:{ action: {'Bet': amount}, player_index: playerIndex ? Number(playerIndex) : 0 }})}
          >
            Bet
          </button>
        </div>
      </div>
    );
}