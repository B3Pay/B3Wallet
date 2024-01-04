import { useReActor } from "@ic-reactor/react"
import { ReActorMethodField } from "@ic-reactor/store"
import { useState } from "react"
import MethodForm from "./candid/MethodForm"
import { Card, CardContent } from "./ui/card"
import { GlobeIcon } from "@radix-ui/react-icons"
import DisplayData from "./DisplayData"
import { Box } from "./ui/box"

interface FetchCandidProps {}

const FetchCandid: React.FC<FetchCandidProps> = () => {
  const { useActorStore } = useReActor()

  const { methodFields } = useActorStore()

  return (
    <Box className="grid gap-2">
      {methodFields.map(field => (
        <CandidField {...field} key={field.functionName} />
      ))}
    </Box>
  )
}

const CandidField: React.FC<ReActorMethodField<any>> = ({
  fields,
  functionName,
  defaultValues
}) => {
  const { useQueryCall } = useReActor()

  const [expanded, setExpanded] = useState(false)

  const { call, data, error, loading } = useQueryCall({
    functionName,
    disableInitialCall: true
  })

  return (
    <div className="bg-line-middle">
      <MethodForm
        fields={fields}
        expanded={expanded}
        actorCallHandler={call}
        functionName={functionName}
        defaultValues={defaultValues}
        onExpand={() => setExpanded(prev => !prev)}
      />
      {expanded && (error || data || loading) ? (
        <Card
          marginTop="sm"
          icon={<GlobeIcon />}
          iconProps={{
            color: loading ? "warning" : error ? "error" : "success",
            roundSide: "tl",
            diagonalRoundSide: "l"
          }}
          title={`${functionName.toTitleCase()} ${
            loading ? "Loading..." : error ? "Error!" : "Success"
          }`}
        >
          <CardContent>
            <DisplayData loading={loading} error={error} data={data} />
          </CardContent>
        </Card>
      ) : null}
    </div>
  )
}

export default FetchCandid
