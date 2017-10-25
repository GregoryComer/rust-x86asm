use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fstenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(Memory(23016, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 54, 232, 89], OperandSize::Word)
}

fn fstenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(IndirectDisplaced(EAX, 154669177, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 176, 121, 16, 56, 9], OperandSize::Dword)
}

fn fstenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1600794886, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 52, 197, 6, 49, 106, 95], OperandSize::Qword)
}

