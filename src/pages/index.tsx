import CreateWallet from "components/CreateWallet"
import Footer from "components/Footer"
import Login from "components/Login"
import UserStatus from "components/UserStatus"
import WalletCanisterStatus from "components/WalletCanisterStatus"
import Head from "next/head"

function HomePage() {
  return (
    <div>
      <Head>
        <title>Internet Computer</title>
      </Head>
      <div className="flex justify-center flex-col space-y-5">
        <h1 className="text-4xl font-bold text-center">
          Welcome to the Internet Computer starter template
        </h1>
        <Login />
        <CreateWallet />
        <UserStatus />
        <WalletCanisterStatus />
        <Footer />
      </div>
    </div>
  )
}

export default HomePage
