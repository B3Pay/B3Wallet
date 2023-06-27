import { AuthClient } from "@dfinity/auth-client"
import { useCallback, useEffect, useMemo, useState } from "react"
import { B3System, createB3SystemActor, createManagmentActor } from "service"
import { IDENTITY_CANISTER_ID, IS_LOCAL } from "../helpers/config"

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
      const identityProvider = IS_LOCAL
        ? `http://${IDENTITY_CANISTER_ID}.localhost:8080`
        : "https://identity.ic0.app/#authorize"

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
    const actor = createB3SystemActor(authClient.getIdentity())

    setSystemActor(actor)
  }, [authClient])

  const logout = () => {
    setIsAuthenticated(false)
    setSystemActor(undefined)

    authClient?.logout({ returnTo: "/" })
  }

  const getManagmentActor = useCallback(() => {
    if (!authClient) return

    const management = createManagmentActor(authClient.getIdentity())

    return management
  }, [authClient])

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

  const principal = useMemo(() => {
    if (authClient == null) return null

    return authClient.getIdentity().getPrincipal().toString()
  }, [authClient])

  return {
    authClient,
    isAuthenticated,
    isAuthenticating,
    login,
    logout,
    principal,
    systemActor,
    getManagmentActor
  }
}

export default useAuth
