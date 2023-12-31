import B3System from "components/B3System"
import Footer from "components/Footer"
import Head from "next/head"

function HomePage() {
  return (
    <div>
      <Head>
        <title>B3Wallet</title>
      </Head>
      {/* <CreateApp />
        <CreateWallet />
        <UserStatus />
        <AppCanisterStatus />
        <AppCanisterVersion /> */}
      <B3System />
      <Footer />
    </div>
  )
}

export default HomePage
