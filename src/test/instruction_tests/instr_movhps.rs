use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 22, 44, 203], OperandSize::Dword)
}

#[test]
fn movhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDX, 988643757, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 22, 170, 173, 129, 237, 58], OperandSize::Qword)
}

#[test]
fn movhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(IndirectDisplaced(ESI, 2120915513, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 23, 134, 57, 154, 106, 126], OperandSize::Dword)
}

#[test]
fn movhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1016501631, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 23, 12, 69, 127, 149, 150, 60], OperandSize::Qword)
}

