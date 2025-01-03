// @ts-nocheck

import React, { useState, useEffect } from 'react';
import styles from '../styles/ActionButtons.module.css'

import {
  CreateActionRequest,
  CreateActionResponse,
  startGameRequest,
  startGameResponse,
} from '../api/clientApi';
import { ResponseData } from '@calimero-is-near/calimero-p2p-sdk';
import { LogicApiDataSource } from '../api/dataSource/LogicApiDataSource';

import { RpcProvider, Contract, WalletAccount, CallData, shortString, cairo } from "starknet";
import { connect } from "get-starknet";
import { getStarknetRpcUrl } from '../utils/env';
import contractData from '../constants/contractData.json';

export function twoFeltToString(x:any, y:any) {
  const str1 = shortString.decodeShortString(x);
  const str2 = shortString.decodeShortString(y);
return str1 + str2;
}

export function stringToTwoFelt(str: string) {
  const arrStr = shortString.splitLongString(str);
  const x = shortString.encodeShortString(arrStr[0]);
  const y = shortString.encodeShortString(arrStr[1]);
return { x, y };
}


export default function ActionButtons() {
  const [amount, setAmount] = React.useState(50);

  const [connection, setConnection] = useState(null);
  const [address, setAddress] = useState("");
  const [mafiaContract, setMafiaContract] = useState(null);
  const [availableChips, setAvailableChips] = useState('0');

  const provider = new RpcProvider({
      nodeUrl: getStarknetRpcUrl(),
    });
  
  //get player index from local storage
  const playerIndex = localStorage.getItem('playerIndex');

  useEffect(() => {
    const handleConnectWallet = async () => {
      try {
        const selectedWalletSWO = await connect({ modalTheme: 'dark' });
        const wallet = await new WalletAccount(
          { nodeUrl: getStarknetRpcUrl() },
          selectedWalletSWO,
        );

        if (wallet) {
          setConnection(wallet);
          setAddress(wallet.walletProvider.selectedAddress);
        }
      } catch (error) {
        console.error('Error connecting wallet:', error);
        // toast.error("Failed to connect wallet. Please try again.");
      }
    };

    handleConnectWallet();
    fetchAvailableChips();
  }, [address]);

  const fetchAvailableChips = async () => {
    const contract = await getContract();
    const balance = await contract.balanceOf(address);
    console.log(balance.toString());
    setAvailableChips(shortenChips(balance.toString()));
    // console.log('Available Chips:', balance);
    // console.log(shortenChips(balance.toString()));
  };
  const getContract = async () => {
    if (mafiaContract != null) {
      return mafiaContract;
    }

    try {
      const { abi: contractAbi } = await provider.getClassAt(
        contractData.contractAddress,
      );
      if (contractAbi === undefined) {
        throw new Error('No ABI found for the contract.');
      }
      const contract = new Contract(
        contractAbi,
        contractData.contractAddress,
        provider,
      );
      setMafiaContract(contract);
      return contract;
    } catch (error) {
      console.error('Error getting contract:', error);
      toast.error(
        'Failed to interact with the game contract. Please try again.',
      );
      return null;
    }
  };


  // Setting functions ========================

  async function makeAction(request: CreateActionRequest) {

    const result: ResponseData<CreateActionResponse> = 
     await new LogicApiDataSource().createAction(request);

    if (result.error) {
      console.error('Error creating action', result.error);
    }
    
    const amountToAdd = result.data;
    console.log('Amount to add', result.data); // Working here

    // 55 -> 55 * 10 **18 -> string
    // const call = await connection.execute([
    //   {
    //     contractAddress: contractData.contractAddress,
    //     entrypoint: 'add_to_pot',
    //     calldata: CallData.compile({
    //       gameId: 3,
    //       amount: cairo.uint256(amountToAdd.toString()),
    //     }),
    //   },
    // ]);

    // console.log(call);
    // await provider.waitForTransaction(call.transaction_hash);


    // Add to pot

    // Here it should return the amount to be added to the pot

    console.log('Action created', result.data);

    // Here sending the amount to the pot contract
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