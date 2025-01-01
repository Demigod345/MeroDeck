'use client'

import React, { useState, useEffect } from 'react'
import styles from '../styles/PokerTable.module.css'
import {
  GetActivePlayerRequest,
  GetActivePlayerResponse,
  GetGameStateRequest,
  GetGameStateResponse,
} from '../api/clientApi';
import {
  LogicApiDataSource,
} from '../api/dataSource/LogicApiDataSource';
import {
  ResponseData,
} from '@calimero-is-near/calimero-p2p-sdk';




interface PlayerData {
  id: number
  player_name: string
  name: string
  connected: boolean
  cards: Card[]
  move: string
}

interface Card {
  rank: string
  suit: string
}

export default function PokerTable() {


  







  // Defining all frontend variables ==================================


  // players will be fetched from the state if null then render not connected if not then render the player
  const[playersData, setPlayersData] = useState<PlayerData[]>([])
  const[potSize, setPotSize] = useState<number>(0)
  const[thisplayerindex, setThisPlayerIndex] = useState<number>(0)
  const[currentposition, setCurrentPosition] = useState<number>(0)

  useEffect(() => {
    // Mock player data
    setPlayersData([
      {
        id: 1,
        player_name: 'Not connected',
        name: 'Not connected',
        connected: false,
        cards: [
          { rank: '', suit: '' },
          { rank: '', suit: '' },
        ],
        move: 'Not played',
      },
      {
        id: 2,
        player_name: 'Not connected',
        name: 'Not connected',
        connected: false,
        cards: [
          { rank: '', suit: '' },
          { rank: '', suit: '' },
        ],
        move: 'Not played',
      },
      {
        id: 3,
        player_name: 'Not connected',
        name: 'Not connected',
        connected: false,
        cards: [
          { rank: '', suit: '' },
          { rank: '', suit: '' },
        ],
        move: 'Not played',
      },
      {
        id: 4,
        player_name: 'Not connected',
        name: 'Not connected',
        connected: false,
        cards: [
          { rank: '', suit: '' },
          { rank: '', suit: '' },
        ],
        move: 'Not played',
      },
      {
        id: 5,
        player_name: 'Not connected',
        name: 'Not connected',
        connected: false,
        cards: [
          { rank: '', suit: '' },
          { rank: '', suit: '' },
        ],
        move: 'Not played',
      },
      {
        id: 6,
        player_name: 'Not connected',
        name: 'Not connected',
        connected: false,
        cards: [
          { rank: '', suit: '' },
          { rank: '', suit: '' },
        ],
        move: 'Not played',
      },
    ])
  }, [])

  const [communityCards, setCommunityCards] = useState<Card[]>([])

  useEffect(() => {
    // Mock community cards
    setCommunityCards([
      { rank: '', suit: '' },
      { rank: '', suit: '' },
      { rank: '', suit: '' },
      { rank: '', suit: '' },
      { rank: '', suit: '' },
    ])
  }, [])

  // Fetching functions ================================================


  async function fetchActivePlayer() {
    const params: GetActivePlayerRequest = {};
    const result: ResponseData<GetActivePlayerResponse> =
      await new LogicApiDataSource().getActivePlayer(params);
    if (result?.error) {
      console.error('Error:', result.error);
      window.alert(`${result.error.message}`);
      return;
    }
    console.log("Recieved data on frontend", result)
    // setActivePlayer(result.data?.active_player ?? null);
  }


  async function fetchGameState() {
    const params: GetGameStateRequest = {};
    const result: ResponseData<GetGameStateResponse> =
      await new LogicApiDataSource().getGameState(params);
    if (result?.error) {
      console.error('Error:', result.error);
      window.alert(`${result.error.message}`);
      return;
    }
    console.log("Recieved data on frontend", result)
    const players = result.data?.game_state.players;
    const potSize = result.data?.game_state.pot;

    setPotSize(potSize);

    const gamePhase = result.data?.game_state.phase;

    const winner = result.data?.game_state.winner;

    if (winner) {
      console.log("Winner is", winner)
      // window.alert(`Winner is ${winner}`);
    }


    //Setting community cards
    const communityCardsData = result.data?.game_state.community_cards;

    if (gamePhase == 'PreFlop') {
      // Set 0 community cards
      setCommunityCards([
        { rank: '', suit: '' },
        { rank: '', suit: '' },
        { rank: '', suit: '' },
        { rank: '', suit: '' },
        { rank: '', suit: '' },
      ])
    }
    else if (gamePhase == 'Flop') {
      // Set 3 community cards
      setCommunityCards([
        { rank: communityCardsData[0].rank, suit: communityCardsData[0].suit },
        { rank: communityCardsData[1].rank, suit: communityCardsData[1].suit },
        { rank: communityCardsData[2].rank, suit: communityCardsData[2].suit },
        { rank: '', suit: '' },
        { rank: '', suit: '' },
      ])
    }
    else if (gamePhase == 'Turn') {
      // Set 4 community cards
      setCommunityCards([
        { rank: communityCardsData[0].rank, suit: communityCardsData[0].suit },
        { rank: communityCardsData[1].rank, suit: communityCardsData[1].suit },
        { rank: communityCardsData[2].rank, suit: communityCardsData[2].suit },
        { rank: communityCardsData[3].rank, suit: communityCardsData[3].suit },
        { rank: '', suit: '' },
      ])
    }
    else if (gamePhase == 'River' || gamePhase == 'Showdown') {
      // Set 5 community cards
      setCommunityCards([
        { rank: communityCardsData[0].rank, suit: communityCardsData[0].suit },
        { rank: communityCardsData[1].rank, suit: communityCardsData[1].suit },
        { rank: communityCardsData[2].rank, suit: communityCardsData[2].suit },
        { rank: communityCardsData[3].rank, suit: communityCardsData[3].suit },
        { rank: communityCardsData[4].rank, suit: communityCardsData[4].suit },
      ])
    }


    // Setting the players data name and cards through this array replication
    const playersDatam = playersData;


    // Seeting name for all the players
    for (let i = 0; i < players.length; i++) {
      if (players[i].player_name){
        playersData[i].player_name = players[i].player_name;
      }
      if (players[i].round_move) {
        playersData[i].move = players[i].round_move;
      }
    }


    //Setting card depending upon the game phase
    if (gamePhase != 'Showdown' && players[thisplayerindex]) {
      // Set cards only for the current player
      if (players[thisplayerindex].cards.length > 0) {
      playersDatam[thisplayerindex].cards[0].rank = players[thisplayerindex].cards[0].rank;
      playersDatam[thisplayerindex].cards[0].suit = players[thisplayerindex].cards[0].suit;
      playersDatam[thisplayerindex].cards[1].rank = players[thisplayerindex].cards[1].rank;
      playersDatam[thisplayerindex].cards[1].suit = players[thisplayerindex].cards[1].suit;
      }
    }
    else {
      // Set cards for all players
      if (players.length > 0) {
      for (let i = 0; i < players.length; i++) {
        //Setting the cards if cards are distributed
        if (players[i].cards.length > 0) {
          playersDatam[i].cards[0].rank = players[i].cards[0].rank;
          playersDatam[i].cards[0].suit = players[i].cards[0].suit;
          playersDatam[i].cards[1].rank = players[i].cards[1].rank;
          playersDatam[i].cards[1].suit = players[i].cards[1].suit;
        }
      }
    }
    }
    
    // console.log("Players data", playersDatam)
    setPlayersData(playersDatam);
  }





  useEffect(() => {
      const intervalId = setInterval(() => {
        // fetchActivePlayer();
        fetchGameState();
      }, 5000);
      return () => clearInterval(intervalId);
    }, []);












  // Rendering functions ================================================

  const renderCard = (card: Card) => {
    let suit;
    let rank;

    if (card.rank == '' || card.suit == '') {
      return <div className={styles.emptyCard} />
    }

    if (card.rank == 'Ace') {
      rank = 'A'
    }
    else if (card.rank == 'King') {
      rank = 'K'
    }
    else if (card.rank == 'Queen') {
      rank = 'Q'
    }
    else if (card.rank == 'Jack') {
      rank = 'J'
    }
    else if (card.rank == 'Ten') {
      rank = '10'
    }
    else if (card.rank == 'Nine') {
      rank = '9'
    }
    else if (card.rank == 'Eight') {
      rank = '8'
    }
    else if (card.rank == 'Seven') {
      rank = '7'
    }
    else if (card.rank == 'Six') {
      rank = '6'
    }
    else if (card.rank == 'Five') {
      rank = '5'
    }
    else if (card.rank == 'Four') {
      rank = '4'
    }
    else if (card.rank == 'Three') {
      rank = '3'
    }
    else if (card.rank == 'Two') {
      rank = '2'
    }

    if (card.suit == 'Spades') {
      suit = '♠'
    }
    else if (card.suit == 'Clubs') {
      suit = '♣'
    }
    else if (card.suit == 'Diamonds') {
      suit = '♦'
    }
    else if (card.suit == 'Hearts') {
      suit = '♥'
    }

    return (
      <div className={styles.card}>
        <span className={styles.cardtext}>{rank}</span>
        <span className={styles.cardtext}>{suit}</span>
      </div>
    )
  }

  const renderPlayer = (player: PlayerData) => {
    // Should render two cards and player name
    const playerposition = player.id;
    const positionClass = "player" + playerposition;
    const cards = player.cards;
    const playername = (thisplayerindex == player.id - 1) ? player.name + " (You)" : player.name;
    const playermove = player.move;
    const foldclass = (playermove == 'Fold') ? styles.folded : '';
    return(
    <div
      key={player.id}
      className={`${styles.playerPosition} ${styles[`player${playerposition}`] } ${foldclass}`}
    > 
      <div className={styles.playerAction}>{playermove}</div>
      {/* <div className={styles.playerAction}>CALL</div> */}
      <div className={styles.playerCards}>
      {renderCard(cards[0])}
      {renderCard(cards[1])}
      </div>
      <div className={styles.playerName}>{playername}</div>
    </div>
    )
  }

  const renderCommunityCards = () => {
    return communityCards.map((card, index) => (
      <React.Fragment key={index}>
        {renderCard(card)}
      </React.Fragment>
    ))
  }

  return (
    <div className={styles.tableContainer}>
      <div className={styles.potSize}>Pot size: {potSize}</div>
      
      <div className={styles.table}>
        {playersData.map((player) => {
          return renderPlayer(player);
        })}

        <div className={styles.communityCards}>
          {renderCommunityCards()}
        </div>
      </div>
    </div>
  )
}