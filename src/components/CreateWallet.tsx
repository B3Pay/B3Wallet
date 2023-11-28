import { useSystemMethod } from "service/system"
import { Button } from "./ui/button"

interface CreateWalletProps {}

const CreateWallet: React.FC<CreateWalletProps> = ({}) => {
  const { call, data, error, loading } = useSystemMethod(
    "create_wallet_canister"
  )

  const createWalletHandler = async () => {
    const res = await call()
    console.log(res)
  }

  return (
    <div>
      <section>
        <h2>B3Wallet</h2>
        <Button onClick={createWalletHandler}>Create Wallet</Button>
      </section>
      <section>
        <label>Response: &nbsp;</label>
        {loading ? <span>Loading...</span> : null}
        {error ? <span>Error: {JSON.stringify(error)}</span> : null}
        {data && <span>{JSON.stringify(data)}</span>}
      </section>
    </div>
  )
}

export default CreateWallet
