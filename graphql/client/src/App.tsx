import { useEffect, useRef, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { gql, InMemoryCache, OnSubscriptionDataOptions, TypedDocumentNode, useQuery, useSubscription } from '@apollo/client'

function SubscriptionTest() {
  const COMMENTS_SUBSCRIPTION:TypedDocumentNode = gql`
  subscription  {
    count {
      name
    }
  }
`;
  const { loading, data } = useSubscription(COMMENTS_SUBSCRIPTION)
  if (loading) return <p>subscription: loading</p>
  return (
    <p> subscription:  {JSON.stringify(data)}
    </p>
  )
}

function App() {
  const [count, setCount] = useState(0)



  const ADD_QUERY = gql`
  query {
    add(a: 1,b:2) {
      name,
      related {
        name
      }
    }
  }
  `



  const { loading, error, data } = useQuery(ADD_QUERY)
  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error : {error.message}</p>;
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
      <p>query: {JSON.stringify(data)}</p>
      <SubscriptionTest>

      </SubscriptionTest>
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
