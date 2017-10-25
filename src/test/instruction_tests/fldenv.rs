use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fldenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectDisplaced(SI, 27, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 100, 27], OperandSize::Word)
}

fn fldenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1973735089, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 36, 77, 177, 206, 164, 117], OperandSize::Dword)
}

fn fldenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(Indirect(RCX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 33], OperandSize::Qword)
}

