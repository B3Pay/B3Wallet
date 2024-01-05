import AppCanisterStatus from "@src/components/AppCanisterStatus"
import AppCanisterVersion from "@src/components/AppCanisterVersion"
import CreateApp from "@src/components/CreateApp"
import CreateWallet from "@src/components/CreateWallet"
import UserStatus from "@src/components/UserStatus"
import Head from "next/head"

function HomePage() {
  return (
    <div>
      <Head>
        <title>B3Wallet</title>
      </Head>
      <CreateApp />
      <CreateWallet />
      <UserStatus />
      <AppCanisterStatus />
      <AppCanisterVersion />
    </div>
  )
}

export default HomePage
