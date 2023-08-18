import type { BackendActor } from "./actor"

import * as agent from "@dfinity/agent"

// Usage of the imported bindings only works if the respective .wasm was loaded, which is done in main.ts.
// See also https://github.com/rollup/plugins/tree/master/packages/wasm#using-with-wasm-bindgen-and-wasm-pack
import * as vetkd from "../../vetkd_user_lib/ic_vetkd_utils.js"

export class CryptoService {
  constructor(private actor: BackendActor) {}

  // Symmetric AES key, used to encrypt and decrypt the notes stored in the dapp
  private vetAesGcmKey: CryptoKey | null = null

  // Function to initialize the service with a password for the first time
  public async createVetkd(password: string = "123") {
    // Create a transport secret key from a 32-byte seed
    const seed = window.crypto.getRandomValues(new Uint8Array(32))
    const tsk = new vetkd.TransportSecretKey(seed)

    // Get the encrypted symmetric key for the caller from the actor
    const ek_bytes_hex = await this.actor.encrypted_symmetric_key_for_caller(
      tsk.public_key()
    )

    // Get the symmetric key verification key from the actor
    const pk_bytes_hex = await this.actor.symmetric_key_verification_key()
    const principal = await agent.Actor.agentOf(this.actor).getPrincipal()

    // Decrypt and hash the encrypted key using the transport secret key
    const aes_256_gcm_key_raw = tsk.decrypt(
      hex_decode(ek_bytes_hex),
      hex_decode(pk_bytes_hex),
      principal.toUint8Array(),
      32,
      new TextEncoder().encode("aes-256-gcm")
    )

    console.log("aes_256_gcm_key_raw", aes_256_gcm_key_raw)

    let encryptedSymmetricKey = await encryptKey(aes_256_gcm_key_raw, password)
    console.log("encryptedSymmetricKey", encryptedSymmetricKey)
    this.actor.setEncryptedSymmetricKeyForCaller(encryptedSymmetricKey)

    // Import the decrypted symmetric key
    this.vetAesGcmKey = await window.crypto.subtle.importKey(
      "raw",
      aes_256_gcm_key_raw,
      "AES-GCM",
      false,
      ["encrypt", "decrypt"]
    )
  }

  /**
   * Fetch the authenticated user's vetKD key and derive an AES-GCM key from it
   */
  public async init(password: string = "123") {
    const aes_256_gcm_key_raw =
      await this.actor.encryptedSymmetricKeyForCaller()

    if ("Ok" in aes_256_gcm_key_raw) {
      const decryptedSymmetricKey = await decryptKey(
        aes_256_gcm_key_raw.Ok,
        password
      )
      this.vetAesGcmKey = await window.crypto.subtle.importKey(
        "raw",
        decryptedSymmetricKey,
        "AES-GCM",
        false,
        ["encrypt", "decrypt"]
      )
    } else {
      this.createVetkd(password)
    }
  }

  public logout() {
    this.vetAesGcmKey = null
  }

  public isInitialized() {
    return this.vetAesGcmKey !== null
  }

  // The function encrypts data with the shared secretKey.
  public async encrypt(data: string) {
    if (this.vetAesGcmKey === null) {
      throw new Error("null shared secret!")
    }
    const data_encoded = Uint8Array.from(
      [...data].map(ch => ch.charCodeAt(0))
    ).buffer
    // The iv must never be reused with a given key.
    const iv = window.crypto.getRandomValues(new Uint8Array(12))
    const ciphertext = await window.crypto.subtle.encrypt(
      {
        name: "AES-GCM",
        iv: iv
      },
      this.vetAesGcmKey,
      data_encoded
    )

    const iv_decoded = String.fromCharCode(...new Uint8Array(iv))
    const cipher_decoded = String.fromCharCode(...new Uint8Array(ciphertext))
    return iv_decoded + cipher_decoded
  }

  // The function decrypts the given input data.
  public async decrypt(data: string) {
    if (this.vetAesGcmKey === null) {
      throw new Error("null shared secret!")
    }
    if (data.length < 13) {
      throw new Error("wrong encoding, too short to contain iv")
    }
    const iv_decoded = data.slice(0, 12)
    const cipher_decoded = data.slice(12)
    const iv_encoded = Uint8Array.from(
      [...iv_decoded].map(ch => ch.charCodeAt(0))
    ).buffer
    const ciphertext_encoded = Uint8Array.from(
      [...cipher_decoded].map(ch => ch.charCodeAt(0))
    ).buffer

    let decrypted_data_encoded = await window.crypto.subtle.decrypt(
      {
        name: "AES-GCM",
        iv: iv_encoded
      },
      this.vetAesGcmKey,
      ciphertext_encoded
    )
    const decrypted_data_decoded = String.fromCharCode(
      ...new Uint8Array(decrypted_data_encoded)
    )
    return decrypted_data_decoded
  }
}

const hex_decode = hexString =>
  Uint8Array.from(hexString.match(/.{1,2}/g).map(byte => parseInt(byte, 16)))
const hex_encode = bytes =>
  bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, "0"), "")

// Function to derive a key from a password
async function deriveKey(
  password: string,
  salt: Uint8Array
): Promise<CryptoKey> {
  const encoder = new TextEncoder()
  const passwordBuffer = encoder.encode(password)
  const keyMaterial = await window.crypto.subtle.importKey(
    "raw",
    passwordBuffer,
    "PBKDF2",
    false,
    ["deriveKey"]
  )
  return window.crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt: salt,
      iterations: 100000,
      hash: "SHA-256"
    },
    keyMaterial,
    { name: "AES-GCM", length: 256 },
    false,
    ["encrypt", "decrypt"]
  )
}

// Function to encrypt the aes_256_gcm_key_raw using a password
async function encryptKey(
  aes_256_gcm_key_raw: Uint8Array,
  password: string
): Promise<Uint8Array> {
  const salt = window.crypto.getRandomValues(new Uint8Array(16))
  const derivedKey = await deriveKey(password, salt)
  const iv = window.crypto.getRandomValues(new Uint8Array(12))
  const encryptedKey = await window.crypto.subtle.encrypt(
    { name: "AES-GCM", iv: iv },
    derivedKey,
    aes_256_gcm_key_raw
  )
  // Concatenate salt, iv, and encrypted key to send to the server
  return new Uint8Array([...salt, ...iv, ...new Uint8Array(encryptedKey)])
}

// Function to decrypt the encrypted key using a password
async function decryptKey(
  encryptedKeyWithSaltIv: Uint8Array | number[],
  password: string
): Promise<Uint8Array> {
  const salt = encryptedKeyWithSaltIv.slice(0, 16)
  const iv = encryptedKeyWithSaltIv.slice(16, 28)
  const encryptedKey = new Uint8Array(encryptedKeyWithSaltIv.slice(28))
  const derivedKey = await deriveKey(password, new Uint8Array(salt))
  const decryptedKey = await window.crypto.subtle.decrypt(
    { name: "AES-GCM", iv: new Uint8Array(iv) },
    derivedKey,
    encryptedKey
  )
  return new Uint8Array(decryptedKey)
}
