import { MagnifyingGlassIcon } from "@radix-ui/react-icons"

import { Icon } from "@src/components/ui/icon"

import ThemeToggle from "./theme-switcher"
import { MainNav } from "./main-nav"
import TeamSwitcher from "./team-switcher"
import UserNav from "./user-nav"

interface HeaderProps {}

const Header: React.FC<HeaderProps> = ({}) => {
  return (
    <header className="flex h-16 items-center px-4 space-x-2 bg-card">
      <TeamSwitcher sharedClassName="w-screen md:w-[200px]" />
      <div className="flex-1">
        <MainNav className="hidden mx-6 md:flex" />
      </div>
      <div className="flex ml-auto items-center space-x-2">
        <Icon
          variant="ghost"
          asButton
          noShadow
          asChild
          onClick={() => console.log("search")}
        >
          <MagnifyingGlassIcon />
        </Icon>
        <ThemeToggle />
        <UserNav />
      </div>
    </header>
  )
}

export default Header
