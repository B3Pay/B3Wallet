import { AppView } from "@src/declarations/b3system/b3system.did"
import { useSystemUpdate } from "@src/service/system"
import { Button } from "@src/components/ui/button"
import { Card, CardContent, CardFooter } from "./ui/card"
import { extractMetadata } from "@src/lib/metadata"
import ImageFromBlob from "./ImageFromBlob"
import { GitHubLogoIcon, Link2Icon } from "@radix-ui/react-icons"
import { useRouter } from "next/router"
import { Box } from "./ui/box"

interface AppProps extends AppView {}

const App: React.FC<AppProps> = ({ metadata, app_id, description }) => {
  const { push } = useRouter()
  const { call: createApp } = useSystemUpdate({
    functionName: "create_app_canister"
  })

  const { repo, name, logo } = extractMetadata(metadata)

  return (
    <Box className="flex">
      <Card roundSide="l" padding="xl" bgGradient="info">
        <ImageFromBlob
          imageData={logo}
          name={name}
          alt={name}
          width={96}
          height={96}
        />
      </Card>
      <Card
        className="flex-grow"
        roundSide="r"
        key={app_id}
        title={name}
        action={
          <div>
            <Button
              asIconButton
              roundSide="bl"
              variant="filled"
              color="secondary"
              onClick={() => push(`${window.location.origin}/app/${app_id}`)}
            >
              <Link2Icon />
            </Button>
            <Button
              asIconButton
              color="info"
              variant="filled"
              roundSide="tr"
              onClick={() => push(repo)}
            >
              <GitHubLogoIcon />
            </Button>
          </div>
        }
      >
        <CardContent>{description}</CardContent>
        <CardFooter>
          <Button fullWidth onClick={() => createApp([app_id])}>
            Create
          </Button>
        </CardFooter>
      </Card>
    </Box>
  )
}

export default App
