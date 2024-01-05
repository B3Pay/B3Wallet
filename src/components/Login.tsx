import { errorHandler } from "@src/lib/utils"
import { useSystemAuthClient } from "@src/service/system"
import Address from "./Address"
import { Button } from "./ui/button"

const Login = () => {
  const {
    login,
    logout,
    loginLoading,
    loginError,
    identity,
    authenticating,
    authenticated
  } = useSystemAuthClient()

  return (
    <div className="flex items-center">
      {loginError ? (
        <div className="text-red-500">{errorHandler(loginError)}</div>
      ) : loginLoading ? (
        <div className="text-blue-500">Loading...</div>
      ) : identity ? (
        <Address size="sm" address={identity?.getPrincipal().toText()} />
      ) : null}
      {authenticated ? (
        <Button onClick={() => logout()}>Logout</Button>
      ) : (
        <Button
          onClick={() =>
            login({
              identityProvider:
                process.env.DFX_NETWORK === "ic"
                  ? "https://identity.ic0.app/#authorize"
                  : `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943/#authorize`
            })
          }
          disabled={authenticating}
        >
          Login
        </Button>
      )}
    </div>
  )
}

export default Login
