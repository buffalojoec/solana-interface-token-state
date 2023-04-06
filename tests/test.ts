import {
    clusterApiUrl,
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    sendAndConfirmTransaction,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js'
import fs from 'fs'
import {
    it,
    describe,
} from 'mocha'
import os from 'os'

function loadKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(fs.readFileSync(path, "utf-8")))
    )
}

const sleepSeconds = async (s: number) => await new Promise(f => setTimeout(f, s))

const CONNECTION = new Connection(clusterApiUrl('devnet'), 'confirmed')
const PAYER = loadKeypairFromFile(os.homedir() + '/.config/solana/id.json')
const PROGRAM = loadKeypairFromFile('program/target/deploy/program-keypair.json')

async function sendTestTransaction(ix: TransactionInstruction) {
    await sleepSeconds(5)
    await sendAndConfirmTransaction(
        CONNECTION, 
        new Transaction().add(ix),
        [PAYER],
        {skipPreflight: true}
    )
}

describe("Interface Token State", async () => {
    it("[ Interface Macro V1 ]:     Create Token V1", async () => createCreateTokenV1())
    it("[ Interface Macro V1 ]:     Read Token V1", async () => createReadTokenV1())
    it("[ Interface Macro V2 ]:     Create Token V2", async () => createCreateTokenV2())
    it("[ Interface Macro V2 ]:     Read Token V2", async () => createReadTokenV2())
    it("[ Interface Macro V2 ]:     Create Token V3", async () => createCreateTokenV3())
    it("[ Interface Macro V2 ]:     Read Token V3", async () => createReadTokenV3())
})
  