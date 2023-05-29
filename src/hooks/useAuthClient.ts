import { AuthClient } from "@dfinity/auth-client"
import { IC_URL, IDENTITY_CANISTER_ID, IS_LOCAL } from "helpers/config"
import { useCallback, useEffect, useState } from "react"
import { B3System, makeB3SystemActor } from "service/actor"

const useAuth = () => {
  const [isAuthenticating, setIsAuthenticating] = useState<boolean>(false)
  const [authClient, setAuthClient] = useState<AuthClient>()
  const [systemActor, setSystemActor] = useState<B3System>()
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false)

  const login = useCallback(async () => {
    const alreadyAuthenticated = await authClient?.isAuthenticated()

    if (alreadyAuthenticated) {
      setIsAuthenticated(true)
    } else {
      // TODO: make it work with different environments
      const identityProvider = IS_LOCAL
        ? `${IC_URL}?canisterId=${IDENTITY_CANISTER_ID}`
        : `https://identity.ic0.app/#authorize`

      const maxTimeToLive = 24n * 60n * 60n * 1000n * 1000n * 1000n

      setIsAuthenticating(true)

      authClient?.login({
        identityProvider,
        maxTimeToLive,
        onSuccess: () => {
          setIsAuthenticating(false)
          setIsAuthenticated(true)
        }
      })
    }
  }, [authClient])

  const initActor = useCallback(() => {
    if (!authClient) return
    const actor = makeB3SystemActor(authClient.getIdentity())

    setSystemActor(actor)
  }, [authClient])

  const logout = () => {
    setIsAuthenticated(false)
    setSystemActor(undefined)
    authClient?.logout({ returnTo: "/" })
  }

  useEffect(() => {
    if (authClient == null) {
      setIsAuthenticating(true)
      AuthClient.create().then(async client => {
        await client?.isAuthenticated()
        setIsAuthenticating(false)
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
    isAuthenticated,
    isAuthenticating,
    login,
    logout,
    systemActor
  }
}

export default useAuth
