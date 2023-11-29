"use client"
import { ReloadIcon } from "@radix-ui/react-icons"
import { objectToString } from "lib/utils"
import { useEffect } from "react"
import { useSystemMethod } from "service/system"
import AddWallet from "./AddWallet"
import CreateWallet from "./CreateWallet"
import InstallWallet from "./InstallWallet"
import { Button } from "./ui/button"

interface UserStatusProps {}

const UserStatus: React.FC<UserStatusProps> = ({}) => {
  const { call, data, error, loading } = useSystemMethod("get_user_status")

  const refreshHandler = async () => {
    const res = await call()
    console.log(res)
  }

  useEffect(() => {
    refreshHandler()
  }, [])

  // const { hash, version } = useMemo(() => {
  //   if (data) {
  //     const hash = getModuleHash(data.canister_status)

  //     const version = data.version

  //     return { hash, version }
  //   }
  //   return { hash: undefined, version: undefined }
  // }, [data])

  return (
    <div>
      <section>
        <h2>Wallet Status</h2>
        <Button onClick={refreshHandler} asIconButton>
          <ReloadIcon />
        </Button>
      </section>
      <section>
        <label>Response: &nbsp;</label>
        {loading ? <span>Loading...</span> : null}
        {error ? <span>Error: {JSON.stringify(error)}</span> : null}
        {data && <span>{objectToString(data)}</span>}
        {/* {hash ? (
          <span>Hash: {hash}</span>
        ) : (
          <InstallWallet canisterId={"ajuq4-ruaaa-aaaaa-qaaga-cai"} />
        )} */}
      </section>
      <InstallWallet />
      <AddWallet />
      <CreateWallet />
    </div>
  )
}

export default UserStatus
