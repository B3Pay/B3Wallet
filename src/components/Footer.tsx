import Image from "next/image"

interface FooterProps {}

const Footer: React.FC<FooterProps> = ({}) => {
  return (
    <footer>
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
