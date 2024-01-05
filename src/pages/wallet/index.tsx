import { useWalletMethodFields } from "@src/service/wallet"
import B3Wallet from "./B3Wallet"
import PageHeader from "@src/components/PageHeader"
import { Box } from "@src/components/ui/box"

function B3WalletPage() {
  const methodFields = useWalletMethodFields()

  return (
    <div>
      <PageHeader title="B3Wallet" />
      <Box className="grid gap-2">
        {methodFields.map((field, index) => (
          <B3Wallet {...field} key={index} />
        ))}
      </Box>
    </div>
  )
}

export default B3WalletPage
