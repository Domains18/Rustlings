/* eslint-disable prettier/prettier */

import { Prop, Schema, SchemaFactory } from '@nestjs/mongoose'
import { AbstractDocument } from 'src/database/abstract.schema' 

@Schema({ versionKey: false })
export class UserDocument extends AbstractDocument {
    @Prop({ required: true })
    email: string

    @Prop({ required: true }) 
    password: string
}

export const UserSchema = SchemaFactory.createForClass(UserDocument)
