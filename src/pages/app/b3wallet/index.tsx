import { useWalletMethodFields } from "@src/service/wallet"
import B3Wallet from "./B3Wallet"
import PageHeader from "@src/components/PageHeader"
import { Box } from "@src/components/ui/box"

function B3WalletPage() {
  const methodFields = useWalletMethodFields()

  return (
    <Box className="grid gap-2">
      <PageHeader title="B3Wallet" />
      {methodFields.map((field, index) => (
        <B3Wallet {...field} key={index} />
      ))}
    </Box>
  )
}

export default B3WalletPage
