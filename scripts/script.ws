#!/usr/bin/env wappen-script

( # loop.ws )


( n = 0 )


( @condition = "
    ( ( $ n ) < 5 )
" )
( @body = "
    ( n = ( ( $ n ) + 1 ) )
" )
( @ while )


( @n = 10 )
( @body = "
    ( n = ( ( $ n ) + 1 ) )
" )
( @ for )

( pid = ( ~ 39 ) )

{
    { "We executed a few loops a total of" ( $ n ) "times" }
    { "And our PID is" ( $ pid ) "in binary" ( -> ( $ pid ) ) "with ptr" ( -> ( -> ( $ pid ) ) ) }
}
