# mpc-signing

## MPC Signing with trusted dealer:

In one terminal, start the dealer, who is listening on 127.0.0.1:50052:

```bash
cargo run -p dealer
```

And in another terminal, run the gateway to send the signing request:

```bash
cargo run -p gateway -- --max-signers <MAX> --min-signers <MIN> sign <LABEL> <MESSAGE>
```

### Example

Run the example to test dealer:

```bash
cargo run -p dealer --example dealer
```
