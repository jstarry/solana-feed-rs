import("@solana/web3.js").then(() => {
  import("../crates/webapp/pkg").then(module => {
    module.run_app();
  });
});
