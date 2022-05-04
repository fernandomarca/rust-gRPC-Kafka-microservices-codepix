import { Console, Command } from 'nestjs-console';
import { getConnection } from 'typeorm';
import * as chalk from 'chalk';
@Console()
export class FixturesCommand {
  @Command({
    command: 'fixtures',
    description: 'seed data in database',
  })
  async command() {
    await this.runMigrations();
    const fixtures = (await import(`./fixtures/bank-${process.env.BANK_CODE}`))
      .default;
    for (const fixture of fixtures) {
      await this.createData(fixture.model, fixture.fields);
    }
    console.log(chalk.green('Data generated'));
  }

  async runMigrations() {
    const conn = getConnection('default');
    for (const migration of conn.migrations.reverse()) {
      await conn.undoLastMigration();
    }
  }

  async createData(model: any, data: any) {
    const repository = this.getRepository(model);
    const obj = repository.create(data);
    await repository.save(obj);
  }

  getRepository(model: any) {
    const conn = getConnection('default');
    return conn.getRepository(model);
  }
}
