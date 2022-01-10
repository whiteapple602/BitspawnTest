const anchor = require('@project-serum/anchor');
const BN = require('bn.js');
const expect = require('chai').expect;
const { SystemProgram, LAMPORTS_PER_SOL } = anchor.web3;

describe('todo', () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const mainProgram = anchor.workspace.Todo;

  function expectBalance(actual, expected, message, slack=20000) {
    expect(actual, message).within(expected - slack, expected + slack)
  }

  async function createUser(airdropBalance) {
    airdropBalance = airdropBalance ?? 10 * LAMPORTS_PER_SOL;
    let user = anchor.web3.Keypair.generate();
    let sig = await provider.connection.requestAirdrop(user.publicKey, airdropBalance);
    await provider.connection.confirmTransaction(sig);

    let wallet = new anchor.Wallet(user);
    let userProvider = new anchor.Provider(provider.connection, wallet, provider.opts);

    return {
      key: user,
      wallet,
      provider: userProvider,
    };
  }

  function createUsers(numUsers) {
    let promises = [];
    for(let i = 0; i < numUsers; i++) {
      promises.push(createUser());
    }

    return Promise.all(promises);
  }

  async function getAccountBalance(pubkey) {
    let account = await provider.connection.getAccountInfo(pubkey);
    return account?.lamports ?? 0;
  }

  function programForUser(user) {
    return new anchor.Program(mainProgram.idl, mainProgram.programId, user.provider);
  }

  async function createList(owner, name, capacity=16) {
    const [listAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([
      "nftPad",
      owner.key.publicKey.toBytes(),
      name.slice(0, 32)
    ], mainProgram.programId);

    let program = programForUser(owner);
    await program.rpc.newList(name, capacity, bump, {
      accounts: {
        list: listAccount,
        owner: owner.key.publicKey,
        name: "new Augment",
        capacity: 10,
        character: [],
        account_bump: bump,
        systemProgram: SystemProgram.programId,
      },
    });

    let list = await program.account.nftPad.fetch(listAccount);
    return { publicKey: listAccount, data: list };
  }
})