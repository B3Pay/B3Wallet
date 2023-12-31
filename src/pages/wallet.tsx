import B3Wallet from "components/B3Wallet"
import Footer from "components/Footer"
import Head from "next/head"

function HomePage() {
  return (
    <div>
      <Head>
        <title>B3Wallet</title>
      </Head>
      <div>
        <B3Wallet />
      </div>
      <Footer />
    </div>
  )
}

export default HomePage
