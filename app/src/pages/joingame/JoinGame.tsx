import React, { useState } from 'react';
import { JoinGameRequest, JoinGameResponse } from '../../api/clientApi';
import { LogicApiDataSource } from '../../api/dataSource/LogicApiDataSource';
import { ResponseData } from '@calimero-is-near/calimero-p2p-sdk';
import { getConfigAndJwt } from '../../api/dataSource/LogicApiDataSource';
import { useNavigate } from 'react-router-dom';


export default function Join() {
  const [playerName, setPlayerName] = useState('');
  const [chips, setChips] = useState('');
  const navigate = useNavigate();

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

  async function buyChips() {
    //Buy the chips using contract call
    console.log("Buying chips haha");
  }




  const handleSubmit =  async (event: React.FormEvent) => {
    event.preventDefault();
    console.log('Player Name:', playerName);
    console.log('Chips:', chips);


    //Buy the chips using contract call
    await buyChips();
    //Do an rpc call to join game and pass the player name and chips

    //Getting calimero public key from jwt
    const { jwtObject, config, error } = getConfigAndJwt();
        if (error) {
          return { error };
        }
    
    await joinGame({request:{ 
      public_key: jwtObject.executor_public_key,
      chips: parseInt(chips),
      player_name: playerName}});

    //Checking if the player index is set
    const playerIndex = localStorage.getItem('playerIndex');
    console.log('Player Index:', playerIndex);
    //redirect to play page
    if (playerIndex !== null) {
      navigate('/play');
    }

  };

  return (
    <div>
      <h1>Join Game</h1>
      <form onSubmit={handleSubmit}>
        <div>
          <label htmlFor="playerName">Player Name:</label>
          <input
            type="text"
            id="playerName"
            value={playerName}
            onChange={(e) => setPlayerName(e.target.value)}
            required
          />
        </div>
        <div>
          <label htmlFor="chips">Amount of Chips:</label>
          <input
            type="number"
            id="chips"
            value={chips}
            onChange={(e) => setChips(e.target.value)}
            required
          />
        </div>
        <button type="submit">Join Game</button>
      </form>
    </div>
  );
}