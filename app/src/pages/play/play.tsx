import React from 'react'
import TopNavbar from '../../components/TopNavbar'
import PokerTable from '../../components/PokerTable'
import ActionButtons from '../../components/ActionButtons'
import styles from '../../styles/page.module.css'

export default function Play() {
  return (
    <div className={styles.container}>
      <TopNavbar />
      <PokerTable />
      <ActionButtons />
    </div>
  )
}