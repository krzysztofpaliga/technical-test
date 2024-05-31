Feature: Technical Test - Private API

  Scenario: Get and Report open orders
    Given I am about to make a request to Kraken Private API
    When  I retrieve open orders
    Then  I assert no error in response
    Then  I report the result
