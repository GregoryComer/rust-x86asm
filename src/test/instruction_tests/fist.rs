use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 77, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 82, 77], OperandSize::Word)
}

fn fist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 16], OperandSize::Dword)
}

fn fist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledDisplaced(RSI, Two, 1869896401, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 20, 117, 209, 90, 116, 111], OperandSize::Qword)
}

fn fist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectDisplaced(BX, 1177, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 151, 153, 4], OperandSize::Word)
}

fn fist_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 20, 138], OperandSize::Dword)
}

fn fist_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1612856414, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 20, 213, 94, 60, 34, 96], OperandSize::Qword)
}

