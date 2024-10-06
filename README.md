# Solana Vista

Solana Vista is a powerful, flexible, and extensible indexer for the Solana blockchain, designed to provide developers with seamless access to real-time and historical Solana data.

## Features

- **Real-time Indexing**: Ingest and process Solana accounts and programs data in real-time.
- **Multi-Source Ingestion**: Support for multiple data ingestion methods (Geyser, WebSocket, gRPC, HTTP).
- **Flexible Provider System**: Easy integration with various RPC providers (QuickNode, Helius, Triton, Syndica, etc.).
- **Extensible Storage**: Plugin-based storage backend supporting PostgreSQL, MongoDB, Cassandra, and custom solutions.
- **GraphQL API**: Powerful API for querying indexed data with real-time subscriptions.
- **Historical Data Indexing**: Support for backfilling and frontfilling with BigTable, ClickHouse, or Filecoin integration.
- **Account Tracking**: Advanced account diff tracking and program account listeners.
- **Anchor IDL Support**: Compatibility with Anchor IDLs (both major versions) for automatic schema generation.
- **Plugin System**: Extensible architecture for custom transformations and indexing logic.
- **Local Development**: Integration with the Luzid local validator for testing and development.
- **Cloud-Ready Deployment**: Easy deployment to major cloud providers with horizontal scaling configurations.
- **Performance Optimization**: Built-in tools for benchmarking and optimizing indexer performance.
- **Flexible Configuration**: JSON-based configuration for easy setup and customization.

## Quick Start

1. Clone the repository:

   ```bash
   git clone https://github.com/0xCipherCoder/solana-vista.git
   cd solana-vista
   ```

2. Run the setup script:

   ```bash
   ./scripts/setup.sh
   ```

3. Configure Vista:
   Edit `config.yml` with your desired settings, including accounts and programs to index.

4. Start the indexer:

   ```bash
   cargo run
   ```

## Usage

After starting Solana Vista, interact with it via the GraphQL API:

```bash
http://localhost:8080/graphql
```

For real-time updates, use the GraphQL subscriptions endpoint:

```bash
ws://localhost:8080/graphql
```

## Configuration

Solana Vista can be configured to index specific accounts and programs. Edit the `config.yml` file to specify:

- Accounts and programs to index
- Ingestion sources and providers
- Storage backend configuration
- API settings

## Examples

Check out the `examples/` directory for sample implementations, including:

- Basic indexer usage
- Custom storage plugin implementation
- Multi-provider setup

## Extending Vista

Vista is designed to be easily extensible:

1. **Custom Plugins**: Create new plugins in the `plugins/` directory.
2. **New Storage Backends**: Implement the `StoragePlugin` trait in `vista-storage`.
3. **Additional Ingestion Methods**: Add new providers in `vista-ingestion`.

Refer to the `CONTRIBUTING.md` file for more details on how to contribute.

## Deployment

Vista supports various deployment options:

- Local deployment with Docker: `docker-compose up`
- Cloud deployment: Use scripts in `vista-deploy` for AWS, GCP, or Azure deployment

## Performance

Benchmarks for different configurations can be found in the `benches/` directory. Run them with:

```bash
cargo bench
```

## Documentation

For full documentation, visit [docs.solanavista.io](https://docs.solanavista.io)(WIP).

## Roadmap

Our development roadmap outlines the planned implementation of features for Solana Vista. This roadmap is subject to change based on community feedback and ecosystem developments.

### Phase 1: MVP (Current)

- [x] Core indexing engine implementation
- [ ] Basic real-time indexing of specified Solana accounts and programs
- [ ] Implement Geyser and HTTP RPC providers
- [ ] Simple GraphQL API for data queries
- [ ] PostgreSQL storage backend
- [ ] Docker container for easy deployment

### Phase 2: Enhanced Functionality

- [ ] WebSocket and gRPC provider implementation
- [ ] Historical data indexing with backfilling support
- [ ] Account diff tracking and program listeners
- [ ] Expanded GraphQL API with real-time subscriptions
- [ ] Basic Anchor IDL support
- [ ] Plugin system for storage backends

### Phase 3: Advanced Features and Scaling

- [ ] Support for multiple RPC providers (QuickNode, Helius, Triton, Syndica)
- [ ] Advanced Anchor IDL support with automatic schema generation
- [ ] Custom transformation plugins
- [ ] Integration with BigTable, ClickHouse for historical data
- [ ] Horizontal scaling configurations
- [ ] Performance optimizations and benchmarking tools

### Phase 4: Ecosystem Integration and Community Growth

- [ ] Comprehensive documentation and tutorials
- [ ] Integration with Luzid local validator
- [ ] Advanced analytics and data export features
- [ ] Advanced cloud deployment options
- [ ] Community plugin marketplace
- [ ] Regular security audits and optimizations
- [ ] Continuous performance improvements based on community feedback

We're committed to the ongoing development and improvement of Solana Vista. This roadmap will be regularly updated to reflect our progress and any changes in priorities based on community needs and ecosystem developments.

Your feedback and contributions are crucial in shaping the future of Solana Vista. Feel free to open issues or submit pull requests on our [GitHub repository](https://github.com/0xCipherCoder/solana-vista) to suggest features or report bugs.

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md)(WIP). for more details.

## License

Vista is licensed under [MIT license](LICENSE-MIT).

## Community

- Twitter: [@SolanaVista](https://twitter.com/SolanaVista)
- Discord: [Join our server](https://discord.gg/solanavista)
- Website: [solanavista.io](https://solanavista.io)
- GitHub: [https://github.com/0xCipherCoder/solana-vista](https://github.com/0xCipherCoder/solana-vista)

## Acknowledgements

Vista is built on the shoulders of giants. We'd like to thank the Solana community, Jito Labs, and all the open-source projects that made this possible.
