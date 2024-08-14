import { useEffect, useRef, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { Chain, Subscription } from './zeus'
import { graphql } from './gql'

function App() {
  const [count, setCount] = useState(0)
  const [msg, setMsg] = useState("loading")

  const a = graphql('query add()')

    ; (async () => {

      const chain = Chain("http://127.0.0.1:8080/graphql")
      const res = await chain('query')({
        add: [{
          a: 0,
          b: 0,
        }, {
          name: true,
          related: {
            name: true
          }
        }]
      });

      setMsg(`fetch: ${JSON.stringify(res)}`)

      const sub = Subscription('http://127.0.0.1:8080/subscriptions');
      sub('subscription')({
        count: {
          name: true
        }

      }).on((res) => {
        console.log(res)
        // setMsg(msg + res.count)
      });

    })()

  return (
    <>
      <div>

        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <p>{msg}</p>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
