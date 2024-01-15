import { useSystemMethodCall, B3System } from "@src/service/system"
import { CandidForm } from "@src/components/candid"
import DisplayData from "@src/components/DisplayData"
import { useState } from "react"
import { ServiceMethodType } from "@ic-reactor/store"

interface B3SystemProps {
  functionName: keyof B3System
  type: ServiceMethodType
}

const B3System: React.FC<B3SystemProps> = ({ functionName, type }) => {
  const [expanded, setExpanded] = useState(false)

  const { call, data, error, loading, field } = useSystemMethodCall({
    type,
    functionName
  })

  return (
    <div className="bg-line-middle">
      <CandidForm
        {...field}
        expanded={expanded}
        actorCallHandler={call}
        onExpand={() => setExpanded(prev => !prev)}
      />
      {expanded && <DisplayData loading={loading} error={error} data={data} />}
    </div>
  )
}

export default B3System
