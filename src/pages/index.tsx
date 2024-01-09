import Login from "@src/components/Login"
import { Box } from "@src/components/ui/box"
import { useSystemAuthClient } from "@src/service/system"
import Account from "@src/components/Account"
import FormEditor from "@src/components/FormEditor"

function HomePage() {
  const { identity } = useSystemAuthClient()

  const isAnonymous = identity ? identity.getPrincipal().isAnonymous() : true

  return (
    <Box className="flex flex-col space-y-2">
      {/* {!isAnonymous && <Account />}
      <Login /> */}
      <FormEditor />
    </Box>
  )
}

export default HomePage
