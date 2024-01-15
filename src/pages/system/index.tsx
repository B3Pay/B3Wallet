import { useSystemMethodNames } from "@src/service/system"
import B3System from "./B3System"
import PageHeader from "@src/components/PageHeader"
import { Box } from "@src/components/ui/box"

function B3SystemPage() {
  const methodNames = useSystemMethodNames()

  return (
    <Box className="grid gap-2">
      <PageHeader title="B3System" />
      {methodNames.map(([type, functionName]) => (
        <B3System type={type} functionName={functionName} key={functionName} />
      ))}
    </Box>
  )
}

export default B3SystemPage
