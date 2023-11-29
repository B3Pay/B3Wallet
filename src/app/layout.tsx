import { ThemeProvider } from "components/theme-provider"
import dynamic from "next/dynamic"
import { Inter } from "next/font/google"
import "./globals.css"

const inter = Inter({ subsets: ["latin"] })

const SystemProvider = dynamic(() => import("service/system"), {
  ssr: false
})

export default function RootLayout({
  children
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <SystemProvider>
          <ThemeProvider
            attribute="class"
            defaultTheme="system"
            enableSystem
            disableTransitionOnChange
          >
            {children}
          </ThemeProvider>
        </SystemProvider>
      </body>
    </html>
  )
}
