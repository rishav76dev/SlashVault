/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/basic.json`.
 */
export type Basic = {
  "address": "JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H",
  "metadata": {
    "name": "basic",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "company",
          "writable": true,
          "signer": true
        },
        {
          "name": "contractor"
        },
        {
          "name": "contract",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  116,
                  114,
                  97,
                  99,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "company"
              },
              {
                "kind": "account",
                "path": "contractor"
              }
            ]
          }
        },
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "contract"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "budget",
          "type": "u64"
        },
        {
          "name": "startTime",
          "type": "i64"
        },
        {
          "name": "penaltyTime",
          "type": "i64"
        },
        {
          "name": "terminateTime",
          "type": "i64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "contract",
      "discriminator": [
        172,
        138,
        115,
        242,
        121,
        67,
        183,
        26
      ]
    }
  ],
  "types": [
    {
      "name": "contract",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "company",
            "type": "pubkey"
          },
          {
            "name": "contractor",
            "type": "pubkey"
          },
          {
            "name": "budget",
            "type": "u64"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "penaltyTime",
            "type": "i64"
          },
          {
            "name": "terminateTime",
            "type": "i64"
          }
        ]
      }
    }
  ]
};
