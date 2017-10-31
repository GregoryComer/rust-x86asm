use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaskmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 140, 47], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1091016434, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 140, 156, 95, 242, 150, 7, 65], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 303976938, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 140, 160, 234, 81, 30, 18], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 140, 44, 178], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectDisplaced(EDX, 965731701, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 142, 162, 117, 229, 143, 57], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectDisplaced(RSI, 838616211, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 142, 190, 147, 68, 252, 49], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1491222810, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 142, 172, 86, 26, 65, 226, 88], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 142, 34], OperandSize::Qword)
}

