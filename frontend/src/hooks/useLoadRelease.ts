import { useCallback, useState } from "react"
import { B3BasicWallet, B3Wallet } from "service/actor"

const chunkGenerator = async function* (
  wasmModule: number[],
  chunkSize = 700000
) {
  for (let start = 0; start < wasmModule.length; start += chunkSize) {
    yield wasmModule.slice(start, start + chunkSize)
  }
}

const useLoadRelease = (actor: B3Wallet | B3BasicWallet) => {
  const [progress, setProgress] = useState<number>(0)
  const [wasmLoading, setWasmLoading] = useState<boolean>(false)

  const uploader = useCallback(async (releaseUrl: string) => {
    setWasmLoading(true)
    const wasm = await fetch(releaseUrl)

    const wasmBuffer = await wasm.arrayBuffer()
    const wasmModule = Array.from(new Uint8Array(wasmBuffer))

    console.log(`Wasm size:`, wasmModule.length)
    let uploadedSize = 0

    for await (const chunks of chunkGenerator(wasmModule)) {
      const result = await actor.load_wasm(chunks)
      console.log(`Chunks :`, result)

      uploadedSize += chunks.length

      setProgress((uploadedSize / wasmModule.length) * 100)
    }

    setWasmLoading(false)
  }, [])

  return {
    wasmLoading,
    progress,
    uploader
  }
}

export default useLoadRelease
