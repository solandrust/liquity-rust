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


## Starting


