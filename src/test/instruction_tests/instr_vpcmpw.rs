use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 237, 13, 63, 231, 38], OperandSize::Dword)
}

#[test]
fn vpcmpw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1048661963, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 221, 9, 63, 28, 221, 203, 79, 129, 62, 97], OperandSize::Dword)
}

#[test]
fn vpcmpw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM24)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 147, 149, 13, 63, 248, 79], OperandSize::Qword)
}

#[test]
fn vpcmpw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 13263309, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 133, 1, 63, 172, 83, 205, 97, 202, 0, 121], OperandSize::Qword)
}

#[test]
fn vpcmpw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 237, 45, 63, 224, 110], OperandSize::Dword)
}

#[test]
fn vpcmpw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 726585047, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 213, 46, 63, 20, 125, 215, 206, 78, 43, 104], OperandSize::Dword)
}

#[test]
fn vpcmpw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM14)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 211, 133, 44, 63, 238, 18], OperandSize::Qword)
}

#[test]
fn vpcmpw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 157, 44, 63, 56, 17], OperandSize::Qword)
}

#[test]
fn vpcmpw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 229, 75, 63, 214, 61], OperandSize::Dword)
}

#[test]
fn vpcmpw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 553848857, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 229, 73, 63, 156, 191, 25, 16, 3, 33, 125], OperandSize::Dword)
}

#[test]
fn vpcmpw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM19)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 179, 197, 79, 63, 235, 6], OperandSize::Qword)
}

#[test]
fn vpcmpw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 77, 63, 28, 249, 49], OperandSize::Qword)
}

