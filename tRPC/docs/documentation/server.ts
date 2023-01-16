// @filename: server.ts
import { initTRPC } from '@trpc/server';
import { z } from 'zod';

const t = initTRPC.create();

const UserSchema = z.object({
    id: z.string(),
    name: z.string(),
});
type User = z.infer<typeof UserSchema>;

const userList: User[] = [
    {
        id: '1',
        name: 'KATT',
    },
];

const appRouter = t.router({
    userById: t.procedure
        // The input is unknown at this time.
        // A client could have sent us anything
        // so we won't assume a certain data type.
        .input((val: unknown) => {
            // If the value is of type string, return it.
            // TypeScript now knows that this value is a string.
            if (typeof val === 'string') return val;

            // Uh oh, looks like that input wasn't a string.
            // We will throw an error instead of running the procedure.
            throw new Error(`Invalid input: ${typeof val}`);
        })
        .query((req) => {
            const { input } = req;
            const user = userList.find((u) => u.id === input);

            return user;
        }),
});

export type AppRouter = typeof appRouter;
