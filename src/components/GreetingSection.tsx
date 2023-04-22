import { Result } from "declarations/b3_user/b3_user.did"
import { useState } from "react"

// Dfinity
import { makeHelloActor } from "service/actor-locator"

export const GreetingSection = () => {
  const [name, setName] = useState("")
  const [loading, setLoading] = useState("")
  const [greetingMessage, setGreetingMessage] = useState<Result>()

  function onChangeName(e: React.ChangeEvent<HTMLInputElement>) {
    const newName = e.target.value
    setName(newName)
  }

  async function sayGreeting() {
    setGreetingMessage(undefined)
    setLoading("Loading...")

    const helloActor = makeHelloActor()
    const greeting = await helloActor.create_account({ Development: null }, [
      name
    ])

    setLoading("")
    setGreetingMessage(greeting)
  }

  console.log("greetingMessage", greetingMessage)

  return (
    <div>
      <section>
        <h2>Greeting</h2>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input
          id="name"
          alt="Name"
          type="text"
          value={name}
          onChange={onChangeName}
        />
        <button onClick={sayGreeting}>Send</button>
      </section>
      <section>
        <label>Response: &nbsp;</label>
        {loading}
        {JSON.stringify(greetingMessage)}
      </section>
    </div>
  )
}
