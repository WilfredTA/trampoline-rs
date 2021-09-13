import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import detectEthereumProvider from "@metamask/detect-provider";
import WalletDetails from './components/WalletDetails';
import PWCore, { Address, Amount, EthProvider, Provider } from '@lay2/pw-core';
import BasicCollector from './collectors/BasicCollector';
import { CHAIN_SPECS } from './helpers/constants';


interface PwObject {
  collector: BasicCollector,
  pwCore: PWCore,
  provider: Provider,
}

interface DataType {
  address: Address,
  capacity: Amount,
  sudtBalance: Amount,
}

interface TransactionTracker {
  txId: string,
  timestamp: number,
  updated: number,
  status: TransactionStatus
}

enum TransactionStatus {
  Pending,
  Confirmed,
  Failed
}

async function initPwCore() {
  const provider = new EthProvider();
  const collector = new BasicCollector("http://localhost:8116");
  const pwCore = await new PWCore("http://localhost:8114").init(provider, collector, undefined, CHAIN_SPECS.dev);
  return { pwCore, provider, collector };
}


function App() {

  const [busy, setBusy] = useState(true);
  const [loading, setLoading] = useState(true);
  const [data, setData] = useState<DataType | null>(null);
  const [transactions, setTransactions] = useState<TransactionTracker[]>([]);
  const [transactionStatusUpdateTime, setTransactionStatusUpdateTime] = useState(0);
  const [pw, setPw] = useState<PwObject | null>(null);

  useEffect(() => {
    detectEthereumProvider()
      .then(function (provider) {
        if (provider) {
          initPwCore()
            .then((pwValues) => {
              setBusy(true);
              setLoading(true);
              setPw(pwValues);
            });
        }
        else
          alert('A MetaMask compatible browser extension was not detected.\nThis tool will not function without one installed.');
      });
  }, [setPw]);


  // useEffect(() => {
  //   getConfig();
  // }, [])

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <h1>{{DAPP_NAME}}</h1>
        <p>
          This project was set up using Trampoline.
        </p>

        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
      <WalletDetails provider={pw?.provider} collector={pw?.collector} />

    </div>
  );
}

export default App;
