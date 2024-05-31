Feature: Technical Test - Public API

  Scenario: Get and Validate the server time
    Given I am about to make a request to Kraken public API
    When  I retrieve the server time
    Then  Unixtime is equal to current time
    Then  Unixtime is equal to RFC1123 time

  Scenario: Get and Validate XBTUSD traiding pair
    Given I am about to make a request to Kraken public API
    When  I retrieve the XBT-USD trading pair
    Then  Result has to expected format for XBT-USD
    And   I report the result under kraken_public_api_response.json