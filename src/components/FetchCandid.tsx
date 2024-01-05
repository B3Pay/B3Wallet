import { useReActor } from "@ic-reactor/react"
import { ReActorMethodField } from "@ic-reactor/store"
import { useState } from "react"
import MethodForm from "./candid/MethodForm"
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
  functionName,
  ...fields
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
        {...fields}
        expanded={expanded}
        actorCallHandler={call}
        functionName={functionName}
        onExpand={() => setExpanded(prev => !prev)}
      />
      {expanded && <DisplayData loading={loading} error={error} data={data} />}
    </div>
  )
}

export default FetchCandid
