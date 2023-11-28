import { Principal } from "@dfinity/principal"
import { getModuleHash, objectToString } from "lib/utils"
import { useEffect, useMemo } from "react"
import { useSystemMethod } from "service/system"
import InstallWallet from "./InstallWallet"

interface WalletStatusProps {
  canisterId: Principal
}

const WalletStatus: React.FC<WalletStatusProps> = ({ canisterId }) => {
  const { call, data, error, loading } = useSystemMethod("user_canister_status")

  useEffect(() => {
    call(canisterId)
  }, [canisterId, call])

  const refreshHandler = async () => {
    const res = await call(canisterId)
    console.log(res)
  }

  const { hash, version } = useMemo(() => {
    if (data) {
      let hash = getModuleHash(data.canister_status)

      const version = data.version

      return { hash, version }
    }
    return { hash: undefined, version: undefined }
  }, [data])

  return (
    <div>
      <section>
        <h2>B3Wallet</h2>
        <button onClick={refreshHandler}>Refresh</button>
      </section>
      <section>
        <label>Response: &nbsp;</label>
        {loading ? <span>Loading...</span> : null}
        {error ? <span>Error: {JSON.stringify(error)}</span> : null}
        {data && <span>{objectToString(data)}</span>}
        {hash ? (
          <span>Hash: {hash}</span>
        ) : (
          <InstallWallet canisterId={canisterId} />
        )}
      </section>
    </div>
  )
}

export default WalletStatus
