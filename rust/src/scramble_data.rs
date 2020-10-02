// Ported from the Swift version I use in Last Layer
use rand::Rng;

pub fn scramble_data_vec() -> Vec<String> {
    // println!("Hello, world!");
    let faces = ["D", "L", "B", "U", "R", "F"];
    let modifiers = ["", "'", "2"];

    let mut count = 0;
    let total = 25;
    let mut moves: Vec<String> = vec![];

    while count < total {
        let mut rng = rand::thread_rng();
        //random number 0 - 5
        let rand_face_num = rng.gen_range(0, 6);
        //random number 0 - 3
        let rand_mod_num = rng.gen_range(0, 3);
        let face_string = faces[rand_face_num];
        let mod_string = modifiers[rand_mod_num];

        let move_string = face_string.to_string() + mod_string;
        // println!("checking string {}", move_string);
        // Avoid having the same move twice in a row.
        //get first character of string '&move_string[0..1]'
        //THis is ugly, but I get the second to last element in the vector the moves[moves.len() - 1] part
        //then get the first character of the element the &[0..1] part
        if count > 0 && &move_string[0..1] == &moves[moves.len() - 1][0..1] {
            // println!("denied string {}", move_string);
            //                print("++++")
            //                print("check 1")
            //                print("\(String(move.first!))==\(String(moves[count - 1].first!))" )
            //                print("++++")
            continue;
        }

        // set up a couple vars to make the if statement easier to work with
        // Avoid move sequences like "R L R", which is the same as "R2 L"
        // first check if former move is the same one, and check if the latter is the opposite face

        //ind3 finds the index of the character being checked, and adds 3 to the index number
        //then eventually it checks if its divisble by 6 to see if its an opposite move,
        //definetly depends on the faces array being in this order ["D", "L", "B", "U", "R", "F"];

        //let ind3 = Int(ScrambleData.faces.firstIndex(of: faceString)!) + 3
        let ind3 = faces.iter().position(|&r| r == face_string).unwrap() + 3;
        if count > 1
            && move_string[0..1] == moves[moves.len() - 2][0..1]
            && &moves[moves.len() - 1][0..1] == faces[ind3 % 6]
        {
            //                                print("::::")
            //                                print("2 back Match")
            //                                print("\(String(move.first!))==\(String(moves[count - 2].first!))")
            //                                print("Previous matches Opposite")
            //                                print("\(String(moves[count - 1].first!))==\(ScrambleData.faces[ ind3 % 6 ])")
            //                                print("Ind3 = \(ind3)")
            //                                print("Ind3 % 6= \(ind3 % 6)")
            //                                print("::::")
            continue;
        }

        moves.push(move_string);
        count += 1;
    }

    // get each 0 remainder index +4  and append it as a string
    // basically just bundling each 5 elements into 1 big string and stuffing it into
    // the chopped_array
    let mut chopped_array: Vec<String> = vec![];
    for (index, _) in moves.iter().enumerate() {
        if index % 5 == 0 {
            let slice = moves.get(index..index + 5);
            // println!("{}", &slice.unwrap().join(" "));
            chopped_array.push(slice.unwrap().join(" "));
        }
    }
    return chopped_array;
    // for elem in chopped_array.iter() {
    //     println!("{}", elem)
    // }
}
