import AppCanisterStatus from "@src/components/AppCanisterStatus"
import AppCanisterVersion from "@src/components/AppCanisterVersion"
import CreateApp from "@src/components/CreateApp"
import CreateWallet from "@src/components/CreateWallet"
import PageHeader from "@src/components/PageHeader"
import UserStatus from "@src/components/UserStatus"

function HomePage() {
  return (
    <div>
      <PageHeader title="Home" />
      <CreateApp />
      <CreateWallet />
      <UserStatus />
      <AppCanisterStatus />
      <AppCanisterVersion />
    </div>
  )
}

export default HomePage
