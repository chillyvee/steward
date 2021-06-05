//! `transfer` subcommand - this subcommand transfers ethereum from one account to another
use crate::application::APP;
/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use abscissa_core::{Command, Options, Runnable};
use ethers::{
    abi::Abi,
    types::{Address},
    contract::Contract,
    providers::{Provider, Http},
};
use std::convert::TryFrom;
use serde_json;

///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct CellarcontractCmd {
    #[options(free)]
    recipient: Vec<String>,
}

impl Runnable for CellarcontractCmd {
    /// Transfer ETH from one account to another with Ganache blockchain emulator.
    fn run(&self) {
        abscissa_tokio::run(&APP, async {
            // this is a fake address used just for this example
            let address = "0x1dF62f291b2E969fB0849d99D9Ce41e2F137006e".parse::<Address>().unwrap();
            
            // ugly way to write the ABI inline, the next PR will read ABI from a JSON file
            let abi: Abi = serde_json::from_str(r#"[
                {
                    "inputs": [
                        {
                            "internalType": "string",
                            "name": "name_",
                            "type": "string"
                        },
                        {
                            "internalType": "string",
                            "name": "symbol_",
                            "type": "string"
                        },
                        {
                            "internalType": "address",
                            "name": "_token0",
                            "type": "address"
                        },
                        {
                            "internalType": "address",
                            "name": "_token1",
                            "type": "address"
                        },
                        {
                            "internalType": "uint24",
                            "name": "_feeLevel",
                            "type": "uint24"
                        },
                        {
                            "components": [
                                {
                                    "internalType": "uint184",
                                    "name": "tokenId",
                                    "type": "uint184"
                                },
                                {
                                    "internalType": "int24",
                                    "name": "tickUpper",
                                    "type": "int24"
                                },
                                {
                                    "internalType": "int24",
                                    "name": "tickLower",
                                    "type": "int24"
                                },
                                {
                                    "internalType": "uint24",
                                    "name": "weight",
                                    "type": "uint24"
                                }
                            ],
                            "internalType": "struct ICellarPoolShare.CellarTickInfo[]",
                            "name": "_cellarTickInfo",
                            "type": "tuple[]"
                        }
                    ],
                    "stateMutability": "nonpayable",
                    "type": "constructor"
                },
                {
                    "anonymous": false,
                    "inputs": [
                        {
                            "indexed": true,
                            "internalType": "address",
                            "name": "token0",
                            "type": "address"
                        },
                        {
                            "indexed": true,
                            "internalType": "address",
                            "name": "token1",
                            "type": "address"
                        },
                        {
                            "indexed": false,
                            "internalType": "uint128",
                            "name": "liquidity",
                            "type": "uint128"
                        },
                        {
                            "indexed": false,
                            "internalType": "uint256",
                            "name": "amount0",
                            "type": "uint256"
                        },
                        {
                            "indexed": false,
                            "internalType": "uint256",
                            "name": "amount1",
                            "type": "uint256"
                        }
                    ],
                    "name": "AddedLiquidity",
                    "type": "event"
                },
                {
                    "anonymous": false,
                    "inputs": [
                        {
                            "indexed": true,
                            "internalType": "address",
                            "name": "owner",
                            "type": "address"
                        },
                        {
                            "indexed": true,
                            "internalType": "address",
                            "name": "spender",
                            "type": "address"
                        },
                        {
                            "indexed": false,
                            "internalType": "uint256",
                            "name": "value",
                            "type": "uint256"
                        }
                    ],
                    "name": "Approval",
                    "type": "event"
                },
                {
                    "anonymous": false,
                    "inputs": [
                        {
                            "indexed": true,
                            "internalType": "address",
                            "name": "token0",
                            "type": "address"
                        },
                        {
                            "indexed": true,
                            "internalType": "address",
                            "name": "token1",
                            "type": "address"
                        },
                        {
                            "indexed": false,
                            "internalType": "uint128",
                            "name": "liquidity",
                            "type": "uint128"
                        },
                        {
                            "indexed": false,
                            "internalType": "uint256",
                            "name": "amount0",
                            "type": "uint256"
                        },
                        {
                            "indexed": false,
                            "internalType": "uint256",
                            "name": "amount1",
                            "type": "uint256"
                        }
                    ],
                    "name": "RemovedLiquidity",
                    "type": "event"
                },
                {
                    "anonymous": false,
                    "inputs": [
                        {
                            "indexed": true,
                            "internalType": "address",
                            "name": "from",
                            "type": "address"
                        },
                        {
                            "indexed": true,
                            "internalType": "address",
                            "name": "to",
                            "type": "address"
                        },
                        {
                            "indexed": false,
                            "internalType": "uint256",
                            "name": "value",
                            "type": "uint256"
                        }
                    ],
                    "name": "Transfer",
                    "type": "event"
                },
                {
                    "inputs": [],
                    "name": "FEEDOMINATOR",
                    "outputs": [
                        {
                            "internalType": "uint16",
                            "name": "",
                            "type": "uint16"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "NONFUNGIBLEPOSITIONMANAGER",
                    "outputs": [
                        {
                            "internalType": "address",
                            "name": "",
                            "type": "address"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "SWAPROUTER",
                    "outputs": [
                        {
                            "internalType": "address",
                            "name": "",
                            "type": "address"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "UNISWAPV3FACTORY",
                    "outputs": [
                        {
                            "internalType": "address",
                            "name": "",
                            "type": "address"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "WETH",
                    "outputs": [
                        {
                            "internalType": "address",
                            "name": "",
                            "type": "address"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "components": [
                                {
                                    "internalType": "uint256",
                                    "name": "amount0Desired",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "amount1Desired",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "amount0Min",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "amount1Min",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "address",
                                    "name": "recipient",
                                    "type": "address"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "deadline",
                                    "type": "uint256"
                                }
                            ],
                            "internalType": "struct ICellarPoolShare.CellarAddParams",
                            "name": "cellarParams",
                            "type": "tuple"
                        }
                    ],
                    "name": "addLiquidityEthForUniV3",
                    "outputs": [],
                    "stateMutability": "payable",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "components": [
                                {
                                    "internalType": "uint256",
                                    "name": "amount0Desired",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "amount1Desired",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "amount0Min",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "amount1Min",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "address",
                                    "name": "recipient",
                                    "type": "address"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "deadline",
                                    "type": "uint256"
                                }
                            ],
                            "internalType": "struct ICellarPoolShare.CellarAddParams",
                            "name": "cellarParams",
                            "type": "tuple"
                        }
                    ],
                    "name": "addLiquidityForUniV3",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "address",
                            "name": "owner_",
                            "type": "address"
                        },
                        {
                            "internalType": "address",
                            "name": "spender",
                            "type": "address"
                        }
                    ],
                    "name": "allowance",
                    "outputs": [
                        {
                            "internalType": "uint256",
                            "name": "",
                            "type": "uint256"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "address",
                            "name": "spender",
                            "type": "address"
                        },
                        {
                            "internalType": "uint256",
                            "name": "amount",
                            "type": "uint256"
                        }
                    ],
                    "name": "approve",
                    "outputs": [
                        {
                            "internalType": "bool",
                            "name": "",
                            "type": "bool"
                        }
                    ],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "address",
                            "name": "account",
                            "type": "address"
                        }
                    ],
                    "name": "balanceOf",
                    "outputs": [
                        {
                            "internalType": "uint256",
                            "name": "",
                            "type": "uint256"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "uint256",
                            "name": "",
                            "type": "uint256"
                        }
                    ],
                    "name": "cellarTickInfo",
                    "outputs": [
                        {
                            "internalType": "uint184",
                            "name": "tokenId",
                            "type": "uint184"
                        },
                        {
                            "internalType": "int24",
                            "name": "tickUpper",
                            "type": "int24"
                        },
                        {
                            "internalType": "int24",
                            "name": "tickLower",
                            "type": "int24"
                        },
                        {
                            "internalType": "uint24",
                            "name": "weight",
                            "type": "uint24"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "decimals",
                    "outputs": [
                        {
                            "internalType": "uint8",
                            "name": "",
                            "type": "uint8"
                        }
                    ],
                    "stateMutability": "pure",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "fee",
                    "outputs": [
                        {
                            "internalType": "uint16",
                            "name": "",
                            "type": "uint16"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "feeLevel",
                    "outputs": [
                        {
                            "internalType": "uint24",
                            "name": "",
                            "type": "uint24"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "name",
                    "outputs": [
                        {
                            "internalType": "string",
                            "name": "",
                            "type": "string"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "owner",
                    "outputs": [
                        {
                            "internalType": "address",
                            "name": "",
                            "type": "address"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "components": [
                                {
                                    "internalType": "uint184",
                                    "name": "tokenId",
                                    "type": "uint184"
                                },
                                {
                                    "internalType": "int24",
                                    "name": "tickUpper",
                                    "type": "int24"
                                },
                                {
                                    "internalType": "int24",
                                    "name": "tickLower",
                                    "type": "int24"
                                },
                                {
                                    "internalType": "uint24",
                                    "name": "weight",
                                    "type": "uint24"
                                }
                            ],
                            "internalType": "struct ICellarPoolShare.CellarTickInfo[]",
                            "name": "_cellarTickInfo",
                            "type": "tuple[]"
                        }
                    ],
                    "name": "rebalance",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "reinvest",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "components": [
                                {
                                    "internalType": "uint256",
                                    "name": "tokenAmount",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "amount0Min",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "amount1Min",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "address",
                                    "name": "recipient",
                                    "type": "address"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "deadline",
                                    "type": "uint256"
                                }
                            ],
                            "internalType": "struct ICellarPoolShare.CellarRemoveParams",
                            "name": "cellarParams",
                            "type": "tuple"
                        }
                    ],
                    "name": "removeLiquidityEthFromUniV3",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "components": [
                                {
                                    "internalType": "uint256",
                                    "name": "tokenAmount",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "amount0Min",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "amount1Min",
                                    "type": "uint256"
                                },
                                {
                                    "internalType": "address",
                                    "name": "recipient",
                                    "type": "address"
                                },
                                {
                                    "internalType": "uint256",
                                    "name": "deadline",
                                    "type": "uint256"
                                }
                            ],
                            "internalType": "struct ICellarPoolShare.CellarRemoveParams",
                            "name": "cellarParams",
                            "type": "tuple"
                        }
                    ],
                    "name": "removeLiquidityFromUniV3",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "uint16",
                            "name": "newFee",
                            "type": "uint16"
                        }
                    ],
                    "name": "setFee",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "address",
                            "name": "_validator",
                            "type": "address"
                        },
                        {
                            "internalType": "bool",
                            "name": "value",
                            "type": "bool"
                        }
                    ],
                    "name": "setValidator",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "symbol",
                    "outputs": [
                        {
                            "internalType": "string",
                            "name": "",
                            "type": "string"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "token0",
                    "outputs": [
                        {
                            "internalType": "address",
                            "name": "",
                            "type": "address"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "token1",
                    "outputs": [
                        {
                            "internalType": "address",
                            "name": "",
                            "type": "address"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "totalSupply",
                    "outputs": [
                        {
                            "internalType": "uint256",
                            "name": "",
                            "type": "uint256"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "address",
                            "name": "recipient",
                            "type": "address"
                        },
                        {
                            "internalType": "uint256",
                            "name": "amount",
                            "type": "uint256"
                        }
                    ],
                    "name": "transfer",
                    "outputs": [
                        {
                            "internalType": "bool",
                            "name": "",
                            "type": "bool"
                        }
                    ],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "address",
                            "name": "sender",
                            "type": "address"
                        },
                        {
                            "internalType": "address",
                            "name": "recipient",
                            "type": "address"
                        },
                        {
                            "internalType": "uint256",
                            "name": "amount",
                            "type": "uint256"
                        }
                    ],
                    "name": "transferFrom",
                    "outputs": [
                        {
                            "internalType": "bool",
                            "name": "",
                            "type": "bool"
                        }
                    ],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "address",
                            "name": "newOwner",
                            "type": "address"
                        }
                    ],
                    "name": "transferOwnership",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "internalType": "address",
                            "name": "",
                            "type": "address"
                        }
                    ],
                    "name": "validator",
                    "outputs": [
                        {
                            "internalType": "bool",
                            "name": "",
                            "type": "bool"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "stateMutability": "payable",
                    "type": "receive"
                }
            ]"#).unwrap();
            
            // connect to the network
            let client = Provider::<Http>::try_from("http://localhost:8545").unwrap();
            
            // create the contract object at the address
            let contract = Contract::new(address, abi, client);
            
            //Test that contract creation was successful by printing contract address
            let addr = contract.address();
            println!("{:?}", addr);

        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
