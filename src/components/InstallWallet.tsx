import { Principal } from "@dfinity/principal"
import { objectToString } from "lib/utils"
import { useSystemMethod } from "service/system"

interface InstallWalletProps {
  canisterId: Principal
}

const InstallWallet: React.FC<InstallWalletProps> = ({ canisterId }) => {
  const { call, data, error, loading } = useSystemMethod(
    "install_wallet_canister"
  )

  const installWalletHandler = async () => {
    const res = await call(canisterId)
    console.log(res)
  }

  return (
    <div>
      <section>
        <h2>B3Wallet</h2>
        <button onClick={installWalletHandler}>Create Wallet</button>
      </section>
      <section>
        <label>Response: &nbsp;</label>
        {loading ? <span>Loading...</span> : null}
        {error ? <span>Error: {JSON.stringify(error)}</span> : null}
        {data && <span>{objectToString(data)}</span>}
      </section>
    </div>
  )
}

export default InstallWallet
