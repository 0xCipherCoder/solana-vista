# Solana Vista

Solana Vista is a powerful, flexible, and extensible indexer for the Solana blockchain, designed to provide developers with seamless access to real-time and historical Solana data.

## Features

- **Comprehensive Indexing**: Real-time and historical indexing of Solana accounts and programs.
- **Multi-Source Ingestion**: Support for multiple data ingestion methods (Geyser, RPC, Jito Shredstream).
- **Account Tracking**: Advanced account diff tracking and program account listeners.
- **Transaction Indexing**: Efficient transaction indexing for quick querying.
- **Anchor IDL Support**: Compatibility with Anchor IDLs (both major versions).
- **Flexible Storage**: Pluggable storage backends (PostgreSQL, MySQL, Supabase, InfluxDB for time-series data).
- **API Access**: GraphQL and REST API for querying indexed data with real-time subscriptions.
- **Local Development**: Integration with the Luzid local validator for testing and development.
- **Cloud-Ready**: Easy deployment to major cloud providers with horizontal scaling configurations.
- **Extensibility**: Plugin system for community contributions and custom functionality.
- **Containerization**: Docker support for straightforward deployment and scaling.

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

After starting Vista, interact with it via the GraphQL API:

```bash
http://localhost:8080/graphql
```

For real-time updates, use the GraphQL subscriptions endpoint:

```bash
ws://localhost:8080/graphql
```

## Configuration

Vista can be configured to index specific accounts and programs. Edit the `config.yml` file to specify:

- Accounts to watch
- Programs to index (including Anchor programs with IDL files)
- Ingestion sources (Geyser, RPC, Jito Shredstream)
- Storage backend configuration
- API settings

## Examples

Check out the `examples/` directory for sample implementations, including:

- Basic indexer usage
- Jito Shredstream integration
- Supabase real-time API
- Custom plugin usage
- Serum DEX indexing

## Extending Vista

Vista is designed to be easily extensible:

1. **Custom Plugins**: Create new plugins in the `plugins/` directory.
2. **New Storage Backends**: Implement the `Storage` trait in `vista-storage`.
3. **Additional Ingestion Methods**: Add new ingestion sources in `vista-ingestion`.

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
