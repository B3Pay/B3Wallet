import { Button } from "components/ui/button"
import React, { useState } from "react"
import { useActorMethod } from "service/hello"
import { Input } from "./ui/input"

interface GreetingProps {}

const Greeting: React.FC<GreetingProps> = ({}) => {
  const { call, data, error, loading } = useActorMethod("greet")

  const [name, setName] = useState("")

  const onChangeName = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newName = e.target.value
    setName(newName)
  }

  return (
    <div className="space-x-2 space-y-2">
      <h4 className="scroll-m-20 text-xl font-semibold tracking-tight">
        Greeting
      </h4>
      <div className="flex items-center text-sm">
        <Input
          id="name"
          alt="Name"
          type="text"
          round="left"
          value={name}
          placeholder="Name"
          onChange={onChangeName}
        >
          <Button round="right" type="button" onClick={() => call(name)}>
            Send
          </Button>
        </Input>
      </div>
      <p className="text-sm">
        This component calls the <code>greet</code> method on the{" "}
        <code>hello</code> actor.
      </p>
      <div>
        {loading && <p>Loading...</p>}
        {error ? <p>Error: {JSON.stringify(error)}</p> : null}
        {data && <p>Message: {JSON.stringify(data)}</p>}
      </div>
      <div className="flex items-center text-sm">
        {["default", "outline", "ghost", "link"].map((variant, i) => (
          <div key={i}>
            <p>{variant}</p>
            {[
              "primary",
              "secondary",
              "error",
              "success",
              "warning",
              "info",
              "muted"
            ].map((color, i) => (
              <Button
                key={i}
                type="button"
                color={color as any}
                variant={variant as any}
                onClick={() => call(name)}
              >
                {color}
              </Button>
            ))}
          </div>
        ))}
      </div>
      <div className="flex items-center text-sm">
        {[
          "primary",
          "secondary",
          "error",
          "success",
          "warning",
          "info",
          "muted"
        ].map((color, i) => (
          <Input key={i} color={color as any} placeholder="Input" />
        ))}
      </div>
    </div>
  )
}

export default Greeting
