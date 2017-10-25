use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xrstors_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 27], OperandSize::Word)
}

fn xrstors_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 44466090, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 156, 223, 170, 127, 166, 2], OperandSize::Dword)
}

fn xrstors_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 739265639, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 156, 147, 103, 76, 16, 44], OperandSize::Qword)
}

