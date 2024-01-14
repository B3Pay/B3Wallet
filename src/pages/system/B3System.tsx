import { SystemDynamicField, useSystemQuery } from "@src/service/system"
import MethodForm from "@src/components/candid/MethodForm"
import DisplayData from "@src/components/DisplayData"
import { useState } from "react"

interface B3SystemProps extends SystemDynamicField {}

const B3System: React.FC<B3SystemProps> = ({ functionName, ...fields }) => {
  const [expanded, setExpanded] = useState(false)

  const { call, data, error, loading } = useSystemQuery({
    functionName,
    refetchOnMount: true
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

export default B3System
