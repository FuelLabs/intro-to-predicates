import { useState } from 'react'
import { useFuel, useIsConnected, useAccount, useWallet } from '@fuel-wallet/react';
import { Address } from 'fuels';
import './App.css'
import Create from './components/create';
const CONTRACT_ID = "0x1f5f2d8b03c1f8d111fe1b3790bacd78255e91d30026f0bbc9f588c9bb6a056b"


function App() {
  const [active, setActive] = useState<'all-items' | 'list-item'>('all-items');
  const { isConnected } = useIsConnected();
  const { fuel } = useFuel();
  const { account } = useAccount();
  const { wallet } = useWallet();

  return (
    <>
    <div className="App">
      {/* <header>
        <h1>Predicate Multisig</h1>
      </header> */}
      {/* <nav>
        <ul>
          <li 
          className={active === 'all-items' ? "active-tab" : ""} 
          onClick={() => setActive('all-items')}
          >
            See All Items
          </li>
          <li 
          className={active === 'list-item' ? "active-tab" : ""} 
          onClick={() => setActive('list-item')}
          >
            List an Item
          </li>
        </ul>
      </nav> */}
      {fuel ? (
        <div>
          { isConnected ? (
            <div>
              <Create/>
            </div>
          ) : (
            <div>
              <button onClick={() => fuel?.connect()}>
                Connect Wallet
              </button>
          </div>
          )}
        </div>
      ) : (
        <div>
          Download the{" "}
          <a
            target="_blank"
            rel="noopener noreferrer"
            href="https://wallet.fuel.network/"
          >
            Fuel Wallet
          </a>{" "}
          to use the app.
        </div>
      )}
    </div>
    </>
  )
}

export default App
