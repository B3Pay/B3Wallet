import { AuthClient } from "@dfinity/auth-client"
import { navigateTo } from "svelte-router-spa"
import { get, writable } from "svelte/store"
import { BackendActor, createActor } from "../lib/actor"
import { CryptoService } from "../lib/crypto"
import { showError } from "./notifications"

export type AuthState =
  | {
      state: "initializing-auth"
    }
  | {
      state: "anonymous"
      actor: BackendActor
      client: AuthClient
    }
  | {
      state: "initializing-crypto"
      actor: BackendActor
      client: AuthClient
    }
  | {
      state: "synchronizing"
      actor: BackendActor
      client: AuthClient
    }
  | {
      state: "initialized"
      actor: BackendActor
      client: AuthClient
      crypto: CryptoService
    }
  | {
      state: "error"
      error: string
    }

export const auth = writable<AuthState>({
  state: "initializing-auth"
})

async function initAuth() {
  const client = await AuthClient.create()
  if (await client.isAuthenticated()) {
    authenticate(client)
  } else {
    auth.update(() => ({
      state: "anonymous",
      actor: createActor(),
      client
    }))
  }
}

initAuth()

export function login() {
  const currentAuth = get(auth)

  if (currentAuth.state === "anonymous") {
    currentAuth.client.login({
      maxTimeToLive: BigInt(1800) * BigInt(1_000_000_000),
      identityProvider:
        process.env.DFX_NETWORK === "ic"
          ? "https://identity.ic0.app/#authorize"
          : "http://qhbym-qaaaa-aaaaa-aaafq-cai.localhost:8080/#authorize",
      onSuccess: () => authenticate(currentAuth.client)
    })
  }
}

export async function logout() {
  const currentAuth = get(auth)

  if (currentAuth.state === "initialized") {
    currentAuth.crypto.logout()
    await currentAuth.client.logout()
    auth.update(() => ({
      state: "anonymous",
      actor: createActor(),
      client: currentAuth.client
    }))
    navigateTo("/")
  }
}

export async function authenticate(client: AuthClient) {
  try {
    const actor = createActor({
      agentOptions: {
        identity: client.getIdentity()
      }
    })

    auth.update(() => ({
      state: "initializing-crypto",
      actor,
      client
    }))

    const cryptoService = new CryptoService(actor)
    await cryptoService.init().catch(e => {
      console.log(e)
      showError(e, "Could not initialize crypto service")
    })

    auth.update(() => ({
      state: "initialized",
      actor,
      client,
      crypto: cryptoService
    }))
  } catch (e) {
    auth.update(() => ({
      state: "error",
      error: e.message || "An error occurred"
    }))
  }
}
