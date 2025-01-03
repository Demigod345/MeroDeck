// @ts-nocheck

import React, { useState, useEffect } from 'react';
import { JoinGameRequest, JoinGameResponse } from '../../api/clientApi';
import { LogicApiDataSource } from '../../api/dataSource/LogicApiDataSource';
import { ResponseData } from '@calimero-is-near/calimero-p2p-sdk';
import { getConfigAndJwt } from '../../api/dataSource/LogicApiDataSource';
import { useNavigate } from 'react-router-dom';
import styles from '../../styles/JoinGame.module.css';
import contractData from '../../constants/contractData.json';
import {
  RpcProvider,
  Contract,
  CallData,
  WalletAccount,
  cairo,
} from 'starknet';
import { connect } from 'get-starknet';
import { getStarknetRpcUrl } from '../../utils/env';
import { getSystemErrorMap } from 'util';

const shortenChips = (chips: string) => {
  while (chips.length < 19) {
    chips = '0' + chips;
  }
  if (chips.length <= 12) {
    return '0.0';
  }
  const shortened = chips.slice(0, -12);
  return `${shortened.slice(0, -6)}.${shortened.slice(-6)}`;
};

export default function Join() {
  const [playerName, setPlayerName] = useState('');
  const [chips, setChips] = useState('');
  const [availableChips, setAvailableChips] = useState('0');
  const [buyChips, setBuyChips] = useState(0);
  const [connection, setConnection] = useState(null);
  const [address, setAddress] = useState('');
  const [mafiaContract, setMafiaContract] = useState(null);

  const navigate = useNavigate();
  const provider = new RpcProvider({
    nodeUrl: getStarknetRpcUrl(),
  });

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

  async function joinGame(request: JoinGameRequest) {
    const result: ResponseData<JoinGameResponse> =
      await new LogicApiDataSource().joinGame(request);

    if (result.error) {
      console.error('Error joining game', result.error);
      window.alert('Error joining game');
    }

    console.log('Game joined', result.data);

    if (result.data !== null && result.data !== undefined) {
      localStorage.setItem('playerIndex', result.data.toString());
    }

    // this is not working as index can be 0
    //set the index in local storage
    // if (result.data) {
    //   localStorage.setItem('playerIndex', result.data.this_player_index.toString());
    // }
  }

  async function buyChip() {
    //Buy the chips using contract call
    console.log('Buying chips haha: ', buyChips);
    if (buyChips == 0) return;
    const call = await connection.execute([
      {
        contractAddress: contractData.contractAddress,
        entrypoint: 'mint',
        calldata: CallData.compile({
          to: address,
          amount: cairo.uint256(buyChips.toString()),
        }),
      },
    ]);

    console.log(call);
    await provider.waitForTransaction(call.transaction_hash);
  }

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    console.log('Player Name:', playerName);
    console.log('Chips:', chips);

    //Buy the chips using contract call
    await buyChip();
    //Do an rpc call to join game and pass the player name and chips

    //Getting calimero public key from jwt
    const { jwtObject, config, error } = getConfigAndJwt();
    if (error) {
      return { error };
    }

    await joinGame({
      request: {
        public_key: jwtObject.executor_public_key,
        chips: parseInt(chips),
        player_name: playerName,
      },
    });

    //Checking if the player index is set
    const playerIndex = localStorage.getItem('playerIndex');
    console.log('Player Index:', playerIndex);
    //redirect to play page
    if (playerIndex !== null) {
      navigate('/play');
    }
  };

  return (
    <div className={styles.formContainer}>
      <h1>Join Game</h1>
      <form onSubmit={handleSubmit} className={styles.form}>
        <div className={styles.inputFieldContainer}>
          <label htmlFor="playerName">Player Name:</label>
          <input
            type="text"
            id="playerName"
            value={playerName}
            onChange={(e) => setPlayerName(e.target.value)}
            required
            className={styles.inputField}
          />
        </div>
        <p>Available Chips: {availableChips}</p>
        <div>
          <label htmlFor="chips">Buy some Chips?</label>
          <input
            type="number"
            id="chips"
            value={chips}
            onChange={(e) => {
              const value = e.target.value;
              const _buyChips = value * 10 ** 18;
              console.log(_buyChips);
              if (value >= 1000) {
                window.alert('Too large!');
                return;
              }
              setBuyChips(_buyChips);
              setChips(value);
            }}
            // required
            className={styles.inputField}
          />
        </div>
        <button type="submit" className={styles.submitButton}>
          Join Game
        </button>
      </form>
    </div>
  );
}
