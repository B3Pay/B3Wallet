import { useSystemAuthClient } from "@src/service/system"
import { Button } from "./ui/button"
import { Card, CardFooter } from "./ui/card"
import { toast } from "sonner"

const Login = () => {
  const { login, logout, loginLoading, authenticated } = useSystemAuthClient({
    onAuthentication: promise =>
      toast.promise(promise, {
        loading: "Loading...",
        success: "Connected to the Internet Computer.",
        error: e => e.message
      }),
    onLogin: promise => {
      toast.promise(promise, {
        loading: "Logging in...",
        success: principal => `Logged in as ${principal.toText()}`,
        error: e => e.message
      })
    }
  })

  const loginHandler = async () => {
    login({
      identityProvider:
        process.env.DFX_NETWORK === "ic"
          ? "https://identity.ic0.app/#authorize"
          : `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943/#authorize`
    })
  }

  return (
    <Card
      titleProps={{
        className:
          "text-2xl font-bold px-4 py-2 flex-1 flex items-center justify-center"
      }}
    >
      {" "}
      {!authenticated ? (
        <CardFooter className="flex flex-col space-y-2 items-center pt-4">
          <p className="text-sm text-center">
            Please login with your ICP wallet to view your installed apps.
          </p>
          <Button onClick={loginHandler} disabled={loginLoading} fullWidth>
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
