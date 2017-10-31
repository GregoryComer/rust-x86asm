use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 116, 222], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 1727912121, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 116, 182, 185, 216, 253, 102], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 116, 245], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RCX, 1805857036, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 116, 145, 12, 49, 163, 107], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 116, 201], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 116, 28, 91], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 116, 231], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 116, 28, 179], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 13, 116, 221], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 864386161, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 12, 116, 172, 123, 113, 124, 133, 51], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 93, 13, 116, 203], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 2, 116, 12, 114], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 43, 116, 212], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 43, 116, 28, 112], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 37, 116, 241], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectDisplaced(RSI, 680121823, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 33, 116, 150, 223, 213, 137, 40], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 73, 116, 253], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 73, 116, 28, 146], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 101, 75, 116, 238], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1583497612, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 5, 73, 116, 20, 197, 140, 65, 98, 94], OperandSize::Qword)
}

