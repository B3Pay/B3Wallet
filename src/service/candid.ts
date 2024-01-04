import {
  Actor,
  ActorMethod,
  ActorSubclass,
  CanisterStatus,
  HttpAgent
} from "@dfinity/agent"
import { IDL } from "@dfinity/candid"
import { Principal } from "@dfinity/principal"

const agent = new HttpAgent({ host: "https://ic0.app" })

export async function getIdlFactoryFromMetadata(canisterIdString: string) {
  const canisterId = Principal.fromText(canisterIdString)

  const status = await CanisterStatus.request({
    agent,
    canisterId,
    paths: ["candid"]
  })
  const did = status.get("candid") as string | null
  if (did) {
    return didToFactory(did)
  } else {
    return undefined
  }
}

export async function getIdlFactoryFromTmpHack(canisterIdString: string) {
  const canisterId = Principal.fromText(canisterIdString)

  type CommonInterface = {
    __get_candid_interface_tmp_hack: ActorMethod<[], string>
  }

  const common_interface: IDL.InterfaceFactory = ({ IDL }) =>
    IDL.Service({
      __get_candid_interface_tmp_hack: IDL.Func([], [IDL.Text], ["query"])
    })

  const actor = Actor.createActor<CommonInterface>(common_interface, {
    agent,
    canisterId
  })

  const data = await actor.__get_candid_interface_tmp_hack()

  return didToFactory(data)
}

async function didToFactory(candid_source: string) {
  // call didjs canister
  const didjs_id = "a4gq6-oaaaa-aaaab-qaa4q-cai"

  const didjs_interface: IDL.InterfaceFactory = ({ IDL }) =>
    IDL.Service({
      did_to_js: IDL.Func([IDL.Text], [IDL.Opt(IDL.Text)], ["query"])
    })

  const didjs: ActorSubclass = Actor.createActor(didjs_interface, {
    agent,
    canisterId: didjs_id
  })

  const js: any = await didjs.did_to_js(candid_source)

  const dataUri =
    "data:text/javascript;charset=utf-8," + encodeURIComponent(js[0])
  const candid: any = await eval('import("' + dataUri + '")')

  return candid.idlFactory
}
