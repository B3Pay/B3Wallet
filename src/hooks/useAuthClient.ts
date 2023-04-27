import { AuthClient } from "@dfinity/auth-client"
import { IC_URL, IDENTITY_CANISTER_ID } from "helpers/config"
import { useCallback, useEffect, useState } from "react"
import { B3User, makeB3UserActor } from "service/actor"

const useAuth = () => {
  const [authClient, setAuthClient] = useState<AuthClient>()
  const [actor, setActor] = useState<B3User>()
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false)

  const login = useCallback(async () => {
    const alreadyAuthenticated = await authClient?.isAuthenticated()

    if (alreadyAuthenticated) {
      setIsAuthenticated(true)
    } else {
      // TODO: make it work with different environments
      const identityProvider =
        process.env.DFX_NETWORK === "ic"
          ? `https://identity.ic0.app/#authorize`
          : `${IC_URL}?canisterId=${IDENTITY_CANISTER_ID}`

      const maxTimeToLive = 700n * 24n * 60n * 60n * 1000n * 1000n * 1000n

      authClient?.login({
        identityProvider,
        maxTimeToLive,
        onSuccess: () => {
          setIsAuthenticated(true)
        }
      })
    }
  }, [authClient])

  const initActor = useCallback(() => {
    const actor = makeB3UserActor(authClient?.getIdentity())

    setActor(actor)
  }, [authClient])

  const logout = () => {
    setIsAuthenticated(false)
    setActor(undefined)
    authClient?.logout({ returnTo: "/" })
  }

  useEffect(() => {
    if (authClient == null) {
      AuthClient.create().then(async client => {
        setAuthClient(client)
      })
    }
  }, [authClient])

  useEffect(() => {
    if (authClient != null) {
      ;(async () => {
        const authenticated = await authClient?.isAuthenticated()
        if (authenticated) {
          setIsAuthenticated(true)
        }
      })()
    }
  }, [authClient])

  useEffect(() => {
    if (isAuthenticated) initActor()
  }, [isAuthenticated, initActor])

  return {
    authClient,
    setAuthClient,
    isAuthenticated,
    setIsAuthenticated,
    login,
    logout,
    actor
  }
}

export default useAuth
