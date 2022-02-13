import { useEffect, useState, useRef } from "react";
import ReactDOM from "react-dom";
import * as buffer from "buffer";
window.Buffer = buffer.Buffer; // https://github.com/isaacs/core-util-is/issues/27#issuecomment-878969583
import * as nearAPI from "near-api-js";
import bs58 from "bs58";
import sha256 from "js-sha256";

import getConfig from "./config";
import * as css from "./index.module.css";

async function viewMethodOnContract(nearConfig, method, params) {
  const paramBytes = Buffer.from(params, "utf8");
  const base58Params = bs58.encode(paramBytes);

  const provider = new nearAPI.providers.JsonRpcProvider(nearConfig.nodeUrl);
  const rawResult = await provider.query(
    `call/${nearConfig.contractName}/${method}`,
    base58Params
  );
  return JSON.parse(
    rawResult.result.map((x) => String.fromCharCode(x)).join("")
  );
}

function App({ question }) {
  const [wallet, setWallet] = useState(null);
  const [contract, setContract] = useState(null);
  const [puzzle, setPuzzle] = useState({ question: "", solution: "" });
  const [prize, setPrize] = useState("");
  const [answer, setAnswer] = useState("");
  const [isWrongAnswer, setIsWrongAnswer] = useState(false);

  const logIn = async () => {
    const nearConfig = getConfig(process.env.NEAR_ENV || "testnet");
    if (!wallet.isSignedIn()) {
      wallet.requestSignIn(
        nearConfig.contractName, // contract requesting access
        "Raaz" // optional
      );
    }
  };

  const submit = async (e) => {
    e.preventDefault();
    const solution = sha256(answer);

    if (solution !== puzzle.solution) {
      setIsWrongAnswer(true);
      setTimeout(() => setIsWrongAnswer(false), 800);
      return;
    }

    // contract.guess_solution()
  };

  useEffect(() => {
    const init = async () => {
      const nearConfig = getConfig(process.env.NEAR_ENV || "testnet");
      // create a keyStore for signing transactions using the user's key
      // which is located in the browser local storage after user logs in
      const keyStore = new nearAPI.keyStores.BrowserLocalStorageKeyStore();
      const near = await nearAPI.connect({ keyStore, ...nearConfig });
      const wallet = new nearAPI.WalletConnection(near);
      const contract = new nearAPI.Contract(
        wallet.account(), // the account object that is connecting
        nearConfig.contractName,
        {
          // name of contract you're connecting to
          viewMethods: ["get_puzzle", "get_ammount_prize"], // view methods do not change state but usually return a value
          changeMethods: ["guess_solution"], // change methods modify state
          sender: wallet.account(), // account object to initialize and sign transactions.
        }
      );
      setWallet(wallet);
      setContract(contract);

      const puzzle = await contract.get_puzzle();
      const prize = await contract.get_ammount_prize();
      setPrize(prize);
      setPuzzle(puzzle);
    };

    init();
  }, []);

  // console.log("wallet", wallet);

  const isLoggedIn = wallet?.isSignedIn();

  return (
    <>
      <div className={css.stars}></div>
      <div className={css.twinkling}></div>
      <div className={css.clouds}></div>
      <h1 className={css.h1}>Raaz</h1>
      <h2 className={css.h2}>
        Answer the puzzle below. Each guess requires you to deposit 5 yoctoNear.
        If you're wrong, the deposit is lost. If you're right then everyone
        else's deposit is yours.
      </h2>
      <main className={css.main}>
        <p className={css.p}>{puzzle.question}</p>
        <form onSubmit={submit}>
          <input
            type="text"
            className={css.input + " " + (isWrongAnswer ? css.wrong : "")}
            placeholder={
              isLoggedIn ? "Type here..." : "Log in first to answer..."
            }
            disabled={!isLoggedIn}
            value={answer}
            onChange={(e) => setAnswer(e.target.value)}
          />
        </form>
        {isLoggedIn ? (
          <button className={css.button} onClick={submit}>
            Submit
          </button>
        ) : (
          <button className={css.button} onClick={logIn}>
            Log in
          </button>
        )}
      </main>
    </>
  );
}

const app = document.getElementById("app");
ReactDOM.render(
  <App
    question={"What is the captial of Paris?"}
    prize={"0.005 NEAR tokens"}
  />,
  app
);
