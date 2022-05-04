import { Observable } from 'rxjs';

interface Account {
  accountId: string;
  accountNumber: string;
  bankId: string;
  bankName: string;
  ownerName: string;
  createdAt: string;
}

export interface PixKeyResponse {
  id: string;
  kind: string;
  key: string;
  created_at: string;
  updated_at: string;
  account_id: string;
  status: string;
}

export interface PixService {
  RegisterPixKey: (data: {
    kind: string;
    key: string;
    accountId: string;
  }) => Observable<{ id: string; status: string; error: string }>;
  find: (data: { kind: string; key: string }) => Observable<PixKeyResponse>;
}
