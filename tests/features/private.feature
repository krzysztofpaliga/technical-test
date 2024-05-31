Feature: Technical Test - Private API

  Scenario: Get and Report open orders
    Given I am about to make a request to Kraken private API
    When  I retrieve open orders
    Then  I assert no error in response
    Then  I report the result under kraken_private_api_response.json
