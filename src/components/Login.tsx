import { useSystemAuthClient } from "service/system"
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
    <>
      <div>
        {loginLoading && <div>Loading...</div>}
        {loginError ? <div>{JSON.stringify(loginError)}</div> : null}
        {identity && <div>{identity.getPrincipal().toText()}</div>}
      </div>
      {authenticated ? (
        <div className="flex flex-col align-center">
          <Button onClick={() => logout()}>Logout</Button>
        </div>
      ) : (
        <div>
          <Button
            onClick={() =>
              login({
                identityProvider:
                  process.env.DFX_NETWORK === "ic"
                    ? "https://identity.ic0.app/#authorize"
                    : `http://localhost:4943?canisterId=rdmx6-jaaaa-aaaaa-aaadq-cai#authorize`
              })
            }
            disabled={authenticating}
          >
            Login
          </Button>
        </div>
      )}
    </>
  )
}

export default Login
