# Scuttle - A CLI tool to build C4 model diagrams from your living codebase

```sh
      *******                                                 ***                        * ***         ***** *               *****  *
    *       ***                               *         *      ***                     *  ****  *   ******  *             ******  *
   *         **                              **        **       **                    *  *  ****   **   *  *             **   *  *
   **        *                               **        **       **                   *  **   **   *    *  *             *    *  *
    ***                     **   ****      ********  ********   **                  *  ***            *  *                  *  *
   ** ***           ****     **    ***  * ********  ********    **       ***       **   **           ** **                 ** **
    *** ***        * ***  *  **     ****     **        **       **      * ***      **   **           ** **                 ** **
      *** ***     *   ****   **      **      **        **       **     *   ***     **   **           ** **               **** **
        *** ***  **          **      **      **        **       **    **    ***    **   **           ** **              * *** **
          ** *** **          **      **      **        **       **    ********     **   **           ** **                 ** **
           ** ** **          **      **      **        **       **    *******       **  **           *  **            **   ** **
            * *  **          **      **      **        **       **    **             ** *      *        *            ***   *  *
  ***        *   ***     *    ******* **     **        **       **    ****    *       ***     *     ****           *  ***    *
 *  *********     *******      *****   **     **        **      *** *  *******         *******     *  *************    ******
*     *****        *****                                         ***    *****            ***      *     *********        ***
*                                                                                                 *
 **                                                                                                **
```

## What is this CLI tool trying to solve?

The C4 model for system diagrams is really great. But adoption is a little tough
if the code base is being explained after the fact. For instance, Agile
practices are interpreted very literally and there's no up-front diagrams. So
now you have a codebase with very little system diagrams. This tool is meant to
consume active codebases and retrieve the system using the C4 model to organize
and spit out very simple Structurizr DSL files which can then be maintained
after creation to bootstrap the process of mass adoption of C4 model system
diagrams and the Structurizr services such as SaaS and CLI tooling.
