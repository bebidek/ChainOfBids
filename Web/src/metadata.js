metadata = {
  "source": {
    "hash": "0xc6417556a6175bab3a22c5bb7390314203804371e4ad47c961825f206cb24b50",
    "language": "ink! 3.4.0",
    "compiler": "rustc 1.68.0-nightly"
  },
  "contract": {
    "name": "chain_of_bids",
    "version": "0.1.0",
    "authors": [
      "Bartłomiej Wacławik <bartek.waclawik@student.uj.edu.pl>"
    ]
  },
  "V3": {
    "spec": {
      "constructors": [
        {
          "args": [
            {
              "label": "fee_denominator",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "new",
          "payable": false,
          "selector": "0x9bae9d5e"
        }
      ],
      "docs": [],
      "events": [],
      "messages": [
        {
          "args": [],
          "docs": [],
          "label": "get_fee_denominator",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "u64"
            ],
            "type": 0
          },
          "selector": "0x4a7c1a92"
        },
        {
          "args": [],
          "docs": [],
          "label": "get_fee_balance",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Balance"
            ],
            "type": 1
          },
          "selector": "0x901fa036"
        },
        {
          "args": [],
          "docs": [],
          "label": "collect_fees",
          "mutates": true,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 13
          },
          "selector": "0x940af456"
        },
        {
          "args": [],
          "docs": [],
          "label": "get_number_of_auctions",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "u64"
            ],
            "type": 0
          },
          "selector": "0xb465a721"
        },
        {
          "args": [
            {
              "label": "name",
              "type": {
                "displayName": [
                  "String"
                ],
                "type": 7
              }
            },
            {
              "label": "description",
              "type": {
                "displayName": [
                  "String"
                ],
                "type": 7
              }
            },
            {
              "label": "amount",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            },
            {
              "label": "start_time",
              "type": {
                "displayName": [
                  "Timestamp"
                ],
                "type": 0
              }
            },
            {
              "label": "end_period_start",
              "type": {
                "displayName": [
                  "Timestamp"
                ],
                "type": 0
              }
            },
            {
              "label": "end_period_stop",
              "type": {
                "displayName": [
                  "Timestamp"
                ],
                "type": 0
              }
            },
            {
              "label": "starting_price",
              "type": {
                "displayName": [
                  "Balance"
                ],
                "type": 1
              }
            }
          ],
          "docs": [],
          "label": "create_new_auction",
          "mutates": true,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 15
          },
          "selector": "0x0e11078c"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_auction_name",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 17
          },
          "selector": "0x28338e10"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_auction_description",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 17
          },
          "selector": "0x9eb2aad1"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_auction_owner",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 19
          },
          "selector": "0xf08d67cb"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_auction_amount",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 20
          },
          "selector": "0x3425ecd4"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_auction_finalization_status",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 21
          },
          "selector": "0x97901be2"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_auction_start_time",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 20
          },
          "selector": "0xa6969e1a"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_auction_end_period_start",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 20
          },
          "selector": "0x58f167ff"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_auction_end_period_end",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 20
          },
          "selector": "0xc276fcb6"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_auction_starting_price",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 22
          },
          "selector": "0xd3a1faf1"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_auction_number_of_bids",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 20
          },
          "selector": "0xea522d56"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            },
            {
              "label": "forefront",
              "type": {
                "displayName": [
                  "ink_prelude",
                  "vec",
                  "Vec"
                ],
                "type": 23
              }
            }
          ],
          "docs": [],
          "label": "finalize_auction",
          "mutates": true,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 24
          },
          "selector": "0x28dd27b4"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            },
            {
              "label": "bid_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_bid_bidder",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 19
          },
          "selector": "0x33072ebc"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            },
            {
              "label": "bid_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_bid_price",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 22
          },
          "selector": "0xd3d8d991"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            },
            {
              "label": "bid_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "get_bid_amount",
          "mutates": false,
          "payable": false,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 20
          },
          "selector": "0xeece14c8"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            },
            {
              "label": "amount",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "make_a_bid",
          "mutates": true,
          "payable": true,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 26
          },
          "selector": "0x2fc44a04"
        },
        {
          "args": [
            {
              "label": "auction_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            },
            {
              "label": "bid_id",
              "type": {
                "displayName": [
                  "u64"
                ],
                "type": 0
              }
            }
          ],
          "docs": [],
          "label": "increase_the_bid",
          "mutates": true,
          "payable": true,
          "returnType": {
            "displayName": [
              "Result"
            ],
            "type": 28
          },
          "selector": "0xa33f41f1"
        }
      ]
    },
    "storage": {
      "struct": {
        "fields": [
          {
            "layout": {
              "cell": {
                "key": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "ty": 0
              }
            },
            "name": "fee_denominator"
          },
          {
            "layout": {
              "cell": {
                "key": "0x0100000000000000000000000000000000000000000000000000000000000000",
                "ty": 1
              }
            },
            "name": "fee_balance"
          },
          {
            "layout": {
              "cell": {
                "key": "0x0200000000000000000000000000000000000000000000000000000000000000",
                "ty": 2
              }
            },
            "name": "owner"
          },
          {
            "layout": {
              "cell": {
                "key": "0x0300000000000000000000000000000000000000000000000000000000000000",
                "ty": 0
              }
            },
            "name": "number_of_auctions"
          },
          {
            "layout": {
              "cell": {
                "key": "0x0400000000000000000000000000000000000000000000000000000000000000",
                "ty": 5
              }
            },
            "name": "auctions"
          },
          {
            "layout": {
              "cell": {
                "key": "0x0500000000000000000000000000000000000000000000000000000000000000",
                "ty": 10
              }
            },
            "name": "bids"
          }
        ]
      }
    },
    "types": [
      {
        "id": 0,
        "type": {
          "def": {
            "primitive": "u64"
          }
        }
      },
      {
        "id": 1,
        "type": {
          "def": {
            "primitive": "u128"
          }
        }
      },
      {
        "id": 2,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "type": 3,
                  "typeName": "[u8; 32]"
                }
              ]
            }
          },
          "path": [
            "ink_env",
            "types",
            "AccountId"
          ]
        }
      },
      {
        "id": 3,
        "type": {
          "def": {
            "array": {
              "len": 32,
              "type": 4
            }
          }
        }
      },
      {
        "id": 4,
        "type": {
          "def": {
            "primitive": "u8"
          }
        }
      },
      {
        "id": 5,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "name": "offset_key",
                  "type": 9,
                  "typeName": "Key"
                }
              ]
            }
          },
          "params": [
            {
              "name": "K",
              "type": 0
            },
            {
              "name": "V",
              "type": 6
            }
          ],
          "path": [
            "ink_storage",
            "lazy",
            "mapping",
            "Mapping"
          ]
        }
      },
      {
        "id": 6,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "name": "name",
                  "type": 7,
                  "typeName": "String"
                },
                {
                  "name": "description",
                  "type": 7,
                  "typeName": "String"
                },
                {
                  "name": "owner",
                  "type": 2,
                  "typeName": "AccountId"
                },
                {
                  "name": "amount",
                  "type": 0,
                  "typeName": "u64"
                },
                {
                  "name": "finalized",
                  "type": 8,
                  "typeName": "bool"
                },
                {
                  "name": "start_time",
                  "type": 0,
                  "typeName": "Timestamp"
                },
                {
                  "name": "end_period_start",
                  "type": 0,
                  "typeName": "Timestamp"
                },
                {
                  "name": "end_period_stop",
                  "type": 0,
                  "typeName": "Timestamp"
                },
                {
                  "name": "starting_price",
                  "type": 1,
                  "typeName": "Balance"
                },
                {
                  "name": "number_of_bids",
                  "type": 0,
                  "typeName": "u64"
                }
              ]
            }
          },
          "path": [
            "chain_of_bids",
            "chain_of_bids",
            "auction",
            "Auction"
          ]
        }
      },
      {
        "id": 7,
        "type": {
          "def": {
            "primitive": "str"
          }
        }
      },
      {
        "id": 8,
        "type": {
          "def": {
            "primitive": "bool"
          }
        }
      },
      {
        "id": 9,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "type": 3,
                  "typeName": "[u8; 32]"
                }
              ]
            }
          },
          "path": [
            "ink_primitives",
            "Key"
          ]
        }
      },
      {
        "id": 10,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "name": "offset_key",
                  "type": 9,
                  "typeName": "Key"
                }
              ]
            }
          },
          "params": [
            {
              "name": "K",
              "type": 11
            },
            {
              "name": "V",
              "type": 12
            }
          ],
          "path": [
            "ink_storage",
            "lazy",
            "mapping",
            "Mapping"
          ]
        }
      },
      {
        "id": 11,
        "type": {
          "def": {
            "tuple": [
              0,
              0
            ]
          }
        }
      },
      {
        "id": 12,
        "type": {
          "def": {
            "composite": {
              "fields": [
                {
                  "name": "bidder",
                  "type": 2,
                  "typeName": "AccountId"
                },
                {
                  "name": "price",
                  "type": 1,
                  "typeName": "Balance"
                },
                {
                  "name": "amount",
                  "type": 0,
                  "typeName": "u64"
                }
              ]
            }
          },
          "path": [
            "chain_of_bids",
            "chain_of_bids",
            "auction",
            "bidding",
            "Bid"
          ]
        }
      },
      {
        "id": 13,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "fields": [
                    {
                      "type": 14
                    }
                  ],
                  "index": 0,
                  "name": "Ok"
                },
                {
                  "fields": [
                    {
                      "type": 14
                    }
                  ],
                  "index": 1,
                  "name": "Err"
                }
              ]
            }
          },
          "params": [
            {
              "name": "T",
              "type": 14
            },
            {
              "name": "E",
              "type": 14
            }
          ],
          "path": [
            "Result"
          ]
        }
      },
      {
        "id": 14,
        "type": {
          "def": {
            "tuple": []
          }
        }
      },
      {
        "id": 15,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "fields": [
                    {
                      "type": 0
                    }
                  ],
                  "index": 0,
                  "name": "Ok"
                },
                {
                  "fields": [
                    {
                      "type": 16
                    }
                  ],
                  "index": 1,
                  "name": "Err"
                }
              ]
            }
          },
          "params": [
            {
              "name": "T",
              "type": 0
            },
            {
              "name": "E",
              "type": 16
            }
          ],
          "path": [
            "Result"
          ]
        }
      },
      {
        "id": 16,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "index": 0,
                  "name": "InvalidNameLength"
                },
                {
                  "index": 1,
                  "name": "InvalidDescriptionLength"
                },
                {
                  "index": 2,
                  "name": "InvalidEndPeriod"
                }
              ]
            }
          },
          "path": [
            "chain_of_bids",
            "chain_of_bids",
            "auction",
            "AuctionCreationError"
          ]
        }
      },
      {
        "id": 17,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "fields": [
                    {
                      "type": 7
                    }
                  ],
                  "index": 0,
                  "name": "Ok"
                },
                {
                  "fields": [
                    {
                      "type": 18
                    }
                  ],
                  "index": 1,
                  "name": "Err"
                }
              ]
            }
          },
          "params": [
            {
              "name": "T",
              "type": 7
            },
            {
              "name": "E",
              "type": 18
            }
          ],
          "path": [
            "Result"
          ]
        }
      },
      {
        "id": 18,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "index": 0,
                  "name": "InvalidAuctionId"
                },
                {
                  "index": 1,
                  "name": "InvalidAuctionOrBidId"
                }
              ]
            }
          },
          "path": [
            "chain_of_bids",
            "chain_of_bids",
            "QueryError"
          ]
        }
      },
      {
        "id": 19,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "fields": [
                    {
                      "type": 2
                    }
                  ],
                  "index": 0,
                  "name": "Ok"
                },
                {
                  "fields": [
                    {
                      "type": 18
                    }
                  ],
                  "index": 1,
                  "name": "Err"
                }
              ]
            }
          },
          "params": [
            {
              "name": "T",
              "type": 2
            },
            {
              "name": "E",
              "type": 18
            }
          ],
          "path": [
            "Result"
          ]
        }
      },
      {
        "id": 20,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "fields": [
                    {
                      "type": 0
                    }
                  ],
                  "index": 0,
                  "name": "Ok"
                },
                {
                  "fields": [
                    {
                      "type": 18
                    }
                  ],
                  "index": 1,
                  "name": "Err"
                }
              ]
            }
          },
          "params": [
            {
              "name": "T",
              "type": 0
            },
            {
              "name": "E",
              "type": 18
            }
          ],
          "path": [
            "Result"
          ]
        }
      },
      {
        "id": 21,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "fields": [
                    {
                      "type": 8
                    }
                  ],
                  "index": 0,
                  "name": "Ok"
                },
                {
                  "fields": [
                    {
                      "type": 18
                    }
                  ],
                  "index": 1,
                  "name": "Err"
                }
              ]
            }
          },
          "params": [
            {
              "name": "T",
              "type": 8
            },
            {
              "name": "E",
              "type": 18
            }
          ],
          "path": [
            "Result"
          ]
        }
      },
      {
        "id": 22,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "fields": [
                    {
                      "type": 1
                    }
                  ],
                  "index": 0,
                  "name": "Ok"
                },
                {
                  "fields": [
                    {
                      "type": 18
                    }
                  ],
                  "index": 1,
                  "name": "Err"
                }
              ]
            }
          },
          "params": [
            {
              "name": "T",
              "type": 1
            },
            {
              "name": "E",
              "type": 18
            }
          ],
          "path": [
            "Result"
          ]
        }
      },
      {
        "id": 23,
        "type": {
          "def": {
            "sequence": {
              "type": 0
            }
          }
        }
      },
      {
        "id": 24,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "fields": [
                    {
                      "type": 14
                    }
                  ],
                  "index": 0,
                  "name": "Ok"
                },
                {
                  "fields": [
                    {
                      "type": 25
                    }
                  ],
                  "index": 1,
                  "name": "Err"
                }
              ]
            }
          },
          "params": [
            {
              "name": "T",
              "type": 14
            },
            {
              "name": "E",
              "type": 25
            }
          ],
          "path": [
            "Result"
          ]
        }
      },
      {
        "id": 25,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "index": 0,
                  "name": "DummyError"
                },
                {
                  "index": 1,
                  "name": "InvalidAuctionId"
                },
                {
                  "index": 2,
                  "name": "AuctionIsAlreadyFinalized"
                },
                {
                  "index": 3,
                  "name": "TooEarlyToFinish"
                },
                {
                  "index": 4,
                  "name": "CallerIsNotOwner"
                },
                {
                  "index": 5,
                  "name": "InvalidForefrontVector"
                }
              ]
            }
          },
          "path": [
            "chain_of_bids",
            "chain_of_bids",
            "auction",
            "AuctionFinalizationError"
          ]
        }
      },
      {
        "id": 26,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "fields": [
                    {
                      "type": 0
                    }
                  ],
                  "index": 0,
                  "name": "Ok"
                },
                {
                  "fields": [
                    {
                      "type": 27
                    }
                  ],
                  "index": 1,
                  "name": "Err"
                }
              ]
            }
          },
          "params": [
            {
              "name": "T",
              "type": 0
            },
            {
              "name": "E",
              "type": 27
            }
          ],
          "path": [
            "Result"
          ]
        }
      },
      {
        "id": 27,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "index": 0,
                  "name": "InvalidAuctionId"
                },
                {
                  "index": 1,
                  "name": "InvalidBidId"
                },
                {
                  "index": 2,
                  "name": "AuctionIsNotActiveNow"
                },
                {
                  "index": 3,
                  "name": "CallerIsNotOriginalBidder"
                },
                {
                  "index": 4,
                  "name": "BidBelowStartingPrice"
                },
                {
                  "index": 5,
                  "name": "ZeroOrTooMany"
                },
                {
                  "index": 6,
                  "name": "FractionalUnitPrice"
                }
              ]
            }
          },
          "path": [
            "chain_of_bids",
            "chain_of_bids",
            "auction",
            "bidding",
            "BiddingError"
          ]
        }
      },
      {
        "id": 28,
        "type": {
          "def": {
            "variant": {
              "variants": [
                {
                  "fields": [
                    {
                      "type": 14
                    }
                  ],
                  "index": 0,
                  "name": "Ok"
                },
                {
                  "fields": [
                    {
                      "type": 27
                    }
                  ],
                  "index": 1,
                  "name": "Err"
                }
              ]
            }
          },
          "params": [
            {
              "name": "T",
              "type": 14
            },
            {
              "name": "E",
              "type": 27
            }
          ],
          "path": [
            "Result"
          ]
        }
      }
    ]
  }
}