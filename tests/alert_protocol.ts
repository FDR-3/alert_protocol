import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { AlertProtocol } from "../target/types/alert_protocol"
import { assert } from "chai"
import { PublicKey } from '@solana/web3.js'

describe("alert_protocol", () =>
{
  //Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.alertProtocol as Program<AlertProtocol>
  const textWith444Characters = "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum feli"
  const notCEOErrorMsg = "Only the CEO can call this function"

  let successorWallet = anchor.web3.Keypair.generate()

  it("Initializes Alert Protocol", async () =>
  {
    await program.methods.initializeAlertProtocol().rpc()
    
    var ceoAccount = await program.account.alertProtocolCeo.fetch(getAlertProtocolCEOAccountPDA())
    assert(ceoAccount.address.toBase58() == program.provider.publicKey.toBase58())
  })

  it("Passes on the Alert Protocol CEO Account", async () => 
  {
    await airDropSol(successorWallet.publicKey)

    await program.methods.passOnAlertProtocolCeo(successorWallet.publicKey, ).rpc()
    
    var ceoAccount = await program.account.alertProtocolCeo.fetch(getAlertProtocolCEOAccountPDA())
    assert(ceoAccount.address.toBase58() == successorWallet.publicKey.toBase58())
  })
  
  it("Passes back the Alert Protocol CEO Account", async () => 
  {
    await program.methods.passOnAlertProtocolCeo(program.provider.publicKey).
    accounts({signer: successorWallet.publicKey})
    .signers([successorWallet])
    .rpc()
    
    var ceoAccount = await program.account.alertProtocolCeo.fetch(getAlertProtocolCEOAccountPDA())
    assert(ceoAccount.address.toBase58() == program.provider.publicKey.toBase58())
  })

  it("Verifies That Only CEO Can Pass On Account", async () => 
  {
    var errorMessage = ""

    try
    {
      await program.methods.passOnAlertProtocolCeo(program.provider.publicKey).
      accounts({signer: successorWallet.publicKey})
      .signers([successorWallet])
      .rpc()
    }
    catch(error)
    {
      errorMessage = error.error.errorMessage
    }

    assert(errorMessage == notCEOErrorMsg)
  })

  it("Clocks In Dead Man's Break", async () => 
  {
    var deadMansBreakAlert = await program.account.deadMansBreakAlert.fetch(getDeadMansBreakAlertPDA())
    assert(deadMansBreakAlert.unixClockInTimeStamp.eq(new anchor.BN(0)))

    await program.methods.clockInDeadMansBreak().rpc()
    
    deadMansBreakAlert = await program.account.deadMansBreakAlert.fetch(getDeadMansBreakAlertPDA())
    assert(deadMansBreakAlert.unixClockInTimeStamp.gt(new anchor.BN(0)))
  })

  it("Triggers New UI Available Alert", async () => 
  {
    var siteUpdateAlert = await program.account.siteUpdateAlert.fetch(getSiteUpdateAlertPDA())
    assert(siteUpdateAlert.siteUpdateCount.eq(new anchor.BN(0)))

    await program.methods.triggerNewUiAvailableAlert().rpc()
    
    siteUpdateAlert = await program.account.siteUpdateAlert.fetch(getSiteUpdateAlertPDA())
    assert(siteUpdateAlert.siteUpdateCount.eq(new anchor.BN(1)))
  })

  it("Triggers New PSA Alert", async () => 
  {
    var sitePSAAlert = await program.account.sitePsaAlert.fetch(getSitePSAAlertPDA())
    assert(sitePSAAlert.sitePsaMsg == "")

    await program.methods.triggerNewPsaAlert(textWith444Characters).rpc()
    
    sitePSAAlert = await program.account.sitePsaAlert.fetch(getSitePSAAlertPDA())
    assert(sitePSAAlert.sitePsaMsg == textWith444Characters)
  })

  it("Toggles Site SOS Alert", async () => 
  {
    var siteSOSAlert = await program.account.siteSosAlert.fetch(getSiteSOSAlertPDA())
    assert(!siteSOSAlert.sosFlag)

    await program.methods.toggleSosAlert(true).rpc()
    siteSOSAlert = await program.account.siteSosAlert.fetch(getSiteSOSAlertPDA())
    assert(siteSOSAlert.sosFlag)

    await program.methods.toggleSosAlert(false).rpc()
    siteSOSAlert = await program.account.siteSosAlert.fetch(getSiteSOSAlertPDA())
    assert(!siteSOSAlert.sosFlag)
  })

  function getAlertProtocolCEOAccountPDA()
  {
    const [alertProtocolCEOPDA] = anchor.web3.PublicKey.findProgramAddressSync
    (
      [
        new TextEncoder().encode("alertProtocolCEO")
      ],
      program.programId
    )
    return alertProtocolCEOPDA
  }

  function getDeadMansBreakAlertPDA()
  {
    const [deadMansBreakAlertPDA] = anchor.web3.PublicKey.findProgramAddressSync
    (
      [
        new TextEncoder().encode("deadMansBreakAlert")
      ],
      program.programId
    )
    return deadMansBreakAlertPDA
  }

  function getSiteUpdateAlertPDA()
  {
    const [siteUpdateAlertPDA] = anchor.web3.PublicKey.findProgramAddressSync
    (
      [
        new TextEncoder().encode("siteUpdateAlert")
      ],
      program.programId
    )
    return siteUpdateAlertPDA
  }

  function getSitePSAAlertPDA()
  {
    const [sitePSAAlertPDA] = anchor.web3.PublicKey.findProgramAddressSync
    (
      [
        new TextEncoder().encode("sitePSAAlert")
      ],
      program.programId
    )
    return sitePSAAlertPDA
  }

  function getSiteSOSAlertPDA()
  {
    const [siteSOSAlertPDA] = anchor.web3.PublicKey.findProgramAddressSync
    (
      [
        new TextEncoder().encode("siteSOSAlert")
      ],
      program.programId
    )
    return siteSOSAlertPDA
  }

  async function airDropSol(walletPublicKey: PublicKey)
  {
    let token_airdrop = await program.provider.connection.requestAirdrop(walletPublicKey, 
    100 * 1000000000) //1 billion lamports equals 1 SOL

    const latestBlockHash = await program.provider.connection.getLatestBlockhash()
    await program.provider.connection.confirmTransaction
    ({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: token_airdrop
    })
  }
})
