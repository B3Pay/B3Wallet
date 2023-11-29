import { ThemeProvider } from "components/theme-provider"
import { AppProps } from "next/app"
import dynamic from "next/dynamic"
import { Inter } from "next/font/google"
import "styles/globals.css"

const inter = Inter({ subsets: ["latin"], variable: "--font-inter" })

const SystemProvider = dynamic(() => import("service/system"), {
  ssr: false
})

const App: React.FC<AppProps> = ({ Component, pageProps }) => {
  return (
    <SystemProvider>
      <ThemeProvider
        attribute="class"
        defaultTheme="system"
        enableSystem
        disableTransitionOnChange
      >
        <main className={`${inter.variable} font-sans`}>
          <Component {...pageProps} />
        </main>
      </ThemeProvider>
    </SystemProvider>
  )
}

export default App
