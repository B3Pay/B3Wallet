import Footer from "@src/layout/Footer"
import Header from "@src/layout/Header"
import { ThemeProvider } from "@src/styles/provider"
import { AppProps } from "next/app"
import { Inter } from "next/font/google"

import "@src/styles/globals.css"

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
          <Header />
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
