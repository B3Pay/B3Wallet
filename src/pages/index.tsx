import AppCanisterStatus from "components/AppCanisterStatus"
import AppCanisterVersion from "components/AppCanisterVersion"
import CreateApp from "components/CreateApp"
import CreateWallet from "components/CreateWallet"
import UserStatus from "components/UserStatus"
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
