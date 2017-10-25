use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fidiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 55], OperandSize::Word)
}

fn fidiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledDisplaced(EDX, Two, 867651944, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 52, 85, 104, 81, 183, 51], OperandSize::Dword)
}

fn fidiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 51], OperandSize::Qword)
}

fn fidiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 48], OperandSize::Word)
}

fn fidiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 52, 147], OperandSize::Dword)
}

fn fidiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 230454986, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 180, 74, 202, 118, 188, 13], OperandSize::Qword)
}

