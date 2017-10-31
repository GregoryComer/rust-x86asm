use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 22, 6], OperandSize::Dword)
}

#[test]
fn movhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 22, 2], OperandSize::Qword)
}

#[test]
fn movhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(IndirectScaledDisplaced(ESI, Two, 605149239, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 23, 12, 117, 55, 216, 17, 36], OperandSize::Dword)
}

#[test]
fn movhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 23, 44, 121], OperandSize::Qword)
}

