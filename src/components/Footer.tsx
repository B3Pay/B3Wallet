"use client"
import Image from "next/image"
import { ModeToggle } from "./Theme"

interface FooterProps {}

const Footer: React.FC<FooterProps> = ({}) => {
  return (
    <footer>
      <ModeToggle />
      <div className="flex justify-center space-x-4">
        <a
          href="https://internetcomputer.org/"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Image
            width={140}
            height={30}
            src="/icp-logo.svg"
            alt="DFINITY logo"
          />
        </a>
      </div>
    </footer>
  )
}

export default Footer
