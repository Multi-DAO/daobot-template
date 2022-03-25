import "regenerator-runtime/runtime";
import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import Big from "big.js";
import SetupDaoForm from "./components/SetupDaoForm";

const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();

const App = ({ contract, currentUser, nearConfig, wallet, nearApi }) => {

  const signIn = () => {
    wallet.requestSignIn(
      nearConfig.contractName,
      "AutoDAO"
    );
  };

  const signOut = () => {
    wallet.signOut();
    window.location.replace(window.location.origin + window.location.pathname);
  };

  const [dao, setDao] = useState(localStorage.getItem("dao") || "");
  const [nft, setNft] = useState(localStorage.getItem("nft") || "");

  // save to local storage
  useEffect(() => {
    if (dao) {
      localStorage.setItem("dao", dao);
    }
    if (nft) {
      localStorage.setItem("nft", nft);
    }
  }, [dao, nft]);

  const registerDaoBotWithDao = async () => {
    if (!dao) { alert("Please enter a dao address"); return; }
    const daocontract = await new nearApi.Contract(wallet.account(), dao, {
      viewMethods: ["get_policy"],
      changeMethods: ["add_proposal"],
      sender: wallet.getAccountId()
    });
    

    const policy = await daocontract.get_policy();
    const newRole = { ...policy.roles[1], name: "DAO Bot", kind: { Group: [contract.contractId] } };
    policy.roles.push(newRole);
    await daocontract.add_proposal({
      proposal: {
        description: "Add DAO Bot role",
        kind: {
          ChangePolicy: {
            policy: policy
          }
        }
      }
    }, "300000000000000", // attached GAS (optional)
      "1000000000000000000000000" // attached deposit in yoctoNEAR (optional));
    );
  };

  const registerCroncat = async () => {
    if (!dao) { alert("Please enter a dao address"); return; }
    const croncatContract = await new nearApi.Contract(wallet.account(), "manager_v1.croncat.testnet", {
      changeMethods: ["create_task"],
      sender: wallet.getAccountId()
    });

    const taskArgs = {
      "contract_id": contract.contractId,
      "function_id": "approve_members",
      "cadence": "*/10 * * * * *",
      "recurring": true,
      "deposit": "0",
      "gas": 240000000000000,
      "arguments": btoa(JSON.stringify({
        'dao_id': dao,
        'nft_id': nft
      }))
    };

    const taskId = await croncatContract.create_task(taskArgs, BOATLOAD_OF_GAS, "1000000000000000000000000");

  };

  // Use get_hash to check for created task: https://github.com/CronCats/cron-ui/blob/main/src/pages/Tasks.vue#L320
  return (
    <main>
      <header>
        <h1>AutoDAO Message</h1>

        {currentUser ?
          <p>Currently signed in as: <code>{currentUser.accountId}</code></p>
          :
          <p>Add an AutoDAO bot! Please login to continue.</p>
        }

        {currentUser
          ? <button onClick={signOut}>Log out</button>
          : <button onClick={signIn}>Log in</button> 
        }
      </header>

      {currentUser && < SetupDaoForm daoId={dao} setDaoId={(id) => setDao(id)} setNftId={(id) => setNft(id)} nftId={nft} onSubmit={(e) => { registerDaoBotWithDao(); e.preventDefault(); }} />}

      {currentUser && <p>Click to go to the DAO and approve the proposal: <a href={"https://testnet-v2.sputnik.fund/#/" + dao} target="_blank">DAO Link</a></p>}

      {currentUser && <button onClick={(e) => { registerCroncat(); e.preventDefault() }}>Register Croncat</button>}

      {currentUser && <p> Try creating a new proposal to add a member to the dao, it should approve the proposal automatically if it owns a token of the nft {nft || "none provided"}.</p>}

    </main>
  );
};

App.propTypes = {
  contract: PropTypes.shape({
  }).isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.string.isRequired
  }),
  nearConfig: PropTypes.shape({
    contractName: PropTypes.string.isRequired
  }).isRequired,
  wallet: PropTypes.shape({
    requestSignIn: PropTypes.func.isRequired,
    signOut: PropTypes.func.isRequired
  }).isRequired
};

export default App;
