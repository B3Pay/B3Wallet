import { MagnifyingGlassIcon } from "@radix-ui/react-icons"
import CreateWallet from "components/CreateWallet"
import Footer from "components/Footer"
import UserStatus from "components/UserStatus"
import WalletCanisterStatus from "components/WalletCanisterStatus"
import { MainNav } from "components/main-nav"
import TeamSwitcher from "components/team-switcher"
import { Icon } from "components/ui/icon"
import UserNav from "components/user-nav"
import Head from "next/head"

function HomePage() {
  return (
    <div>
      <Head>
        <title>Internet Computer</title>
      </Head>
      <div className="flex justify-center flex-col space-y-5">
        <div className="border-b">
          <div className="flex h-16 items-center px-4">
            <TeamSwitcher sharedClassName="w-screen sm:w-[300px] md:w-[200px]" />
            <MainNav className="hidden mx-6 md:flex" />
            <div className="flex ml-auto items-center space-x-2">
              <Icon
                size="xl"
                color="muted"
                asButton
                onClick={() => console.log("search")}
              >
                <MagnifyingGlassIcon />
              </Icon>
              <UserNav />
            </div>
          </div>
        </div>
        <CreateWallet />
        <UserStatus />
        <WalletCanisterStatus />
        <Footer />
      </div>
    </div>
  )
}

export default HomePage
