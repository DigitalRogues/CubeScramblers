// struct ScrambleData {
//    var text = "Hello, World!"
// }

//
//  main.swift
//  Scrambler
//
//  Created by punk on 9/1/18.
//  Copyright © 2018 DigitalRogues. All rights reserved.
//  Original scramble source https://github.com/DigitalRogues/rubiks-cube-scrambler/blob/gh-pages/index.html

import Foundation

public class ScrambleData {
    static let faces: [String] = ["D", "L", "B", "U", "R", "F"]
    static let modifiers: [String] = ["", "'", "2"]

    public init() {}

    public func randString() -> [String] {
        var count = 0
        let total = 25
        var moves: [String] = []

        while count < total {
        
            let randFaceNum = Int(arc4random_uniform(6))
            let randModNum = Int(arc4random_uniform(3))
            let faceString = ScrambleData.faces[randFaceNum]
            let modString = ScrambleData.modifiers[randModNum]

            let move = "\(faceString)\(modString)"
            
            // Avoid having the same move twice in a row.
            if count > 0, move.first == moves[count - 1].first {
                //                print("++++")
                //                print("check 1")
                //                print("\(String(move.first!))==\(String(moves[count - 1].first!))" )
                //                print("++++")
                continue
            }

            // set up a couple vars to make the if statement easier to work with
            // Avoid move sequences like "R L R", which is the same as "R2 L"
            // first check if former move is the same one, and check if the latter is the opposite face
            let ind3 = Int(ScrambleData.faces.firstIndex(of: faceString)!) + 3
            if count > 1, move.first! == moves[count - 2].first!, String(moves[count - 1].first!) == ScrambleData.faces[ind3 % 6] {
//                                print("::::")
//                                print("2 back Match")
//                                print("\(String(move.first!))==\(String(moves[count - 2].first!))")
//                                print("Previous matches Opposite")
//                                print("\(String(moves[count - 1].first!))==\(ScrambleData.faces[ ind3 % 6 ])")
//                                print("Ind3 = \(ind3)")
//                                print("Ind3 % 6= \(ind3 % 6)")
//                                print("::::")
                continue
            }

            moves.append(move)
            count += 1
        }

        // get each 0 remainder index +4  and append it as a string
        var choppedArray: [String] = []
        for (index, _) in moves.enumerated() {
            if index % 5 == 0 {
                let slice = moves[index ... index + 4]
                choppedArray.append(slice.joined(separator: "  "))
            }
        }
        return choppedArray
    }
}