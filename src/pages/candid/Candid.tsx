import { useActor } from "@ic-reactor/react"
import { ActorMethodField } from "@ic-reactor/store"
import { useState } from "react"
import MethodForm from "@src/components/candid/MethodForm"
import DisplayData from "@src/components/DisplayData"
import { Box } from "@src/components/ui/box"

interface CandidProps {}

const Candid: React.FC<CandidProps> = () => {
  const { useMethodFields } = useActor<any>()

  const methodFields = useMethodFields()

  return (
    <Box className="grid gap-2">
      {methodFields.map(field => (
        <CandidField {...field} key={field.functionName} />
      ))}
    </Box>
  )
}

const CandidField: React.FC<ActorMethodField<any>> = ({
  functionName,
  ...fields
}) => {
  const { useQueryCall } = useActor()

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

export default Candid
