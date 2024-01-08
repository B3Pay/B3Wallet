import Apps from "@src/components/Apps"
import { Box } from "@src/components/ui/box"
import { Card } from "@src/components/ui/card"
import { useSystemQuery } from "@src/service/system"

function HomePage() {
  return (
    <Box className="grid gap-2">
      <Card
        titleProps={{
          className:
            "text-2xl font-bold px-4 py-2 flex-1 flex items-center justify-center"
        }}
        title="ICP Apps"
      />
      <Apps />
    </Box>
  )
}

export default HomePage
