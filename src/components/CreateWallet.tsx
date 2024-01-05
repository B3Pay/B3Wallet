import { useSystemUpdate } from "@src/service/system"
import DisplayData from "./DisplayData"
import { Button } from "./ui/button"

interface CreateWalletProps {}

const CreateWallet: React.FC<CreateWalletProps> = ({}) => {
  const { call, data, error, loading } = useSystemUpdate({
    functionName: "create_app_canister",
    args: ["b3wallet"]
  })

  return (
    <div>
      <section>
        <h2>B3Wallet</h2>
        <Button isLoading={loading} onClick={() => call()}>
          Create Wallet
        </Button>
      </section>
      <DisplayData loading={loading} error={error} data={data} />
    </div>
  )
}

export default CreateWallet
