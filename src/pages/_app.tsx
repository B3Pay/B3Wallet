import { AppProps } from "next/app"
import React from "react"
import { SystemProvider } from "service/system"
import "styles/global.css"

const App: React.FC<AppProps> = ({ Component, pageProps }) => {
  return (
    <SystemProvider>
      <Component {...pageProps} />
    </SystemProvider>
  )
}

export default App
