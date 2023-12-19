# Car Rental System Canister

This repository contains the source code for a car rental system canister on the Internet Computer (IC). The canister allows users to manage cars, customers, reservations, and generate reports for a car rental service. The system is designed to handle operations like adding cars, updating car information, making reservations, and generating reports.

## Functionality

The car rental system canister provides the following functionality:

### Car Management

- **Add Car (`add_car`):** Add a new car to the system.
- **Update Car (`update_car`):** Update information about an existing car.
- **Get Car (`get_car`):** Retrieve information about a specific car.
- **Is Booked (`is_booked`):** Check if a car is currently booked.
- **Delete Car (`delete_car`):** Delete a car from the system.

### Customer Management

- **Add Customer (`add_customer`):** Add a new customer to the system.
- **Get Customer (`get_customer`):** Retrieve information about a specific customer.
- **Delete Customer (`delete_customer`):** Delete a customer from the system.

### Reservation Management

- **Make Reservation (`make_reservation`):** Make a reservation for a car by a customer.
- **Get Reservation (`get_reservation`):** Retrieve information about a reservation.
- **Cancel Reservation (`cancel_reservation`):** Cancel a reservation for a car.

### Reporting

- **Generate Report (`generate_report`):** Generate a report with information about all cars in the system.

## Deployment on Local Machine

To deploy the canister locally, follow these steps:

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/sammy-dev/car-management-system-canister.git
   cd car-management-system-canister
   ```

2. **Build the Canister:**
   ```bash
   dfx build
   ```

3. **Deploy the Canister:**
   ```bash
   dfx deploy
   ```

4. **Use the Generated Canister Identifier:**
   The deployment process will provide you with a canister identifier. Use this identifier to interact with the deployed canister.

For additional deployment options and configurations, refer to the [Internet Computer SDK documentation](https://sdk.dfinity.org/docs/quickstart/local-quickstart.html).

## Testing

To run tests, use the following command:

```bash
cargo test
```

## Contributing

Feel free to contribute to the project by submitting issues or pull requests. Follow the standard GitHub flow for contributing.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```

You can save this content in a file with a `.md` extension, for example, `README.md`.
