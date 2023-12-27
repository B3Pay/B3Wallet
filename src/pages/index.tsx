import AppCanisterStatus from "components/AppCanisterStatus"
import AppCanisterVersion from "components/AppCanisterVersion"
import Candid from "components/Candid"
import CreateApp from "components/CreateApp"
import CreateWallet from "components/CreateWallet"
import Footer from "components/Footer"
import UserStatus from "components/UserStatus"
import Head from "next/head"

function HomePage() {
  return (
    <div>
      <Head>
        <title>B3Wallet</title>
      </Head>
      <div>
        <CreateApp />
        <CreateWallet />
        <UserStatus />
        <AppCanisterStatus />
        <AppCanisterVersion />
        <Candid />
      </div>
      <Footer />
    </div>
  )
}

export default HomePage
