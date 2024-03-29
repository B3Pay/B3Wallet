import { AppView } from "@src/declarations/b3system/b3system.did"
import { useSystemUpdate } from "@src/service/system"
import { Button } from "@src/components/ui/button"
import { Card, CardContent, CardFooter } from "./ui/card"
import { extractMetadata, nanoTimeToDate } from "@src/lib/converter"
import ImageFromBlob from "./ImageFromBlob"
import {
  GitHubLogoIcon,
  Link2Icon,
  ReloadIcon,
  StarFilledIcon,
  StarIcon
} from "@radix-ui/react-icons"
import { useRouter } from "next/router"
import { Box } from "./ui/box"
import ReleaseTable from "./ReleaseTable"
import { useEffect, useRef, useState } from "react"
import { cn } from "@src/lib/utils"
import Address from "./Address"
import { toast } from "sonner"
import { useFlipResize } from "@src/lib/hook/useResize"

interface AppProps extends AppView {
  refreshHandler?: () => void
  appRefresher: () => void
}

const App: React.FC<AppProps> = ({
  metadata,
  releases,
  app_id,
  description,
  install_count,
  created_by,
  created_at,
  updated_at,
  refreshHandler
}) => {
  const { push } = useRouter()
  const { call: createApp, loading } = useSystemUpdate({
    functionName: "create_app_canister",
    throwOnError: true,
    onSuccess: data => {
      if (data && "Ok" in data) {
        console.log("data", data.Ok.toText())
        refreshHandler?.()
        toast.success(`${name} installed on (${data.Ok.toText()})`)
      }
    }
  })

  const { repo, name, logo } = extractMetadata(metadata)

  const [selectedRelease, setSelectedRelease] = useState(releases[0])

  const [starred, setStarred] = useState(false)

  const { isFlipped, flip, containerRef, backRef, frontRef } = useFlipResize()

  return (
    <Box
      className="relative transition-transform duration-800 transform-gpu"
      ref={containerRef}
    >
      <Box
        roundSize="xl"
        className={cn(
          "absolute w-full top-0 shadow rounded-xl flex transform transition backface-hidden",
          isFlipped ? "rotate-y-180" : ""
        )}
        ref={frontRef}
      >
        <Card
          roundSide="l"
          padding="xl"
          bgGradient="success"
          className="flex items-center justify-center flex-col"
          noShadow
        >
          <ImageFromBlob
            imageData={logo}
            name={name}
            alt={name}
            width={76}
            height={76}
          />
          <p className="text-xs mt-4 text-center">
            {install_count.toString()} Installs
          </p>
        </Card>
        <Card
          noShadow
          className="flex-grow"
          roundSide="r"
          key={app_id}
          title={name}
          titleProps={{ padding: "sm", className: "text-xl font-bold" }}
          action={
            // make this buttons animate slightly to top when hovered
            <div className="relative">
              <Button
                asIconButton
                innerShadow
                roundSide="bl"
                variant="filled"
                color="warning"
                onClick={() => setStarred(!starred)}
              >
                {starred ? <StarFilledIcon /> : <StarIcon />}
              </Button>
              <Button
                asIconButton
                innerShadow
                roundSide="none"
                variant="filled"
                color="secondary"
                onClick={() => push(`${window.location.origin}/app/${app_id}`)}
              >
                <Link2Icon />
              </Button>
              <Button
                asIconButton
                innerShadow
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
          <CardContent className="flex flex-col space-y-1">
            {description}
            <div className="flex flex-col space-y-2 text-gray-500 text-xs pt-2">
              <Address
                iconSize="xs"
                address={created_by}
                prefix="Created by:"
              />
              <div>
                Created on: {nanoTimeToDate(created_at).toLocaleString()}
              </div>
              <div>
                Last updated on: {nanoTimeToDate(updated_at).toLocaleString()}
              </div>
            </div>
          </CardContent>
          <CardFooter>
            <Button fullWidth onClick={flip} roundSide="l" color="secondary">
              Select Release
            </Button>
            <Button
              fullWidth
              disabled={loading}
              onClick={() => {
                toast.promise(createApp([app_id]), {
                  loading: "Installing...",
                  success: "Installed!",
                  error: "Error!"
                })
              }}
              roundSide="r"
            >
              Install Latest
            </Button>
          </CardFooter>
        </Card>
      </Box>
      <Card
        noShadow
        ref={backRef}
        className={cn(
          "absolute w-full top-0 transform-gpu transition backface-hidden",
          isFlipped ? "rotate-y-0" : "rotate-y-180"
        )}
        key={app_id}
        title={`${name} Releases`}
        titleProps={{ padding: "sm", className: "text-xl font-bold" }}
        icon={
          <ImageFromBlob
            imageData={logo}
            name={name}
            alt={name}
            width={20}
            height={20}
          />
        }
        iconProps={{
          color: "muted"
        }}
        action={
          <Button
            innerShadow
            asIconButton
            color="info"
            variant="filled"
            diagonalRoundSide="r"
            onClick={refreshHandler}
          >
            <ReloadIcon />
          </Button>
        }
      >
        <CardContent>
          {releases?.length > 0 && (
            <ReleaseTable
              releases={releases}
              selectedRelease={selectedRelease}
              selectHandler={setSelectedRelease}
            />
          )}
        </CardContent>
        <CardFooter>
          <Button fullWidth roundSide="l" color="secondary" onClick={flip}>
            Back
          </Button>
          <Button
            fullWidth
            roundSide="r"
            disabled={loading}
            onClick={() => createApp([app_id])}
          >
            Install {selectedRelease.version}
          </Button>
        </CardFooter>
      </Card>
    </Box>
  )
}

export default App
