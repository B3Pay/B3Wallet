import { ReActorProvider } from "@ic-reactor/react"
import { ExternalLinkIcon } from "@radix-ui/react-icons"
import FetchCandid from "./FetchCandid"
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
import { useState } from "react"
import { useForm } from "react-hook-form"
import PageHeader from "@src/components/PageHeader"

function CandidPage() {
  const [defaultValues, setDefaultValues] = useState({
    canisterId: "ss2fx-dyaaa-aaaar-qacoq-cai"
  })

  const form = useForm({
    defaultValues
  })

  return (
    <div>
      <PageHeader title="Candid" />
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
      <ReActorProvider
        host="https://ic0.app"
        canisterId={defaultValues.canisterId}
      >
        <FetchCandid />
      </ReActorProvider>
    </div>
  )
}

export default CandidPage
