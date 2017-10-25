use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fiadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 2], OperandSize::Word)
}

fn fiadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1755594367, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 132, 147, 127, 62, 164, 104], OperandSize::Dword)
}

fn fiadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 4, 66], OperandSize::Qword)
}

fn fiadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 7], OperandSize::Word)
}

fn fiadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 4, 223], OperandSize::Dword)
}

fn fiadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 700926222, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 132, 242, 14, 73, 199, 41], OperandSize::Qword)
}

