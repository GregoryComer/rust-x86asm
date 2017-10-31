use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 117, 240], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 1855733484, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 117, 154, 236, 62, 156, 110], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 117, 234], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1556224282, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 117, 4, 141, 26, 25, 194, 92], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 117, 237], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 117, 44, 64], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 117, 253], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RCX, 1682033961, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 117, 177, 41, 205, 65, 100], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 13, 117, 249], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 1221851914, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 14, 117, 154, 10, 251, 211, 72], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 61, 1, 117, 243], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1950001452, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 29, 10, 117, 28, 181, 44, 169, 58, 116], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 46, 117, 217], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 45, 117, 60, 211], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 101, 34, 117, 243], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 1167734916, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 45, 117, 146, 132, 56, 154, 69], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 77, 117, 239], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EAX, 2057401006, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 78, 117, 176, 174, 114, 161, 122], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 69, 67, 117, 240], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 77, 117, 52, 199], OperandSize::Qword)
}

