problem: Express multi‑clause diplomatic deals in JSON; validate strictly; attach EV/risk for both sides; never allow illegal or dominated deals.
perf_budget: Negotiator path ≤ 80ms/offer with caching; schema validation ≤ 2ms.

schema (conceptual)

Top‑level keys: give, take, threat?, duration?, conditions?, meta?

Primitive clauses: gold, open_borders, resource_license{tech, turns}, research_agreement{field, turns}, casus_belli{reason}, sanction{scope, turns}, ceasefire{turns}

Constraints:

All durations positive and ≤ 30 (v0).

Licensing requires tech ownership; EV must be ≥ 0 for licensee.

No overlapping obligations that violate existing treaties.

acceptance_tests

10k random generated deals → 0 schema or legality violations.

EV/rationality filter rejects dominated offers.

machine‑readable (JSON Schema draft)
```
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "schemas/deal.schema.json",
  "title": "Deal",
  "type": "object",
  "additionalProperties": false,
  "required": ["give","take"],
  "properties": {
    "give": { "$ref": "#/$defs/Clauses" },
    "take": { "$ref": "#/$defs/Clauses" },
    "threat": { "type": "object", "properties": { "casus_belli": { "type": "string" } }, "additionalProperties": false },
    "duration": { "type": "integer", "minimum": 1, "maximum": 30 },
    "conditions": { "type": "array", "items": { "type": "string" } },
    "meta": { "type": "object", "additionalProperties": true }
  },
  "$defs": {
    "Clauses": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "gold": { "type": "integer", "minimum": 0 },
        "open_borders": { "type": "boolean" },
        "iron_license": {
          "type": "object",
          "required": ["tech","turns"],
          "properties": {
            "tech": { "type": "string" },
            "turns": { "type": "integer", "minimum": 1, "maximum": 30 }
          },
          "additionalProperties": false
        },
        "research_agreement": {
          "type": "object",
          "required": ["field","turns"],
          "properties": {
            "field": { "enum": ["Military","Industry","Civics","Science"] },
            "turns": { "type": "integer", "minimum": 1, "maximum": 20 }
          },
          "additionalProperties": false
        }
      }
    }
  }
}
```