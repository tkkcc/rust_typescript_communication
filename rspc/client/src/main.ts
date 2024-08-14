import './style.css'
import typescriptLogo from './typescript.svg'
import viteLogo from '/vite.svg'
import { setupCounter } from './counter.ts'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <a href="https://vitejs.dev" target="_blank">
      <img src="${viteLogo}" class="logo" alt="Vite logo" />
    </a>
    <a href="https://www.typescriptlang.org/" target="_blank">
      <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <h1>Vite + TypeScript</h1>
    <p id="message"></p>
    <p id="stream"></p>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
    <p class="read-the-docs">
      Click on the Vite and TypeScript logos to learn more
    </p>
  </div>
`

setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)


import { createClient, WebsocketTransport } from "@rspc/client";
import { Procedures, User } from './bindings.ts'

// You must provide the generated types as a generic and create a transport (in this example we are using HTTP Fetch) so that the client knows how to communicate with your API.
const client = createClient<Procedures>({
  // Refer to the integration your using for the correct transport.
  transport: new WebsocketTransport("ws://localhost:3000/rspc/ws"),
});

async function ff() {
  let node = document.querySelector<HTMLButtonElement>('#message')!;
  // Now use the client in your code!
  let version = await client.query(["version"]); // The types will be inferred from your backend.

  // The types will be inferred from your backend.
  let newUser = await client.mutation(["toggle", {
    mobile: '0',
    more: 0,
    what: [false, true]
  }]);

  node.innerText = "query: " + JSON.stringify(newUser) + ` version: ${version}`

  // stream
  client.addSubscription(["listen_new_user", {
    mobile: '',
    more: 0,
    what: null
  }], {
    onData: (data: User) => {
      let node = document.querySelector<HTMLButtonElement>('#stream')!;
      node.innerText = "subscription: " + JSON.stringify(data)
    }
  });
}
ff()
