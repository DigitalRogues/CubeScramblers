faces = {"D", "L", "B", "U", "R", "F"}
modifiers = {"", "'", "2"}

function getIndex(value)
    -- this is messy as hell, it creates an inverse index so I can find the index matching of the faceString
    local reverseIndex = {}
    for k, v in pairs(moves) do reverseIndex[v] = k end
    return reverseIndex[value]
end

function chopArray(moves)
    -- the whole starting arrays at index 1 really peaks my confusion.
    choppedArray = {}
    for i, element in pairs(moves) do
        if (i % 5 == 0) then
            -- because of array start is 1 lets shift back the index so we get all 25 moves 
            index = i - 4
            moveString =
                " " .. moves[index] .. "  " .. moves[index + 1] .. "  " ..
                    moves[index + 2] .. "  " .. moves[index + 3] .. "  " ..
                    moves[index + 4]
            table.insert(choppedArray, moveString)
        end
    end
    print(table.concat(choppedArray, "\n"))
end

function randString()
    count = 0
    total = 25
    moves = {}
    math.randomseed(os.time())

    -- This is somewhat akward but lets try adding the move to the array then do the check
    -- if checks fail, remove it and pass through to start over.

    while (count < total) do
        -- set up the new move
        randFaceNum = math.random(6) -- Int(arc4random_uniform(6))
        randModNum = math.random(3) -- Int(arc4random_uniform(3))
        faceString = faces[randFaceNum]
        modString = modifiers[randModNum]
        move = faceString .. modString

        -- insert into the end of the table, and increment the count
        -- if the checks fail, we remove the move from the table and decrement the count
        table.insert(moves, move)
        -- print("prospective move: " .. move)

        skipCheck2 = false
        -- move check #1, this will run after the first pass (so staring the 2nd pass) and there is at least 1 item in the table
        -- this check is to make sure the last move is not the same as the previous move
        -- check if the last element's first character is == to the first character of the element just previous (last and second to last)
        if (count > 0 and string.sub(moves[#moves], 1, 1) ==
            string.sub(moves[#moves - 1], 1, 1)) then
            -- print("removing check1 move: " .. move)
            table.remove(moves, #moves)
            count = count - 1
            -- set skipCheck2 to true so we dont end up removing too many elements in that check
            skipCheck2 = true
        end

        if (skipCheck2 == false) then
            -- move check #2
            -- find the instance 
            -- this is to just check that the output isnt nil. in swift it returns 0 not nil
            -- ind3 = 0
            -- faceIndex = getIndex(faceString)
            -- if (faceIndex ~= nil) then ind3 = faceIndex end
            -- ind3 = ind3 + 4
            -- print(faces[ind3 % 6])

            -- make sure the move isnt the same as the former and latter moves?
            -- again check the last element with the 3rd to last to make sure they arent the same and 
            -- check the second to last element == 
            -- and string.sub(moves[#moves - 1], 1, 1) == faces[ind3 % 6])
            if (count > 1 and string.sub(moves[#moves], 1, 1) ==
                string.sub(moves[#moves - 2], 1, 1)) then
                -- print("removing check2 move: " .. move)
                table.remove(moves, #moves)
                count = count - 1
            end
        end -- ifcheck end
        count = count + 1
    end -- while count end
    return moves
end

moves = randString()
chopArray(moves)
