/* eslint-disable prettier/prettier */
import { ApolloDriver, ApolloDriverConfig } from '@nestjs/apollo';
import {Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';
import {GraphQLModule} from '@nestjs/graphql';
import * as Joi from 'joi';
import { AuthModule } from '.auth/auth.module';
import { AuthModule } from './auth/auth.module';
import { BookController } from './book/book.controller';


@Module({
    imports: [
        AuthModule,
        ConfigModule.forRoot({
            isGlobal: true,
            validationSchema: Joi.object({
                JWT_SECRET: Joi.string().required(),
                JWT_EXPIRATION: Joi.string().required(),
                MONGO_URI: Joi.string().required(),
                PORT: Joi.number().required(),
            })
        }),
        GraphQLModule.forRoot<ApolloDriverConfig>({
            driver: ApolloDriver,
            autoSchemaFile: true
        })
    ],
    controllers: [BookController],
    providers: [],
})


export class AppModule{}
