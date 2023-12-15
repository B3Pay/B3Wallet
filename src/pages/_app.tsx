import { MagnifyingGlassIcon } from "@radix-ui/react-icons"
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
        <div className="flex justify-center flex-col space-y-5">
          <div className="border-b">
            <div className="flex h-16 items-center px-4">
              <TeamSwitcher sharedClassName="w-screen sm:w-[300px] md:w-[200px]" />
              <MainNav className="hidden mx-6 md:flex" />
              <div className="flex ml-auto items-center space-x-2">
                <Icon size="xl" asButton onClick={() => console.log("search")}>
                  <MagnifyingGlassIcon />
                </Icon>
                <UserNav />
              </div>
            </div>
          </div>
          <Component {...pageProps} />
        </div>
      </main>
    </ThemeProvider>
  )
}

export default App
