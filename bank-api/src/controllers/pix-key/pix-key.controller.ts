import {
  Body,
  Controller,
  Get,
  HttpCode,
  Inject,
  InternalServerErrorException,
  Param,
  ParseUUIDPipe,
  Post,
  Query,
  UnprocessableEntityException,
  ValidationPipe,
} from '@nestjs/common';
import { ClientGrpc } from '@nestjs/microservices';
import { InjectRepository } from '@nestjs/typeorm';
import { lastValueFrom } from 'rxjs';
import { PixkeyExistsDto } from 'src/dto/pix-key-exists.dto';
import { PixkeyDto } from 'src/dto/pix-key.dto';
import { PixService } from 'src/grpc-types/pix-service.grpc';
import { BankAccount } from 'src/models/bank-account.model';
import { PixKey } from 'src/models/pix-key.model';
import { Repository } from 'typeorm';

@Controller('bank-accounts/:bankAccountId/pix-keys')
export class PixKeyController {
  constructor(
    @InjectRepository(PixKey)
    private pixkeyRepo: Repository<PixKey>,
    @InjectRepository(BankAccount)
    private bankAccountRepo: Repository<BankAccount>,
    @Inject('CODEPIX_PACKAGE')
    private client: ClientGrpc,
  ) {}

  @Get()
  index(
    @Param('bankAccountId', new ParseUUIDPipe({ version: '4' }))
    bankAccountId: string,
  ) {
    return this.pixkeyRepo.find({
      where: {
        bank_account_id: bankAccountId,
      },
      order: {
        created_at: 'DESC',
      },
    });
  }

  @Post()
  async store(
    @Param('bankAccountId', new ParseUUIDPipe({ version: '4' }))
    bankAccountId: string,
    @Body(new ValidationPipe({ errorHttpStatusCode: 422 }))
    body: PixkeyDto,
  ) {
    await this.bankAccountRepo.findOneOrFail(bankAccountId);
    //
    const pixService: PixService = this.client.getService('PixService');
    //
    const exists = await this.exists(body);
    if (exists) {
      throw new UnprocessableEntityException('Pixkey already exists');
    }
    const createPixkey = await lastValueFrom(
      pixService.RegisterPixKey({
        ...body,
        accountId: bankAccountId,
      }),
    );

    if (createPixkey.error) {
      throw new UnprocessableEntityException(createPixkey.error);
    }

    const pixkey = this.pixkeyRepo.create({
      id: createPixkey.id,
      bank_account_id: bankAccountId,
      ...body,
    });

    return await this.pixkeyRepo.save(pixkey);
  }

  // async checkPixkeyNotFound(params: { kind: string; key: string }) {
  //   const pixService: PixService = this.client.getService('PixService');
  //   try {
  //     await lastValueFrom(
  //       pixService.find({ kind: params.kind, key: params.key }),
  //     );
  //     return false;
  //   } catch (error) {
  //     if (
  //       error.details ===
  //       'Record not found: code: 404, message: Record not found'
  //     ) {
  //       return true;
  //     }
  //     throw new InternalServerErrorException('server not available');
  //   }
  // }

  @Get('exists')
  @HttpCode(204)
  async exists(
    @Query(new ValidationPipe({ errorHttpStatusCode: 422 }))
    body: PixkeyExistsDto,
  ) {
    const pixService: PixService = this.client.getService('PixService');
    try {
      await lastValueFrom(pixService.find({ kind: body.kind, key: body.key }));
      return true;
    } catch (error) {
      if (
        error.details ===
        'Record not found: code: 404, message: Record not found'
      ) {
        return false;
      }
      throw new InternalServerErrorException('Server not available');
    }
  }
}
