import React, { useState, useEffect } from "react";
// Assuming these imports are correct and exist in your project
import {
  Predicate,
  Provider,
  Address,
  BN,
  ScriptTransactionRequest,
  InputValue,
  WalletUnlocked,
} from "fuels";
import { useWallet } from "@fuel-wallet/react";
import { PredicateAbi__factory } from "../multisig-predicate";

const Create: React.FC = () => {
  const [address1, setAddress1] = useState("");
  const [address2, setAddress2] = useState("");
  const [address3, setAddress3] = useState("");
  const [withdrawalAddress, setWithdrawalAddress] = useState("");
  const [amountToSend, setAmount] = useState<string>("0");
  const [threshold, setThreshold] = useState<number>(1);
  const [transactionHash, setTransactionHash] = useState("");
  const [predicateInstance, setPredicateInstance] = useState<Predicate<InputValue[]> | null>(null);
  const [request, setRequest] = useState<ScriptTransactionRequest>();
  const [signatures, setSignatures] = useState("");
  const [isMultisigGenerated, setIsMultisigGenerated] = useState(false);
  const [isBalanceNonZero, setIsBalanceNonZero] = useState(false);
  const [displayAddress, setDisplayAddress] = useState(""); // Added state for the display address
  const [displayBalance, setDisplayBalance] = useState(""); // Added state for the display address
  const [active, setActive] = useState<
    "Create Multisig" | "Build Transaction" | "Sign Transaction" | "Receipt"
  >("Create Multisig");

  const { wallet } = useWallet();

  // Reset isMultisigGenerated whenever address or threshold changes
  useEffect(() => {
    setIsMultisigGenerated(false);
  }, [address1, address2, address3, threshold]);

  const handleNextClick = () => {
    setActive("Build Transaction");
  };

  const handleBackClick = () => {
    switch (active) {
      case "Build Transaction":
        setActive("Create Multisig");
        break;
      case "Sign Transaction":
        setActive("Build Transaction");
        break;
      default:
        setActive("Create Multisig");
        break;
    }
  };

  const handleSignClick = async () => {
    console.log(await wallet?.address);
    if (wallet) {
        const currentSignature = await wallet.signMessage(transactionHash);
        setSignatures(currentSignature);
        // setSignatures((prevSignatures) => [...prevSignatures, currentSignature]);
        console.log("Signatures", signatures);
    }
  };

  const handleClearClick = async () => {
    setSignatures("");
  };

  const handleSendClick = async () => {
    console.log(await wallet?.address);
    try {
      const provider = await Provider.create(
        "https://beta-5.fuel.network/graphql"
      );
      const wallet = new WalletUnlocked(
        Address.fromB256(
          "0x1645232ff766da588e0883ac61318b2b91f9874fc6bf1cf170575a90a1743b3b"
        ).toBytes(),
        provider
      );
      console.log(signatures)
      const transactionRequest = await wallet?.populateTransactionWitnessesSignature({
        ...request,
        witnesses: [signatures],
      });
      const res = await predicateInstance?.sendTransaction(transactionRequest);
    } catch (error) {
      console.error("Error occurred while sending transaction:", error);
      // Handle the error here, you can log it or show a user-friendly message
      // For now, let's rethrow the error to propagate it further if necessary
      throw error;
    }
  };
  // 0xe1ebd1bcd5069cbda99831831398531a4e832c17eccf492ddcb8f28193e5b513
  const handleBuildClick = async () => {
    const areAllFieldsFilled =
      withdrawalAddress && displayBalance > amountToSend;
    if (areAllFieldsFilled) {
      setActive("Sign Transaction");
      const provider = await Provider.create(
        "https://beta-5.fuel.network/graphql"
      );
      const request = new ScriptTransactionRequest({
        script: PredicateAbi__factory.bin,
        gasLimit: 10_000,
        gasPrice: provider.getGasConfig().minGasPrice,
      });
      setRequest(request);
      setTransactionHash(request.getTransactionId(0));
      console.log(await request.getTransactionId(0));
      console.log(wallet);
    } else {
      alert("Please fill in destination address and amount to send");
    }
  };

  const handleConfigureClick = async () => {
    const areAllFieldsFilled =
      address1 && address2 && address3 && threshold > 0;
    if (areAllFieldsFilled) {
      setIsMultisigGenerated(true);
      try {
        const provider = await Provider.create(
          "https://beta-5.fuel.network/graphql"
        );
        const configurable = {
          REQUIRED_SIGNATURES: threshold,
          SIGNERS: [
            address1 ? { value: Address.fromString(address1).toB256() } : null,
            address2 ? { value: Address.fromString(address2).toB256() } : null,
            address3 ? { value: Address.fromString(address3).toB256() } : null,
          ].filter(Boolean), // Adjusted as per your actual type
        };
        // Assuming Predicate constructor matches your usage
        const predicateInstance = new Predicate(
          PredicateAbi__factory.bin,
          provider,
          PredicateAbi__factory.abi,
          configurable
        );
        setPredicateInstance(predicateInstance);
        const simulatedMultisigAddress = predicateInstance.address.toB256();
        console.log(await predicateInstance.getBalance());
        const simulatedMultisigBalance = new BN(
          await predicateInstance.getBalance()
        );
        if (!simulatedMultisigBalance.isZero()) {
          console.log(!simulatedMultisigBalance.isZero());
          setIsBalanceNonZero(true); // Assuming you want to set this state based on the balance check
        } else {
          setIsBalanceNonZero(false);
        }
        setDisplayAddress(simulatedMultisigAddress); // Set the simulated address for display
        setDisplayBalance(simulatedMultisigBalance.formatUnits().toString()); // Set the simulated address for display
        // Perform any action with predicateInstance here
      } catch (error) {
        alert(error);
      }
    } else {
      alert("Please fill all address fields and select a threshold.");
    }
  };

  return (
    <div>
      <h2>{active}</h2>
      {active === "Create Multisig" && (
        <div>
          {isMultisigGenerated && (
            <div>
              <strong>Predicate root:</strong> {displayAddress}
              <br />
              <strong>Available balance:</strong> {displayBalance} ETH
            </div>
          )}
          <input
            type="text"
            value={address1}
            onChange={(e) => setAddress1(e.target.value)}
            placeholder="0x0000000000000000000000000000000000000000"
          />
          <input
            type="text"
            value={address2}
            onChange={(e) => setAddress2(e.target.value)}
            placeholder="0x0000000000000000000000000000000000000000"
          />
          <input
            type="text"
            value={address3}
            onChange={(e) => setAddress3(e.target.value)}
            placeholder="0x0000000000000000000000000000000000000000"
          />

          {/* Dropdown menu for threshold */}
          <select
            value={threshold.toString()}
            onChange={(e) => setThreshold(parseInt(e.target.value))}
          >
            <option value="1">1</option>
            <option value="2">2</option>
            <option value="3">3</option>
          </select>

          {/* Configure button */}
          <button onClick={handleConfigureClick}>Configure</button>

          {/* Display the address if it's set */}
          {isMultisigGenerated && (
            <div>
              {isBalanceNonZero && (
                <button onClick={handleNextClick}>Next</button>
              )}
            </div>
          )}
        </div>
      )}
      {active === "Build Transaction" && (
        <div>
          {isMultisigGenerated && (
            <div>
              <strong>Predicate root:</strong> {displayAddress}
              <br />
              <strong>Available balance:</strong> {displayBalance} ETH
            </div>
          )}
          <input
            type="text"
            value={withdrawalAddress}
            onChange={(e) => setWithdrawalAddress(e.target.value)}
            placeholder="0x0000000000000000000000000000000000000000"
          />
          <input
            id="amountToSend"
            type="number"
            required
            min="0"
            step="any"
            inputMode="decimal"
            placeholder="0.00"
            onChange={(e) => {
              setAmount(e.target.value);
            }}
          />
          <br />
          <button onClick={handleBackClick}>Back</button>
          <button onClick={handleBuildClick}>Build</button>
        </div>
      )}
      {active === "Sign Transaction" && (
        <div>
          <div>
            <strong>Withdraw address:</strong> {withdrawalAddress}
            <br />
            <strong>Amount to send:</strong> {amountToSend} ETH
            <br />
            <strong>Transaction hash:</strong> {transactionHash}
            <br />
            {/* Map over signatures to display them */}
            {/* {signatures.map((signature, index) => (
              <div key={index}>
                Wallet {index + 1}: {signature} signed succesfully ✅
              </div>
            ))} */}
            <button onClick={handleSignClick}>Sign</button>
            <button onClick={handleClearClick}>Clear</button>
          </div>
          <button onClick={handleBackClick}>Back</button>
          <button onClick={handleSendClick}>Send</button>
        </div>
      )}
    </div>
  );
};

export default Create;
