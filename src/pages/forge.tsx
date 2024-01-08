import { Box } from "@src/components/ui/box"
import AppList from "@src/components/AppList"

function HomePage() {
  return (
    <Box className="flex flex-col space-y-2">
      <AppList />
    </Box>
  )
}

export default HomePage
