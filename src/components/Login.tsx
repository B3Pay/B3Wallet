import { errorHandler } from "@src/lib/utils"
import { useSystemAuthClient } from "@src/service/system"
import { Button } from "./ui/button"
import { Card, CardFooter } from "./ui/card"

const Login = () => {
  const { login, logout, loginError, loginLoading, authenticated } =
    useSystemAuthClient()

  return (
    <Card
      titleProps={{
        className:
          "text-2xl font-bold px-4 py-2 flex-1 flex items-center justify-center"
      }}
    >
      {loginError ? (
        <div className="text-red-500">{errorHandler(loginError)}</div>
      ) : null}
      {!authenticated ? (
        <CardFooter className="flex flex-col space-y-2 items-center pt-4">
          <p className="text-sm text-center">
            Please login with your ICP wallet to view your installed apps.
          </p>
          <Button
            onClick={() =>
              login({
                identityProvider:
                  process.env.DFX_NETWORK === "ic"
                    ? "https://identity.ic0.app/#authorize"
                    : `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943/#authorize`
              })
            }
            disabled={loginLoading}
            fullWidth
          >
            {loginLoading ? "Loading..." : "Login"}
          </Button>
        </CardFooter>
      ) : (
        <Button fullWidth onClick={() => logout()}>
          Logout
        </Button>
      )}
    </Card>
  )
}

export default Login
