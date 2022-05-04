![Imersão Full Stack && Full Cycle](https://events-fullcycle.s3.amazonaws.com/events-fullcycle/static/site/img/grupo_4417.png)

![rust](https://github.com/fernandomarca/rust-codepix/blob/main/prisma/r.jpg)

## Sobre o repositório

Esse repositório contém o código do Codepix modulo da imersão FullCycle escrito em Rust para referência didáticas.

Se for útil para seus estudo Rust dê uma estrelinha.

O projeto também conta com as integrações:

- [x] Prisma-rust-client experimental.
- [x] Tonic gRPC HTTP/2.
- [x] gerador Prost Protocol Buffers proto3.
- [x] Diesel Orm.
- [x] Postgres.
- [x] Kafka/rdkafka.

## Erd do projeto

![erd](https://github.com/fernandomarca/rust-codepix/blob/main/prisma/erd.svg)

## Erd criado pelo do dev.dbml

![dbml](https://github.com/fernandomarca/rust-codepix/blob/main/prisma/dbml/bdml.png)

## Instruções de inicialização

1 - Ambos os projetos dependem do Apache-kafka para comunicação, então este é o primeiro serviço que deve ser inicializado. Para tal:
- entre no diretório do apache-kafka e execute "docker-compose up -d" pressupõe-se que tenha docker e docker-compose instalados.

2 - O segundo serviço a ser inicializado pode ser o bank-api. Para tal:
- entre no diretório bank-api e execute npm install ou yarn para instalar as dependências do projeto.
- Para teste possuímos no diretório o docker-compose_bbx.yaml isso inicializa uma instância da bank-api para simular um dos banco. Então execute "docker-compose -f docker-compose_bbx.yaml", está instância roda na porta 8001.
- E possível manter em execução 2 instâncias de bancos para testes. Então execute "docker-compose -f docker-compose_cter.yaml" em outro terminal para iniciar o segundo banco, está instância roda na porta 8002.

3 - Por último execute o servidor gRPC e o serviço do Kafka. Para executar estes serviços a aplicação rust-codepix possui uma CLI(command line interface). 
- Os serviço podem ser inicializados independentes com "cargo run -- start" para o servidor gRPC e "cargo run -- kafka" para o kafka. 
- Como precisamos de ambos os serviços em execução, o Rust nós permite que os serviço sejam executados na mesma "thread" mais de forma simultânea sem bloqueios (assíncrona), também foi configurado um comando único para isso, execute "cargo run -- all". 
- Utilizamos Tokio runtime dependência para esse gerenciamento. É uma ótima oportunidade para experimentar também o desempenho desses serviços os executando de forma assíncrona em 2 "threads independentes". Para isso altere o comando em /cmd/mod.rs linhas 88 e 92 seguindo as configurações de Tokio runtime para está finalidade.