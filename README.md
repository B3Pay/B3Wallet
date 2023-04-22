# Next.js Internet Computer Starter Template

This project provides a simple starter template for Dfinity Internet Computer using Next.js framework as frontend.

**The Most Recent Updates**

- NextJS 13.3
- DFX 0.13.1
- NodeJS 18.16

**Backend**

- A simple greeting hello world canister written in Rust

**Frontend**

- A simple React HTML form with name input, sending it to greet canister and showing the returned result

## Live Demo in IC Mainnet 🥳

https://u4gun-5aaaa-aaaah-qabma-cai.raw.ic0.app

![Screenshot](/public/demo-screenshot.png)

## Quick Start (Run locally)

Install:

- NodeJS 18.\* or higher https://nodejs.org/en/download/
- Internet Computer dfx CLI https://internetcomputer.org/docs/current/developer-docs/setup/install/

```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

Clone this Git repository:

```bash
git clone https://github.com/dappblock/nextjs-ic-starter
```

Open command terminal:
Enter the commands to start dfx local server in background:

```bash
cd nextjs-ic-starter
dfx start --background
```

Note: If you run it in MacOS, you may be asked to allow connections from dfx local server.

Enter the commands to install dependencies, deploy canister and run Next.js dev server:

```bash
npm install
dfx deploy
dfx generate
npm run dev
```

http://localhost:3000/

Cleanup - stop dfx server running in background:

```bash
dfx stop
```

## Project Structure

Internet Computer has the concept of [Canister](https://smartcontracts.org/docs/current/concepts/canisters-code/) which is a computation unit. This project has 3 canisters:

- hello (backend)
- hello_assets (frontend)

Canister configurations are stored in dfx.json.

### Backend

Backend code is inside /backend/ written in [Rust language](https://internetcomputer.org/docs/current/developer-docs/backend/rust/). Rust is a type-safe language with modern language features like async/await and actor build-in. It also has [Orthogonal persistence](https://internetcomputer.org/docs/current/Rust/main/Rust/#orthogonal-persistence) which I find very interesting.

### Frontend

Entry page code is inside /pages/index.js where the magic starts. With the DFX UI declarations generated code, frontend can use RPC style call to server side actor and its functions without worrying about HTTP request and response parsing.

To generate UI declarations:

```
dfx generate
```

We use a service locator pattern through actor-locator.js that will handle the dfx agent host using env var NEXT_PUBLIC_IC_HOST.

Creating hello actor:

```javascript
import { makeHelloActor } from "../ui/service/actor-adapter"
const hello = makeHelloActor()
```

Calling hello actor:

```javascript
const greeting = await hello.greet(name)
```

Webpack configuration:  
In Next.js, it's located in next.config.js.

## Backend dev

After marking changes in backend code e.g main.mo in /backend/service/hello, you can deploy it to the local DFX server using:

```bash
dfx deploy hello
```

**hello** is the backend canister name defined in dfx.json.

## Frontend dev - Next.js Static Code

Next.js developers are familiar with the handy hot code deployed in the Next.js dev environment when making changes in frontend code.

After deploying your backend code as shown above, you can run Next.js local dev server **npm run dev** and edit your frontend code with all the benefits of hot code deploy.

One thing to note is we use Next.js static code export here for hosting in Internet Computer so we can't use any features of Next.js that require server side NodeJS. Potentially, there might be ways to use Internet Computer canister as backend while deploying Next.js dapp to a hosting like Vercel that supports NodeJS server in the future. Further research is needed on that aspect. However, if you do want to run everything decentralized on blockchain including the frontend, you would want to deploy the exported static code to Internet Computer as well.

## Deploy and run frontend in local DFX server

In order to simulate the whole Internet Computer experience, you can deploy and run frontend code to local DFX server by running:

```bash
dfx start --background
npm run build
dfx deploy hello_assets
```

**hello_assets** is the frontend canister defined in dfx.json.

**npm run build** builds and export Next.js as static code storing in **/out** folder which would be picked up by **dfx deploy hello_assets** as defined in dfx.json with **/out** as the source.

When it completes, you can open Chrome and browse to:  
http://localhost:8000/?canisterId=[canisterId]

Replace [canisterId] with the hello_assets canister ID which you can find by running:

```bash
dfx canister id hello_assets
```

## Environment Configuration

There are three key configs following Next.js [Environment Variables](https://nextjs.org/docs/basic-features/environment-variables) configuration:

**.env.development** stores configs for use in local dev.

```
NEXT_PUBLIC_IC_HOST=http://localhost:8000
```

**.env.production** is used when building and exporting static code using **npm run build**

```
NEXT_PUBLIC_IC_HOST=http://localhost:8000
```

Notice both files are identical if we want the Next.js dapp to interact with the local dfx server.

Note **NEXT_PUBLIC** is the prefix used by Next.js to make env vars available to client side code through [build time inlining](https://nextjs.org/docs/basic-features/environment-variables).

**.env.icprod** is included for deployment to Internet Computer ic network which would be covered below.

## Deploy to IC Network Canister

The most exciting part is to deploy your Next.js / Internet Computer Dapp to production Internet Computer mainnet blockchain network.

To do that you will need:

- ICP tokens and convert it to [cycles](https://internetcomputer.org/docs/current/concepts/tokens-cycles/)
- Cycles wallet

Follow the [Network Deployment](https://internetcomputer.org/docs/current/developer-docs/setup/cycles/cycles-wallet/) guide to create a wallet.  
Dfinity offers [free cycle](https://faucet.dfinity.org/) to developers.

Now, you can deploy your Next.js Dapp to Internet Computer IC network by adding **--network ic** to the dfx subcommand. We will first update our env var to point to IC network host. Then deploy the backend canister first, export Next.js static code and deploy frontend canister **hello_assets**.

```bash
cp .env.icprod .env.production
dfx deploy --network ic
```

Open Chrome and go to https://[canisterId].raw.ic0.app/  
Replace [canisterId] by the hello_assets canister id in the IC network. You can find it by running:

```bash
dfx canister --network ic id hello_assets
```

Congratulations !! Well Done !! 👏 🚀 🎉

## Troubleshooting

Use Chrome Dev Tools / Console / Network. Check if the dapp uses the right canister id and hostname.

## Author

Henry Chan, henry@contentfly.app
Twitter: @kinwo

## Contributing

Please feel free to raise an issue or submit a pull request.

## License

MIT
