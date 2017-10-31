use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaskmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 591842459, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 140, 177, 155, 204, 70, 35], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 806497624, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 140, 20, 117, 88, 45, 18, 48], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1348056162, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 140, 132, 114, 98, 180, 89, 80], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 140, 28, 216], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 142, 8], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectDisplaced(RDX, 167195985, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 142, 138, 81, 53, 247, 9], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledDisplaced(EAX, Four, 2023225955, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 142, 20, 133, 99, 250, 151, 120], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledDisplaced(RSI, Two, 1557403090, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 142, 28, 117, 210, 21, 212, 92], OperandSize::Qword)
}

