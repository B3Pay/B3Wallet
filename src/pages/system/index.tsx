import { useSystemMethodFields } from "@src/service/system"
import B3System from "./B3System"
import PageHeader from "@src/components/PageHeader"
import { Box } from "@src/components/ui/box"

function B3SystemPage() {
  const methodFields = useSystemMethodFields()

  return (
    <div>
      <PageHeader title="B3System" />
      <Box className="grid gap-2">
        {methodFields.map((field, index) => (
          <B3System {...field} key={index} />
        ))}
      </Box>
    </div>
  )
}

export default B3SystemPage
