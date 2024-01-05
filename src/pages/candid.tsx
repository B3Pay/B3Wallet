import { ReActorProvider } from "@ic-reactor/react"
import { DownloadIcon, ExternalLinkIcon, GearIcon } from "@radix-ui/react-icons"
import FetchCandid from "components/FetchCandid"
import { Button } from "components/ui/button"
import { Card, CardContent } from "components/ui/card"
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "components/ui/form"
import { Input } from "components/ui/input"
import Head from "next/head"
import { useState } from "react"
import { useForm } from "react-hook-form"

function HomePage() {
  const [canisterId, setCanisterId] = useState("ss2fx-dyaaa-aaaar-qacoq-cai")

  const form = useForm({
    defaultValues: {
      canisterId
    }
  })

  return (
    <div>
      <Head>
        <title>Candid</title>
      </Head>
      <Card
        marginBottom="sm"
        icon={<ExternalLinkIcon />}
        iconProps={{
          color: "secondary",
          roundSide: "tl",
          diagonalRoundSide: "l"
        }}
        title="Fetch Candid"
      >
        <CardContent>
          <form
            onSubmit={form.handleSubmit(({ canisterId }) =>
              setCanisterId(canisterId)
            )}
          >
            <Form {...form}>
              <FormField
                control={form.control}
                name="canisterId"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Canister ID</FormLabel>
                    <div className="flex items-center">
                      <FormControl>
                        <Input
                          className="flex-grow"
                          placeholder="Canister ID"
                          roundSide="l"
                          closeHandler={() => form.setValue("canisterId", "")}
                          {...field}
                        />
                      </FormControl>
                      <FormControl>
                        <Button type="submit" roundSide="r">
                          Submit
                        </Button>
                      </FormControl>
                    </div>
                    <FormDescription>
                      Enter the Canister ID of the Candid file you want to
                      fetch.
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </Form>
          </form>
        </CardContent>
      </Card>
      <ReActorProvider host="https://ic0.app" canisterId={canisterId}>
        <FetchCandid />
      </ReActorProvider>
    </div>
  )
}

export default HomePage
