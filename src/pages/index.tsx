import AppCanisterStatus from "components/AppCanisterStatus"
import AppCanisterVersion from "components/AppCanisterVersion"
import CreateApp from "components/CreateApp"
import CreateWallet from "components/CreateWallet"
import Footer from "components/Footer"
import UserStatus from "components/UserStatus"
import Head from "next/head"

function HomePage() {
  return (
    <div>
      <Head>
        <title>Internet Computer</title>
      </Head>
      <CreateApp />
      <CreateWallet />
      <UserStatus />
      <AppCanisterStatus />
      <AppCanisterVersion />
      <Footer />
    </div>
  )
}

export default HomePage
