import { MagnifyingGlassIcon } from "@radix-ui/react-icons"
import Footer from "components/Footer"
import ThemeToggle from "components/Theme"
import { MainNav } from "components/main-nav"
import TeamSwitcher from "components/team-switcher"
import { ThemeProvider } from "components/theme-provider"
import { Icon } from "components/ui/icon"
import UserNav from "components/user-nav"
import { AppProps } from "next/app"
import { Inter } from "next/font/google"

import "styles/globals.css"

const inter = Inter({ subsets: ["latin"], variable: "--font-inter" })

const App: React.FC<AppProps> = ({ Component, pageProps }) => {
  return (
    <ThemeProvider
      attribute="class"
      defaultTheme="system"
      enableSystem
      disableTransitionOnChange
    >
      <main className={`${inter.variable} font-sans`}>
        <div className="flex justify-center flex-col space-y-2">
          <div className="flex h-16 items-center px-4 space-x-2 bg-card">
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
          </div>
          <div className="max-w-2xl mx-auto w-full px-2">
            <Component {...pageProps} />
          </div>
          <Footer />
        </div>
      </main>
    </ThemeProvider>
  )
}

export default App
