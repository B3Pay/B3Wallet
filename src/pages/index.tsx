import CreateWallet from "components/CreateWallet"
import Footer from "components/Footer"
import UserStatus from "components/UserStatus"
import WalletCanisterStatus from "components/WalletCanisterStatus"
import Head from "next/head"

function HomePage() {
  return (
    <div>
      <Head>
        <title>Internet Computer</title>
      </Head>
      <CreateWallet />
      <UserStatus />
      <WalletCanisterStatus />
      <Footer />
    </div>
  )
}

export default HomePage
