import { ActorProvider } from "@ic-reactor/react"
import { ExternalLinkIcon } from "@radix-ui/react-icons"
import Candid from "./Candid"
import { Button } from "@src/components/ui/button"
import { Card, CardContent } from "@src/components/ui/card"
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "@src/components/ui/form"
import { Input } from "@src/components/ui/input"
import { useEffect, useState } from "react"
import { useForm } from "react-hook-form"
import PageHeader from "@src/components/PageHeader"
import { Box } from "@src/components/ui/box"
import { useRouter } from "next/router"

function CandidPage() {
  const { query } = useRouter()

  const form = useForm({
    defaultValues: {
      canisterId: ""
    }
  })

  console.log("query", query)
  const [defaultValues, setDefaultValues] = useState({
    canisterId: undefined as string | undefined
  })

  useEffect(() => {
    if (query.canisterId) {
      setDefaultValues({
        canisterId: query.canisterId as string
      })
      form.setValue("canisterId", query.canisterId as string)
    }
  }, [query])

  return (
    <Box className="grid gap-2">
      <PageHeader title="Candid" />
      <Card
        icon={<ExternalLinkIcon />}
        iconProps={{
          color: "secondary",
          roundSide: "tl",
          diagonalRoundSide: "l"
        }}
        title="Fetch Candid"
      >
        <CardContent>
          <form onSubmit={form.handleSubmit(setDefaultValues)}>
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
      {defaultValues.canisterId && (
        <ActorProvider
          canisterId={defaultValues.canisterId}
          loadingComponent={
            <div className="flex flex-col items-center justify-center h-80">
              <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-primary" />
              <div>Loading...</div>
            </div>
          }
        >
          <Candid />
        </ActorProvider>
      )}
    </Box>
  )
}

export default CandidPage
