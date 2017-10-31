use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fidivr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 21753, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 185, 249, 84], OperandSize::Word)
}

#[test]
fn fidivr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledDisplaced(EBX, Two, 31558132, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 60, 93, 244, 137, 225, 1], OperandSize::Dword)
}

#[test]
fn fidivr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1714892793, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 60, 85, 249, 47, 55, 102], OperandSize::Qword)
}

#[test]
fn fidivr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectDisplaced(SI, 10645, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 188, 149, 41], OperandSize::Word)
}

#[test]
fn fidivr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1105689528, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 188, 254, 184, 123, 231, 65], OperandSize::Dword)
}

#[test]
fn fidivr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 59], OperandSize::Qword)
}

