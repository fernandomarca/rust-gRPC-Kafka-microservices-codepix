import { IsIn, IsNotEmpty, IsString } from 'class-validator';
import { PixKeyKind } from 'src/models/pix-key.model';

export class PixkeyDto {
  @IsString()
  @IsNotEmpty()
  readonly key: string;

  @IsString()
  @IsIn(Object.values(PixKeyKind))
  @IsNotEmpty()
  readonly kind: PixKeyKind;
}
