use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaskmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 140, 60, 64], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 887004449, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 140, 52, 157, 33, 157, 222, 52], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDI, 552300406, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 140, 167, 118, 111, 235, 32], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1472666357, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 140, 132, 89, 245, 26, 199, 87], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1490447563, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 142, 172, 198, 203, 108, 214, 88], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectDisplaced(RSI, 134800894, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 142, 166, 254, 229, 8, 8], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 142, 1], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 7235757, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 142, 36, 253, 173, 104, 110, 0], OperandSize::Qword)
}

