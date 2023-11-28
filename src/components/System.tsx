"use client"
import React, { useEffect } from "react"
import { useSystemMethod } from "service/system"
import CreateWallet from "./CreateWallet"
import WalletStatus from "./WalletStatus"

interface SystemProps {}

const System: React.FC<SystemProps> = ({}) => {
  const { call, data, error, loading } = useSystemMethod("get_canisters")

  useEffect(() => {
    call()
  }, [])

  return (
    <div>
      <section>
        <h2>B3System</h2>
      </section>
      <section>
        <label>Response: &nbsp;</label>
        {loading ? <span>Loading...</span> : null}
        {error ? <span>Error: {JSON.stringify(error)}</span> : null}
        {data ? (
          data.map(canisterId => (
            <WalletStatus key={canisterId.toText()} canisterId={canisterId} />
          ))
        ) : (
          <CreateWallet />
        )}
      </section>
    </div>
  )
}

export default System
