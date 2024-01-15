import { useActor } from "@ic-reactor/react"
import { useState } from "react"
import { CandidForm } from "@src/components/candid/"
import DisplayData from "@src/components/DisplayData"
import { Box } from "@src/components/ui/box"
import { ExtractedFunction } from "@ic-reactor/store"

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

const CandidField: React.FC<ExtractedFunction<any>> = ({
  functionName,
  ...fields
}) => {
  const { useQueryCall } = useActor()

  const [expanded, setExpanded] = useState(false)

  const { call, data, error, loading } = useQueryCall({
    functionName
  })

  return (
    <div className="bg-line-middle">
      <CandidForm
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
