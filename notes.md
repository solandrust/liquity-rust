## 2021/08/10

We're starting a new project to help us learn Solana.
This time porting [liquity],
an stablecoin borrowing platform on Ethereum
that is kind of like MakerDAO
but simpler and without big governance.

[liquity]: https://github.com/liquity/dev

We are planning on basically copying the code.
As we start,
one of the things that is going to be challenging is that
Liquity relies a lot on inheritance in Solidity,
and there is no inheritance in Rust.
Solidity is a lot different than Rust in other ways too,
so it'll be an interesting challenge.

We also intend to use the [Anchor framework] for this project.
Interoperability standards between Solana programs is minimal &mdash;
it's an immature ecosystem &mdash;
but Anchor is by the Serum developers,
and Serum is the biggest app on Solana,
and Anchor is intended to define certain standards on Solana.
Anchor though is not production-ready yet,
so we are certainly setting ourselves up for unexpected pain.

[Anchor framework]: https://github.com/project-serum/anchor


## Creating the project

We start by creating a directory structure to mirror Liquity's,
and decide the [LiquityBase.sol] contract is a good one
to start cloning,
so add a directory, `packages/contracts/contracts/liquity-base`.

From there we add a skeleton Solana entrypoint with the [`solana-program`] crate,
and make sure we can build with `cargo build-bpf`.

After that works, we add the [`anchor-lang`] dependency,
and completely rewrite the `liquity-base` crate to contain
the anchor "counter" example code,
then get that to build.


## Cloning liquity

Now we take a closer look at `LiquityBase.sol` and think about how to
translate it to Rust.
This contract, which I am thinking of as a "class",
mostly contains constants and methods on those constants.
Note something I would use inheritance, classes, or data structures for in Rust.
But the `LiquityBase` contract also contains a few abstract fields,
like

```solidity
IActivePool public activePool;
IDefaultPool public defaultPool;
```

So `LiquityBase` depends on some behavior from other types,
but doesn't care about the implementation.

For this we probably do want a data structure and implementation in Rust.

It looks though like our `liquity-base` is _not_ a contract (or Solana program),
but just a Rust library that the eventual program will link to.

So now I want to sketch out what a near-exact mirror of the Solidity `LiquityBase`
contract might look like in Rust.

For now I'm just going to write all of it in the `liquity-base` crate
I have already created,
but once I've reached another milestone,
I'll refactor it into modules and/or crates.

