use triton_vm::{isa::triton_program, prelude::bfe_vec, proof::Claim, vm::{NonDeterminism, PublicInput}};
use triton_vm::prelude::BFieldElement;

fn prove_program(){
    // y = x + y;
    
}

fn output_res(claim : &Claim){
    claim.clone().output.into_iter().for_each(|output|{
        println!("The claim output is {:?}", output.value())
    });
}

fn main() {
    let solo_safe_program = triton_program!(
        read_io 3
        dup 0
        push 1 eq
        skiz
            call increase_balance 
        push 2 eq
        skiz
            call decrease_balance
        halt 

        


        increase_balance : 
            pop 1
            add
            write_io 1
            return 
        
        decrease_balance :
            swap 1
            invert
            add
            write_io 1
            return 

    );

    // first param : balance 
    // second param : amount 
    // third param : operation : 1 for add and 2 for sub 
    let public_input = PublicInput::new(bfe_vec![7, 90, 2]);
    let non_determinism = NonDeterminism::default();
    let (stark, claim, proof) = triton_vm::prove_program(solo_safe_program, public_input, non_determinism).unwrap();
    // output : the new balance 
    output_res(&claim);
}

fn another_test_func(){
    
    let simple_add_program = triton_program!(
        read_io 5
        write_io 1
        halt
    );

    let max_bfield_elt = BFieldElement::MAX;
    println!("The max number is {max_bfield_elt}");

    let public_input = PublicInput::new(bfe_vec![100, 21, 7, 9, 56, 4, 6, 6,56, 90]);
    let non_determinism = NonDeterminism::default();

    let (stark, claim, proof) = triton_vm::prove_program(simple_add_program, public_input, non_determinism).unwrap();
    claim.output.into_iter().for_each(|output|{
        println!("The claim output is {:?}", output.value())
    });
    // println!("The claim output is {:?}", claim.output[0].value());
}
