-- Your SQL goes here
INSERT INTO bank (id,code,name) 
VALUES ('ccbdb76c-909f-4735-b6bd-a2de0c8d07c6','002','BANCO BANESCO XCHANGE2');

INSERT INTO bank (id,code,name)
VALUES ('f434fed7-dedc-4658-b547-d8d036b97ca5','001','BANCO BANESCO XCHANGE1');

--accounts

INSERT INTO account (id,owner_name,number,bank_id) 
VALUES ('51a720b2-5144-4d7f-921d-57023b1e24c1','User BBX 2','2222','ccbdb76c-909f-4735-b6bd-a2de0c8d07c6');

INSERT INTO account (id,owner_name,number,bank_id) 
VALUES ('6e4635ce-88d1-4e58-9597-d13fc446ee47','User BBX 1','1111','ccbdb76c-909f-4735-b6bd-a2de0c8d07c6');

INSERT INTO account (id,owner_name,number,bank_id) 
VALUES ('103cc632-78e7-4476-ab63-d5ad3a562d26','User CTER 1','3333','f434fed7-dedc-4658-b547-d8d036b97ca5');

INSERT INTO account (id,owner_name,number,bank_id) 
VALUES ('463b1b2a-b5fa-4b88-9c31-e5c894a20ae3','User CTER 2','4444','f434fed7-dedc-4658-b547-d8d036b97ca5');